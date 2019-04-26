use crate::types::{Map, Node};
use std::collections::HashSet;

#[inline]
fn distance(a: usize, b: usize, n: usize) -> usize
{
	let x = (((a / n) as isize) - ((b / n) as isize)).abs() as usize;
	let y = (((a % n) as isize) - ((b % n) as isize)).abs() as usize;
	x + y
}

// Returns the sum of distance between the start position and the end position of each tiles
// The empty tile is ignored if we want the heuristic to be admissible
#[inline]
pub fn manhattan(mut node: Node, end: &Map, size: usize) -> Node
{
	for index in 0..(size * size)
	{
		let elem = node.map[index];
		if elem == 0 { continue }
		node.cost[index] = distance(index, end[elem], size);
	}
	node.h = node.cost.iter().sum();
	node
}

#[inline]
pub fn partial_manhattan(mut node: Node, end: &Map, size: usize) -> Node
{
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let elem = node.map[index];
	let cost = distance(index, end[elem], size);
	node.h += cost - node.cost[elem];
	node.cost[elem] = cost;
	node
}


// Returns the number of misplaced tiles in the puzzle expect the empty tile
#[inline]
pub fn misplaced_tiles(mut node: Node, end: &Map, size: usize) -> Node
{
	for index in 0..(size * size)
	{
		let elem = node.map[index];
		if elem == 0 { continue }
		node.cost[index] = if index != end[elem] { 1 } else { 0 };
	}
	node.h = node.cost.iter().sum();
	node
}

#[inline]
pub fn partial_misplaced(mut node: Node, end: &Map, size: usize) -> Node
{
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let elem = node.map[index];
	let cost = (index != end[elem]) as usize;
	node.h += cost - node.cost[elem];
	node.cost[elem] = cost;
	node
}


// Returns the number of tiles misplaced from their axes
#[inline]
pub fn out_of_axes(mut node: Node, end: &Map, size: usize) -> Node
{
	for index in 0..(size * size)
	{
		let elem = node.map[index];
		if elem == 0 { continue }
		node.cost[index] += (index / size == end[elem] / size) as usize;
		node.cost[index] += (index & size == end[elem] % size) as usize;
	}
	node.h = node.cost.iter().sum();
	node
}

pub fn partial_out_of_axes(mut node: Node, end: &Map, size: usize) -> Node
{
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let elem = node.map[index];
	let cost = (index / size == end[elem] / size) as usize
			+ (index & size == end[elem] % size) as usize;
	node.h += cost - node.cost[elem];
	node.cost[elem] = cost;
	node
}


// Linear conflict explained here:
// https://algorithmsinsight.wordpress.com/graph-theory-2/a-star-in-general/implementing-a-star-to-solve-n-puzzle/
#[inline]
fn conflict(start: usize, cur: usize, end: usize, size: usize, inc: i8) -> bool
{
	match (inc.abs() == 1, inc > 0)
	{
		(true, true) => (start / size == cur / size) && (cur <= end),
		(false, true) => (start % size == cur % size) && (cur <= end),
		(true, false) => (start / size == cur / size) && (cur >= end),
		(false, false) => (start % size == cur % size) && (cur >= end)
	}
}

#[inline]
fn conflict_pair(a: usize, b: usize) -> (usize, usize)
{
	if a < b { (a, b) } else { (b, a) }
}

#[inline]
fn count_conflicts(start_pos: usize, inc: usize, size: usize, start: &Map, end: &Map) -> Option<Vec<(usize, usize)>>
{
	let mut conflicts: Vec<(usize, usize)> = vec![];
	let mut pos = start_pos;
	let end_pos = end[start[start_pos]];
	let inc = (inc as i8) * if start_pos < end_pos { 1 } else { -1 };

	while pos != end_pos
	{
		pos = ((pos as i8) + inc) as usize;
		if start[pos] == 0 { continue };
		let pos_ = end[start[pos]];
		if conflict(start_pos, pos_, end_pos, size, inc)
		{
			let pair = conflict_pair(start[start_pos], start[pos]);
			conflicts.push(pair);
		}
	}
	if conflicts.len() == 0 { None } else { Some(conflicts) }
}

#[inline]
pub fn linear_conflict(mut node: Node, end: &Map, size: usize) -> Node
{
	node = manhattan(node, end, size);
	// Get the sum of conflicts for each tile
	let raw_list: Vec<Option<Vec<(usize, usize)>>> = node.map.iter().enumerate().map(| (i, x) |
	{
		if *x == 0 { return None }
		let same_row = i / size == end[*x] / size;
		let same_column = i % size == end[*x] % size;
		let pair_list = match (same_row, same_column)
		{
			(false, false) | (true, true) => None,
			(false, true) => count_conflicts(i, size, size, &node.map, end),
			(true, false) => count_conflicts(i, 1, size, &node.map, end),
		};
		pair_list
	}).collect();

	let mut conflicts: HashSet<(usize, usize)> = HashSet::new();
	for pair_list in raw_list
	{
		if let Some(pair) = pair_list
		{
			for elem in pair
			{
				if conflicts.contains(&elem) == false
				{
					node.cost[elem.0] += 1;
					node.cost[elem.1] += 1;
					conflicts.insert(elem);
				}
			}
		}
	}
	node.h = node.cost.iter().sum();
	node
}

// pub fn partial_conflict(mut node: Node, end: &Map, size: usize) -> Node
// {
// 	let old_index = node.pos.as_index(size);
// 	let index = node.pos.moved_element(&node.movement).as_index(size);
// 	let elem = node.map[index];
// 	let same_row = index / size == end[elem] / size;
// 	let same_column = index % size == end[elem] % size;
// 	let pair_list = match (same_row, same_column)
// 	{
// 		(false, false) | (true, true) => None,
// 		(false, true) => count_conflicts(index, size, size, &node.map, end),
// 		(true, false) => count_conflicts(index, 1, size, &node.map, end),
// 	};
// 	node
// }

// #[cfg(test)]
// mod tests
// {
//     use crate::types::Map;

// 	#[test]
// 	fn distance()
// 	{
// 		assert_eq!(super::distance(1, 8, 3), 3);
// 	}

//     #[test]
//     fn manhattan_0()
// 	{
//         let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
// 		// index and position of end were swapped => [tile 0 at index 8, tile 1 at index 0, ...]
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::manhattan(&start, &end, 3), 0);
//     }

//     #[test]
//     fn manhattan_1()
// 	{
//         let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::manhattan(&start, &end, 3), 1);
//     }

//     #[test]
//     fn manhattan_10()
// 	{
//         let start: Map = vec![2, 1, 8, 4, 6, 5, 7, 3, 0];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::manhattan(&start, &end, 3), 10);
//     }

// 	#[test]
// 	fn misplaced_tiles_0()
// 	{
// 		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::misplaced_tiles(&start, &end, 3), 0);
// 	}

// 	#[test]
// 	fn misplaced_tiles_1()
// 	{
// 		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::misplaced_tiles(&start, &end, 3), 1);
// 	}

// 	#[test]
// 	fn misplaced_tiles_8()
// 	{
// 		let start: Map = vec![2, 3, 4, 5, 6, 7, 8, 0, 1];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::misplaced_tiles(&start, &end, 3), 8);
// 	}

// 	#[test]
// 	fn out_of_axes_0()
// 	{
// 		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::out_of_axes(&start, &end, 3), 0);
// 	}

// 	#[test]
// 	fn out_of_axes_1()
// 	{
// 		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::out_of_axes(&start, &end, 3), 1);
// 	}

// 	#[test]
// 	fn out_of_axes_10()
// 	{
// 		let start: Map = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::out_of_axes(&start, &end, 3), 10);
// 	}

// 	#[test]
// 	fn linear_conflict_3x3_1()
// 	{
// 		let start: Map = vec![3, 0, 1, 2, 4, 6, 8, 5, 7];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::linear_conflict(&start, &end, 3), 15);
// 	}

// 	#[test]
// 	fn linear_conflict_3x3_2()
// 	{
// 		let start: Map = vec![3, 8, 1, 6, 4, 5, 0, 2, 7];
//         let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

//         assert_eq!(super::linear_conflict(&start, &end, 3), 22);
// 	}

// 	#[test]
// 	fn linear_conflict_4x4()
// 	{
// 		let start: Map = vec![4, 1, 15, 2, 6, 8, 5, 7, 12, 9, 3, 10, 14, 13, 11, 0];
//         let end: Map = vec![15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

//         assert_eq!(super::linear_conflict(&start, &end, 4), 46);
// 	}

// }
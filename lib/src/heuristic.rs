use crate::types::{Map, Node};
use std::collections::HashSet;

// fn display_map(map: &Map, size: usize)
// {
// 	for i in 0..(size * size)
// 	{
// 		if i != 0 && i % size == 0 { println!("\n") }
// 		print!(" {}\t", map[i]);
// 	}
// 	println!("");
// }

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
		node.cost[elem] = distance(index, end[elem], size);
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
	node.h = (node.h as i32 + (cost as i32 - node.cost[elem] as i32)) as usize;
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
		node.cost[elem] = if index != end[elem] { 1 } else { 0 };
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
	node.h = (node.h as i32 + (cost as i32 - node.cost[elem] as i32)) as usize;
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
		node.cost[elem] = (index / size != end[elem] / size) as usize
						+ (index & size != end[elem] % size) as usize;
	}
	node.h = node.cost.iter().sum();
	node
}

pub fn partial_out_of_axes(mut node: Node, end: &Map, size: usize) -> Node
{
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let elem = node.map[index];
	let cost = (index / size != end[elem] / size) as usize
			+ (index & size != end[elem] % size) as usize;
	node.h = (node.h as i32 + (cost as i32 - node.cost[elem] as i32)) as usize;
	node.cost[elem] = cost;
	node
}

struct Conflict
{
	id: usize,
	start: usize,
	end: usize,
	direction: i32
}

impl Conflict
{
	fn new(id: usize, start: usize, end: usize) -> Self
	{
		Self
		{
			id: id,
			start: start,
			end: end,
			direction: 0
		}
	}

	fn conflict_possibility(&mut self, size: usize) -> bool
	{
		let same_row = self.start / size == self.end / size;
		let same_col = self.start % size == self.end % size;
		match (same_row, same_col)
		{
			(true, ..) =>
			{
				self.direction = if self.start < self.end { 1 } else { -1 };
				true
			}
			(.., true) =>
			{
				self.direction = size as i32 * if self.start < self.end { 1 } else { -1 };
				true
			}
			(false, false) => false
		}
	}

	fn conflict_with(&self, other: &Self, size: usize) -> bool
	{
		match self.direction
		{
			1 => (self.start / size == other.end / size) && (other.end <= self.end),
			n if n > 0 => (self.start % size == other.end % size) && (other.end <= self.end),
			-1 => (self.start / size == other.end / size) && (other.end >= self.end),
			n if n < 0 => (self.start % size == other.end % size) && (other.end >= self.end),
			_ => false
		}
	}

	fn conflict_pair(&self, other: &Self) -> (usize, usize)
	{
		if self.id < other.id { (self.id, other.id) } else { (other.id, self.id) }
	}

	fn get_conflicts(&self, start: &Map, end: &Map, size: usize) -> Vec<(usize, usize)>
	{
		let mut list: Vec<(usize, usize)> = vec![];
		let mut cursor = self.start;
		while cursor != self.end
		{
			cursor = (cursor as i32 + self.direction) as usize;
			if start[cursor] == 0 || start[cursor] == self.id { continue }
			let other = Conflict::new(start[cursor], cursor, end[start[cursor]]);
			if self.conflict_with(&other, size) == false { continue }
			list.push(self.conflict_pair(&other));
		}
		list
	}
}

// Linear conflict explained here:
// https://algorithmsinsight.wordpress.com/graph-theory-2/a-star-in-general/implementing-a-star-to-solve-n-puzzle/
pub fn linear_conflict(mut node: Node, end: &Map, size: usize) -> Node
{
	node = manhattan(node, end, size);
	let mut list: Vec<(usize, usize)> = vec![];
	for index in 0..(size * size)
	{
		let id = node.map[index];
		let mut elem = Conflict::new(id, index, end[id]);
		if elem.id == 0 || elem.conflict_possibility(size) == false { continue }
		list.extend(elem.get_conflicts(&node.map, end, size));
	}
	// Remove duplicate pairs
	let set: HashSet<_> = list.drain(..).collect();
	list.extend(set.into_iter());

	// Add the additional costs
	for pair in &list
	{
		node.cost[pair.0] += 1;
		node.cost[pair.1] += 1;
	}
	node.h += 2 * list.len();
	node
}

pub fn partial_conflict(mut node: Node, end: &Map, size: usize) -> Node
{
	node = partial_manhattan(node, end, size);
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let id = node.map[index];

	// Get conflict list of moved elem before the move
	let prev_index = node.pos.as_index(size);
	let mut prev_elem = Conflict::new(id, prev_index, end[id]);
	let cp = prev_elem.conflict_possibility(size);
	let prev_list = if cp { prev_elem.get_conflicts(&node.map, end, size) } else { vec![] };

	// Get current conflict list of moved elem
	let mut elem = Conflict::new(id, index, end[id]);
	let cp = elem.conflict_possibility(size);
	let list = if cp { elem.get_conflicts(&node.map, end, size) } else { vec![] };

	// Remove cost of old conflicts
	for pair in &prev_list
	{
		node.cost[pair.0] -= 1;
		node.cost[pair.1] -= 1;
	}

	// Add the new conflicts costs
	for pair in &list
	{
		node.cost[pair.0] += 1;
		node.cost[pair.1] += 1;
	}

	node.h = (node.h as i32 + 2 * (list.len() as i32 - prev_list.len() as i32)) as usize;
	node
}


// #[inline]
// fn conflict(start: usize, cur: usize, end: usize, size: usize, inc: i8) -> bool
// {
// 	match (inc.abs() == 1, inc > 0)
// 	{
// 		(true, true) => (start / size == cur / size) && (cur <= end),
// 		(false, true) => (start % size == cur % size) && (cur <= end),
// 		(true, false) => (start / size == cur / size) && (cur >= end),
// 		(false, false) => (start % size == cur % size) && (cur >= end)
// 	}
// }

// #[inline]
// fn conflict_pair(a: usize, b: usize) -> (usize, usize)
// {
// 	if a < b { (a, b) } else { (b, a) }
// }

// #[inline]
// fn count_conflicts(start_pos: usize, inc: usize, size: usize, start: &Map, end: &Map) -> Option<Vec<(usize, usize)>>
// {
// 	let mut conflicts: Vec<(usize, usize)> = vec![];
// 	let mut pos = start_pos;
// 	let end_pos = end[start[start_pos]];
// 	let inc = (inc as i8) * if start_pos < end_pos { 1 } else { -1 };

// 	while pos != end_pos
// 	{
// 		pos = ((pos as i8) + inc) as usize;
// 		if start[pos] == 0 { continue };
// 		let pos_ = end[start[pos]];
// 		if conflict(start_pos, pos_, end_pos, size, inc)
// 		{
// 			let pair = conflict_pair(start[start_pos], start[pos]);
// 			conflicts.push(pair);
// 		}
// 	}
// 	if conflicts.len() == 0 { None } else { Some(conflicts) }
// }

// #[inline]
// pub fn linear_conflict(mut node: Node, end: &Map, size: usize) -> Node
// {
// 	node = manhattan(node, end, size);
// 	// Get the sum of conflicts for each tile
// 	let raw_list: Vec<Option<Vec<(usize, usize)>>> = node.map.iter().enumerate().map(| (i, x) |
// 	{
// 		if *x == 0 { return None }
// 		let same_row = i / size == end[*x] / size;
// 		let same_column = i % size == end[*x] % size;
// 		let pair_list = match (same_row, same_column)
// 		{
// 			(false, false) | (true, true) => None,
// 			(false, true) => count_conflicts(i, size, size, &node.map, end),
// 			(true, false) => count_conflicts(i, 1, size, &node.map, end),
// 		};
// 		pair_list
// 	}).collect();

// 	let mut conflicts: HashSet<(usize, usize)> = HashSet::new();
// 	for pair_list in raw_list
// 	{
// 		if let Some(pair) = pair_list
// 		{
// 			for elem in pair
// 			{
// 				if conflicts.contains(&elem) == false
// 				{
// 					node.cost[elem.0] += 1;
// 					node.cost[elem.1] += 1;
// 					conflicts.insert(elem);
// 				}
// 			}
// 		}
// 	}
// 	node.h = node.cost.iter().sum();
// 	node
// }

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
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
	let prev_cost = distance(node.pos.as_index(size), end[elem], size);
	let cost = distance(index, end[elem], size);
	node.h = (node.h as i32 + (cost as i32 - prev_cost as i32)) as usize;
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
						+ (index % size != end[elem] % size) as usize;
	}
	node.h = node.cost.iter().sum();
	node
}

#[inline]
pub fn partial_out_of_axes(mut node: Node, end: &Map, size: usize) -> Node
{
	let index = node.pos.moved_element(&node.movement).as_index(size);
	let elem = node.map[index];
	let cost = (index / size != end[elem] / size) as usize
			+ (index % size != end[elem] % size) as usize;
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

	fn get_direction(&mut self, size: usize) -> bool
	{
		let same_row = self.start / size == self.end / size;
		let same_col = self.start % size == self.end % size;
		match (same_row, same_col)
		{
			(true, ..) =>
			{
				self.direction = match self.start
				{
					start if start < self.end => 1,
					start if start > self.end => -1,
					_ => 0
				};
				true
			}
			(.., true) =>
			{
				self.direction = match self.start
				{
					start if start < self.end => size as i32,
					start if start > self.end => -(size as i32),
					_ => 0
				};
				true
			}
			(false, false) => false
		}
	}

	// Check every possibility of conflict
	// (Can be simplified)
	fn conflict_with(&self, other: &Self, way: i32) -> bool
	{
		match (self.direction, other.direction, way)
		{
			// Can't be in conflict if they both don't need to move
			(0, 0, ..) => false,

			// If they both go in the same direction, check if the one before doesn't stop after the other
			n if n.0 == n.1 && n.0.abs() == n.2.abs() =>
			{
				match self.direction > 0
				{
					true => match self.start < other.start
					{
						true => self.end >= other.end,
						false => self.end <= other.end
					}
					false => match self.start > other.start
					{
						true => self.end <= other.end,
						false => self.end >= other.end
					}
				}
			}

			// If they go in opposite ways on the same axis
			// then they are in conflict if they are in the way of each other
			n if n.0 == -n.1 && n.0.abs() == n.2.abs() =>
			{
				match self.direction > 0
				{
					true => self.start < other.start && self.end >= other.end,
					false => self.start > other.start && self.end <= other.end,
				}
			}

			// If self doesn't move && moving on the same axis then check if self is in the way of other
			n if n.0 == 0 && n.1.abs() == n.2.abs() =>
			{
				match other.direction > 0
				{
					true => other.start < self.start && other.end >= self.end,
					false => other.start > self.start && other.end <= self.end
				}
			}

			// If other doesn't move && moving on the same axis then check if other is in the way of self
			n if n.1 == 0 && n.0.abs() == n.2.abs() =>
			{
				match self.direction > 0
				{
					true => self.start < other.start && self.end >= other.end,
					false => self.start > other.start && self.end <= other.end,
				}
			}

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
			let mut other = Conflict::new(start[cursor], cursor, end[start[cursor]]);
			if other.get_direction(size) == false { continue }
			if self.conflict_with(&other, self.direction) == false { continue }
			list.push(self.conflict_pair(&other));
		}
		list
	}

	fn get_row_conflicts(&self, start: &Map, end: &Map, size: usize) -> Vec<(usize, usize)>
	{
		let mut list: Vec<(usize, usize)> = vec![];
		let mut cursor = (self.start / size) * size;
		let last_index = cursor + size - 1;
		while cursor <= last_index
		{
			if start[cursor] != 0 && start[cursor] != self.id
			{
				let mut other = Conflict::new(start[cursor], cursor, end[start[cursor]]);
				if other.get_direction(size)
				{
					if self.conflict_with(&other, 1) { list.push(self.conflict_pair(&other)) }
				}
			}
			cursor += 1;
		}
		list
	}

	fn get_column_conflicts(&self, start: &Map, end: &Map, size: usize) -> Vec<(usize, usize)>
	{
		let mut list: Vec<(usize, usize)> = vec![];
		let mut cursor = self.start % size;
		let last_index = size * size - size + cursor;
		while cursor <= last_index
		{
			if start[cursor] != 0 && start[cursor] != self.id
			{
				let mut other = Conflict::new(start[cursor], cursor, end[start[cursor]]);
				if other.get_direction(size)
				{
					if self.conflict_with(&other, size as i32) { list.push(self.conflict_pair(&other)) }
				}
			}
			cursor += size;
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
		if elem.id == 0 || elem.get_direction(size) == false { continue }
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
	let mut prev_list: Vec<(usize, usize)> = vec![];
	if prev_elem.get_direction(size)
	{
		prev_list = prev_elem.get_row_conflicts(&node.map, end, size);
		prev_list.extend(prev_elem.get_column_conflicts(&node.map, end, size));
	}

	// Get current conflict list of moved elem
	let mut elem = Conflict::new(id, index, end[id]);
	let mut list: Vec<(usize, usize)> = vec![];
	if elem.get_direction(size)
	{
		list = elem.get_row_conflicts(&node.map, end, size);
		list.extend(elem.get_column_conflicts(&node.map, end, size));
	}

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

#[cfg(test)]
mod tests
{
    use crate::types::{Map, Node};

	#[test]
	fn distance()
	{
		assert_eq!(super::distance(1, 8, 3), 3);
	}

    #[test]
    fn manhattan()
	{
		let start: Node = Node::new(vec![2, 1, 8, 4, 6, 5, 7, 3, 0]);
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(start, &end, 3).h, 10);
    }

	#[test]
	fn misplaced_tiles()
	{
		let start: Node = Node::new(vec![2, 3, 4, 5, 6, 7, 8, 0, 1]);
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(start, &end, 3).h, 8);
	}

	#[test]
	fn out_of_axes()
	{
		let start: Node = Node::new(vec![
			0, 1, 2,
			3, 4, 5,
			6, 7, 8]);
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(start, &end, 3).h, 10);
	}

	#[test]
	fn linear_conflict_1()
	{
		let start: Node = Node::new(vec![3, 0, 1, 2, 4, 6, 8, 5, 7]);
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(start, &end, 3).h, 15);
	}

	#[test]
	fn linear_conflict_2()
	{
		let start: Node = Node::new(vec![3, 8, 1, 6, 4, 5, 0, 2, 7]);
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(start, &end, 3).h, 22);
	}

	#[test]
	fn linear_conflict_3()
	{
		let start: Node = Node::new(vec![4, 1, 15, 2, 6, 8, 5, 7, 12, 9, 3, 10, 14, 13, 11, 0]);
        let end: Map = vec![15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        assert_eq!(super::linear_conflict(start, &end, 4).h, 46);
	}

}
use crate::types::Map;
use std::collections::HashMap;

fn distance(a: usize, b: usize, n: usize) -> usize
{
	let x = (((a / n) as isize) - ((b / n) as isize)).abs() as usize;
	let y = (((a % n) as isize) - ((b % n) as isize)).abs() as usize;
	x + y
}

// Returns the sum of distance between the start position and the end position of each tiles
// The empty tile is ignored if we want the heuristic to be admissible
pub fn manhattan(start: &Map, end: &Map, size: usize) -> usize
{
	start
		.iter()
		.enumerate()
		.fold(0, | acc, (i, x) | acc + if *x != 0 { distance(i, end[*x], size) } else { 0 })
}

// Returns the number of misplaced tiles in the puzzle expect the empty tile
pub fn misplaced_tiles(start: &Map, end: &Map, _size: usize) -> usize
{
	start
		.iter()
		.enumerate()
		.fold(0, | acc, (i, x) | acc + if *x != 0 && i != end[*x] { 1 } else { 0 })
}

// Returns the number of tiles misplaced from their axes
pub fn out_of_axes(start: &Map, end: &Map, size: usize) -> usize
{
	start.iter().enumerate().fold(0, | acc, (i, x) |
	{
		if *x == 0 { return acc }
		let same_row = i / size == end[*x] / size;
		let same_column = i % size == end[*x] % size;
		acc + match (same_row, same_column)
		{
			(false, false) => 2,
			(false, true) | (true, false) => 1,
			(true, true) => 0
		}
	})
}

// Linear conflict explained here:
// https://algorithmsinsight.wordpress.com/graph-theory-2/a-star-in-general/implementing-a-star-to-solve-n-puzzle/
pub fn linear_conflict(start: &Map, end: &Map, size: usize) -> usize
{
	let mut records: HashMap<String, bool> = HashMap::new();

	let pair_to_key = | a: usize, b: usize | if a < b { format!("{},{}", a, b) } else { format!("{},{}", b, a) };

	// count the number of conflicts for a single tile
	let mut count_conflicts = | pos: usize, axis: usize | -> usize
	{
		let mut count = 0;
		let mut start_pos = pos as isize;
		let end_pos = end[start[pos]] as isize;
		let inc = if start_pos < end_pos { axis as isize } else { -(axis as isize) };

		let same_axis = | x: usize | if axis == 1 { pos / size == x / size } else { pos % size == x % size };
		let in_the_way = | x: usize | if inc > 0 { x <= end_pos as usize } else { x >= end_pos as usize };
		while start_pos != end_pos
		{
			start_pos += inc;
			if start[start_pos as usize] == 0 { continue };
			let end_pos = end[start[start_pos as usize]];
			count += match end_pos
			{
				x if same_axis(x) && in_the_way(x) =>
				{
					let key = pair_to_key(start[pos], start[start_pos as usize]);
					let result = if records.contains_key(&key) { 0 } else { 1 };
					records.insert(key, true);
					result
				},
				_ => 0
			}
		}
		count
	};

	// Get the sum of conflicts for each tile
	let conflicts = start.iter().enumerate().fold(0, | acc, (i, x) |
	{
		if *x == 0 { return acc }
		let same_row = i / size == end[*x] / size;
		let same_column = i % size == end[*x] % size;
		acc + match (same_row, same_column)
		{
			(false, false) | (true, true) => 0,
			(false, true) => count_conflicts(i, size),
			(true, false) => count_conflicts(i, 1),
		}
	});
	manhattan(start, end, size) + 2 * conflicts
}

#[cfg(test)]
mod tests
{
    use crate::types::Map;

	#[test]
	fn distance()
	{
		assert_eq!(super::distance(1, 8, 3), 3);
	}

    #[test]
    fn manhattan_0()
	{
        let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
		// index and position of end were swapped => [tile 0 at index 8, tile 1 at index 0, ...]
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&start, &end, 3), 0);
    }

    #[test]
    fn manhattan_1()
	{
        let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&start, &end, 3), 1);
    }

    #[test]
    fn manhattan_10()
	{
        let start: Map = vec![2, 1, 8, 4, 6, 5, 7, 3, 0];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&start, &end, 3), 10);
    }

	#[test]
	fn misplaced_tiles_0()
	{
		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&start, &end, 3), 0);
	}

	#[test]
	fn misplaced_tiles_1()
	{
		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&start, &end, 3), 1);
	}

	#[test]
	fn misplaced_tiles_8()
	{
		let start: Map = vec![2, 3, 4, 5, 6, 7, 8, 0, 1];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&start, &end, 3), 8);
	}

	#[test]
	fn out_of_axes_0()
	{
		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&start, &end, 3), 0);
	}

	#[test]
	fn out_of_axes_1()
	{
		let start: Map = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&start, &end, 3), 1);
	}

	#[test]
	fn out_of_axes_10()
	{
		let start: Map = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&start, &end, 3), 10);
	}

	#[test]
	fn linear_conflict_3x3_1()
	{
		let start: Map = vec![3, 0, 1, 2, 4, 6, 8, 5, 7];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(&start, &end, 3), 15);
	}

	#[test]
	fn linear_conflict_3x3_2()
	{
		let start: Map = vec![3, 8, 1, 6, 4, 5, 0, 2, 7];
        let end: Map = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(&start, &end, 3), 22);
	}

	#[test]
	fn linear_conflict_4x4()
	{
		let start: Map = vec![4, 1, 15, 2, 6, 8, 5, 7, 12, 9, 3, 10, 14, 13, 11, 0];
        let end: Map = vec![15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        assert_eq!(super::linear_conflict(&start, &end, 4), 46);
	}
}
use crate::types::Puzzle;
use std::collections::HashMap;

fn distance(a: usize, b: usize, n: usize) -> usize
{
	let x = (((a / n) as isize) - ((b / n) as isize)).abs() as usize;
	let y = (((a % n) as isize) - ((b % n) as isize)).abs() as usize;
	x + y
}

// Returns the sum of distance between the initial position and the goal position of each tiles
// The empty tile is ignored if we want the heuristic to be admissible
pub fn manhattan(initial: &Puzzle, goal: &Puzzle, size: usize) -> usize
{
	initial
		.iter()
		.enumerate()
		.fold(0, | acc, (i, x) | acc + if *x != 0 { distance(i, goal[*x], size) } else { 0 })
}

// Returns the number of misplaced tiles in the puzzle expect the empty tile
pub fn misplaced_tiles(initial: &Puzzle, goal: &Puzzle, _size: usize) -> usize
{
	initial
		.iter()
		.enumerate()
		.fold(0, | acc, (i, x) | acc + if *x != 0 && i != goal[*x] { 1 } else { 0 })
}

// Returns the number of tiles misplaced from their axes
pub fn out_of_axes(initial: &Puzzle, goal: &Puzzle, size: usize) -> usize
{
	initial.iter().enumerate().fold(0, | acc, (i, x) |
	{
		if *x == 0 { return acc }
		let same_row = i / size == goal[*x] / size;
		let same_column = i % size == goal[*x] % size;
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
pub fn linear_conflict(initial: &Puzzle, goal: &Puzzle, size: usize) -> usize
{
	let mut records: HashMap<String, bool> = HashMap::new();

	let pair_to_key = | a: usize, b: usize | if a < b { format!("{},{}", a, b) } else { format!("{},{}", b, a) };

	// count the number of conflicts for a single tile
	let mut count_conflicts = | pos: usize, axis: usize | -> usize
	{
		let mut count = 0;
		let mut start = pos as isize;
		let end = goal[initial[pos]] as isize;
		let inc = if start < end { axis as isize } else { -(axis as isize) };

		let same_axis = | x: usize | if axis == 1 { pos / size == x / size } else { pos % size == x % size };
		let in_the_way = | x: usize | if inc > 0 { x <= end as usize } else { x >= end as usize };
		while start != end
		{
			start += inc;
			if initial[start as usize] == 0 { continue };
			let goal_pos = goal[initial[start as usize]];
			count += match goal_pos
			{
				x if same_axis(x) && in_the_way(x) =>
				{
					let key = pair_to_key(initial[pos], initial[start as usize]);
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
	let conflicts = initial.iter().enumerate().fold(0, | acc, (i, x) |
	{
		if *x == 0 { return acc }
		let same_row = i / size == goal[*x] / size;
		let same_column = i % size == goal[*x] % size;
		acc + match (same_row, same_column)
		{
			(false, false) | (true, true) => 0,
			(false, true) => count_conflicts(i, size),
			(true, false) => count_conflicts(i, 1),
		}
	});
	manhattan(initial, goal, size) + 2 * conflicts
}

#[cfg(test)]
mod tests
{
    use crate::types::Puzzle;

	#[test]
	fn distance()
	{
		assert_eq!(super::distance(1, 8, 3), 3);
	}

    #[test]
    fn manhattan_0()
	{
        let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
		// index and position of goal were swapped => [tile 0 at index 8, tile 1 at index 0, ...]
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&initial, &goal, 3), 0);
    }

    #[test]
    fn manhattan_1()
	{
        let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&initial, &goal, 3), 1);
    }

    #[test]
    fn manhattan_10()
	{
        let initial: Puzzle = vec![2, 1, 8, 4, 6, 5, 7, 3, 0];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::manhattan(&initial, &goal, 3), 10);
    }

	#[test]
	fn misplaced_tiles_0()
	{
		let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&initial, &goal, 3), 0);
	}

	#[test]
	fn misplaced_tiles_1()
	{
		let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&initial, &goal, 3), 1);
	}

	#[test]
	fn misplaced_tiles_8()
	{
		let initial: Puzzle = vec![2, 3, 4, 5, 6, 7, 8, 0, 1];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::misplaced_tiles(&initial, &goal, 3), 8);
	}

	#[test]
	fn out_of_axes_0()
	{
		let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&initial, &goal, 3), 0);
	}

	#[test]
	fn out_of_axes_1()
	{
		let initial: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 0, 8];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&initial, &goal, 3), 1);
	}

	#[test]
	fn out_of_axes_10()
	{
		let initial: Puzzle = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::out_of_axes(&initial, &goal, 3), 10);
	}

	#[test]
	fn linear_conflict_3x3_1()
	{
		let initial: Puzzle = vec![3, 0, 1, 2, 4, 6, 8, 5, 7];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(&initial, &goal, 3), 15);
	}

	#[test]
	fn linear_conflict_3x3_2()
	{
		let initial: Puzzle = vec![3, 8, 1, 6, 4, 5, 0, 2, 7];
        let goal: Puzzle = vec![8, 0, 1, 2, 3, 4, 5, 6, 7];

        assert_eq!(super::linear_conflict(&initial, &goal, 3), 22);
	}

	#[test]
	fn linear_conflict_4x4()
	{
		let initial: Puzzle = vec![4, 1, 15, 2, 6, 8, 5, 7, 12, 9, 3, 10, 14, 13, 11, 0];
        let goal: Puzzle = vec![15, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        assert_eq!(super::linear_conflict(&initial, &goal, 4), 46);
	}
}
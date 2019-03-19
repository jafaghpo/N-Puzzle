use crate::types::Puzzle;

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

pub fn out_of_axes(initial: &Puzzle, goal: &Puzzle, size: usize) -> usize
{
	initial.iter().enumerate().fold(0, | acc, (i, x) |
	{
		if *x == 0 { return acc }
		let same_row = i / size == goal[*x] / size;
		let same_column = i % size == goal[*x] % size;
		let res = acc + match (same_row, same_column)
		{
			(false, false) => 2,
			(false, true) | (true, false) => 1,
			(true, true) => 0
		};
		println!("res: {}, initial: {}, goal: {}", res, i, goal[*x]);
		return res;
	})
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
}
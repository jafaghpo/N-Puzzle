use crate::types::Puzzle;

// Returns the sum of inversions for each tiles except the empty one
fn get_inversions(puzzle: &Puzzle) -> usize
{
    let mut inversions = 0;
    for i in 0..puzzle.len() - 1
	{
        for j in i + 1..puzzle.len()
		{
            if puzzle[i] == 0 || puzzle[j] == 0 { continue }
            if puzzle[i] > puzzle[j] { inversions += 1 }
        }
    }
    return inversions;
}

// The solvability of a puzzle is explaned here (including inversions):
// http://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html
pub fn is_solvable(initial: &Puzzle, goal: &Puzzle, size: usize) -> bool
{
    let mut initial_inv = get_inversions(initial);
    let mut goal_inv = get_inversions(goal);

	// If the size is even, we take into account the position of the empty tile
    if size % 2 == 0
	{
        initial_inv += initial.iter().position(|x| *x == 0).unwrap() / size;
        goal_inv += goal.iter().position(|x| *x == 0).unwrap() / size;
    }
	// The "total" polarity (depending on the polarity of the size)
	// of a solvable puzzle MUST be the same as that of its final state
    return initial_inv % 2 == goal_inv % 2;
}

// Check the puzzle content and returns a vector containing all the tiles
pub fn check_puzzle(lines: Vec<Vec<&str>>, size: usize) -> Result<Puzzle, String>
{
	// Check if the number of rows and colums is equal to the puzzle size
	if lines.len() != size { return Err(format!("invalid number of rows")) }
	if lines.iter().any(|line| line.len() != size) { return Err(format!("invalid number of columns")) }

	// Check if the tiles are positive numbers and are in range
	let mut tiles: Vec<usize> = vec![];
	for line in lines
	{
		for number in line
		{
			match number.parse()
			{
				Ok(n) if (n < size * size) => { tiles.push(n) },
				Err(_) => { return Err(format!("invalid tile number: '{}'", number)) },
				_ => { return Err(format!("tile number not in range: '{}'", number)) }
			}
		}
	}

	// Check if there is duplicate numbers
	let mut number_list: Vec<bool> = vec![false; size * size];
	for number in &tiles
	{
		match number_list[*number]
		{
			true =>	{ return Err(format!("duplicate tile of '{}'", number)) },
			false => { number_list[*number] = true }
		}
	}
	return Ok(tiles);
}


#[cfg(test)]
mod tests
{
    use crate::types::Puzzle;

    #[test]
    fn inversions()
	{
        let a: Puzzle = vec![4, 3, 2, 1];

        assert_eq!(super::get_inversions(&a), 6);
    }

    #[test]
    fn inversions_ignore_zero()
	{
        let a: Puzzle = vec![3, 2, 1, 0];

        assert_eq!(super::get_inversions(&a), 3);
    }

    #[test]
    fn inversions_none()
	{
        let a: Puzzle = vec![1, 2, 3, 4];

        assert_eq!(super::get_inversions(&a), 0);
    }

    #[test]
    fn is_solvable_inverted()
	{
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Puzzle = vec![1, 3, 2, 4, 5, 6, 7, 8, 0];

        assert!(!super::is_solvable(&a, &b, 3));
        assert!(!super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_identity()
	{
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4()
	{
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 0, 15];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

    #[test]
    fn is_solvable_4x4_identity()
	{
        let a: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
        let b: Puzzle = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];

        assert!(super::is_solvable(&a, &b, 3));
        assert!(super::is_solvable(&b, &a, 3));
    }

	#[test]
	fn check_puzzle_valid()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"],
			vec!["7", "8", "0"]
		];

		match super::check_puzzle(a, 3)
		{
			Ok(_) => {},
			Err(e) => assert!(false, "did not expect error: [{}]", e)
		}
	}

	#[test]
	fn check_puzzle_invalid_rows()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"]
		];
		let expected = "invalid number of rows";

		match super::check_puzzle(a, 3)
		{
			Ok(_) => assert!(false, "should return an error"),
			Err(e) =>
			{
				assert!(
					e == expected,
					"expected [{}] instead of [{}]",
					expected, e
				);
			}
		}
	}

	#[test]
	fn check_puzzle_invalid_columns()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"],
			vec!["7", "8"],
		];
		let expected = "invalid number of columns";

		match super::check_puzzle(a, 3)
		{
			Ok(_) => assert!(false, "should return [{}]", expected),
			Err(e) =>
			{
				assert!(
					e == expected,
					"expected [{}] instead of [{}]",
					expected, e
				);
			}
		}
	}

	#[test]
	fn check_puzzle_invalid_tile_number()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"],
			vec!["7", "8", "error"]
		];
		let expected = "invalid tile number: 'error'";

		match super::check_puzzle(a, 3)
		{
			Ok(_) => assert!(false, "should return [{}]", expected),
			Err(e) =>
			{
				assert!(
					e == expected,
					"expected [{}] instead of [{}]",
					expected, e
				);
			}
		}
	}

	#[test]
	fn check_puzzle_invalid_tile_range()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"],
			vec!["7", "8", "9"]
		];
		let expected = "tile number not in range: '9'";

		match super::check_puzzle(a, 3)
		{
			Ok(_) => assert!(false, "should return [{}]", expected),
			Err(e) =>
			{
				assert!(
					e == expected,
					"expected [{}] instead of [{}]",
					expected, e
				);
			}
		}
	}

	#[test]
	fn check_puzzle_invalid_tile_duplicate()
	{
		let a: Vec<Vec<&str>> = vec!
		[
			vec!["1", "2", "3"],
			vec!["4", "5", "6"],
			vec!["7", "8", "3"]
		];
		let expected = "duplicate tile of '3'";

		match super::check_puzzle(a, 3)
		{
			Ok(_) => assert!(false, "should return [{}]", expected),
			Err(e) =>
			{
				assert!(
					e == expected,
					"expected [{}] instead of [{}]",
					expected, e
				);
			}
		}
	}
}
use crate::types::Puzzle;

// Generate puzzle in reversed style
pub fn classic(size: usize) -> Puzzle
{
	let mut goal: Puzzle = (1..size * size).collect();
	goal.push(0);
	return goal;
}

// Generate puzzle in reversed style
pub fn reversed(size: usize) -> Puzzle
{
	return (0..size * size).rev().collect();
}

// Generate puzzle in snail style
pub fn snail(size: usize) -> Puzzle
{
	let nb_tiles = size * size;
	let mut goal: Puzzle = vec![0; nb_tiles];
	let mut x: i32 = -1;
	let mut y: i32 = 0;
	let mut index: usize = 1;
	let mut inc: i32 = 1;
	let mut n = size;

	while index < nb_tiles
	{
		// Fill top and bottom
		for _ in 0..n
		{
			x += inc;
			goal[(x + y * size as i32) as usize] = index;
			index += 1;
		}
		n -= 1;
		// Fill left and right
		for _ in 0..n
		{
			y += inc;
			goal[(x + y * size as i32) as usize] = index;
			index += 1;
		}
		// Change fill direction
		inc = -inc;
	}
	return goal;
}


#[cfg(test)]
mod tests
{

    #[test]
    fn classic_3x3()
	{
		let expected = vec!
		[
			1, 2, 3,
			4, 5, 6,
			7, 8, 0
		];
		let got = super::classic(3);
        assert_eq!(got, expected);
    }

	#[test]
	fn classic_4x4()
	{
		let expected = vec!
		[
			1,	2,	3,	4,
			5,	6,	7,	8,
			9,	10,	11,	12,
			13,	14,	15,	0
		];
		let got = super::classic(4);
        assert_eq!(got, expected);
    }

	#[test]
	fn reversed_3x3()
	{
		let expected = vec!
		[
			8, 7, 6,
			5, 4, 3,
			2, 1, 0
		];
		let got = super::reversed(3);
        assert_eq!(got, expected);
    }

	#[test]
	fn reversed_4x4()
	{
		let expected = vec!
		[
			15,	14,	13,	12,
			11,	10,	9,	8,
			7,	6,	5,	4,
			3,	2,	1,	0
		];
		let got = super::reversed(4);
        assert_eq!(got, expected);
    }

	#[test]
    fn snail_3x3()
	{
		let expected = vec!
		[
			1, 2, 3,
			8, 0, 4,
			7, 6, 5
		];
		let got = super::snail(3);
        assert_eq!(got, expected);
    }

	#[test]
	fn snail_4x4()
	{
		let expected = vec!
		[
			1,	2,	3,	4,
			12,	13,	14,	5,
			11,	0,	15,	6,
			10,	9,	8,	7
		];
		let got = super::snail(4);
        assert_eq!(got, expected);
    }

}
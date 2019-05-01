extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;
use crate::{Map, Move, Position, Container};

pub struct Generator
{
	pub size: usize,
	pub iter: usize,
	pub target: String,
	pub pos: Position
}

impl Generator
{
	pub fn new(size: usize, iter: Option<usize>, level: &str, goal: &str, dir_path: &str) -> Self
	{
		let iterations = match iter
		{
			Some(i) => i,
			None => match level
			{
				"easy" => 5 * size,
				"normal" => 25 * size,
				"hard" => 100 * size,
				"epic" => 10000 * size,
				_ => 1
			}
		};

		let str_iter = if let Some(i) = iter { format!("{}", i) } else { level.to_owned() };
		let target = format!("{}/{}_{}_{3}x{3}", dir_path, goal, &str_iter, size);

		Generator
		{
			size: size,
			iter: iterations,
			target: target,
			pos: Position { x: 0, y: 0 }
		}
	}

	pub fn shuffle_map(&self, mut map: Map, mut pos: Position) -> Map
	{
		for _ in 0..self.iter
		{
			let moves: Vec<Move> = pos
				.possible_moves(self.size)
				.into_iter()
				.filter(|m| *m != Move::No)
				.collect();

			let mut rng = thread_rng();
			let movement = moves.choose(&mut rng).unwrap();
			map = movement.do_move(map, &pos, self.size);
			pos = pos.update(movement);
		}
		map
	}

	pub fn generate_map(&self, goal: &str) -> Result<String, String>
	{
		let map = Generator::generate_goal(goal, self.size);
		let index = map.iter().position(|&x| x == 0).unwrap();
		let pos = Position { x: index % self.size, y: index / self.size };
		let map = self.shuffle_map(map, pos);
		if let Err(e) = Container(map, self.size).create_file(&self.target)
		{
			return Err(e);
		}
		Ok(self.target.clone())
	}

	pub fn generate_goal(goal: &str, size: usize) -> Map
	{
		match goal
		{
			"classic" => Generator::classic(size),
			"reversed" => Generator::reversed(size),
			"snail" | _ => Generator::snail(size),
		}
	}

	pub fn classic(size: usize) -> Map
	{
		let mut goal: Map = (1..size * size).collect();
		goal.push(0);
		return goal;
	}

	// Generate puzzle in reversed style
	pub fn reversed(size: usize) -> Map
	{
		return (0..size * size).rev().collect();
	}

	// Generate puzzle in snail style
	pub fn snail(size: usize) -> Map
	{
		let nb_tiles = size * size;
		let mut goal: Map = vec![0; nb_tiles];
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
		let got = super::Generator::classic(3);
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
		let got = super::Generator::classic(4);
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
		let got = super::Generator::reversed(3);
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
		let got = super::Generator::reversed(4);
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
		let got = super::Generator::snail(3);
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
		let got = super::Generator::snail(4);
        assert_eq!(got, expected);
    }

}
use std::fmt;
use std::fs::File;
use std::io::Write;

pub mod solver;
pub mod node;
pub mod parser;
pub mod algorithm;
pub mod heuristic;
pub mod generator;
pub mod display;

pub type Map = Vec<usize>;

pub struct Container(pub Map, pub usize);

impl Container
{
    pub fn swap_indexes(&self) -> Map
    {
        self.0
			.iter()
			.enumerate()
			.fold(vec![0; self.0.len()], | mut acc, (i, x) | { acc[*x] = i; acc } )
    }

	pub fn create_file(&self, filepath: &str) -> Result<(), String>
	{
		let mut file = match File::create(filepath)
		{
			Ok(f) => Ok(f),
			Err(e) => Err(e.to_string())
		}?;

		let data = format!("{}\n{}", self.1, self.to_string());
		if let Err(e) = file.write_all(data.as_bytes())
		{
			return Err(e.to_string())
		};
		Ok(())
	}
}

impl fmt::Display for Container
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut to_display = String::new();
        for i in 0..self.0.len()
        {
            match i
            {
                0 => to_display.push_str(&format!("{}", self.0[i])),
                i if i % self.1 == 0 => to_display.push_str(&format!("\n{}", self.0[i])),
                _ => to_display.push_str(&format!("   {}", self.0[i]))
            }
        }
		to_display.push_str("\n");
        write!(f, "{}", to_display)
    }
}

pub struct Flag
{
	pub verbosity: bool,
	pub debug: bool,
	pub greedy: bool,
	pub uniform: bool
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Position
{
	pub x: usize,
	pub y: usize
}

impl Position
{
	pub fn as_index(&self, size: usize) -> usize
	{
		self.y * size + self.x
	}

	pub fn update(&self, movement: &Move) -> Position
	{
		match movement
		{
			Move::Left(x) | Move::Right(x) => Position
			{
				x: (self.x as i64 + x) as usize,
				y: self.y
			},
			Move::Up(_) => Position
			{
				x: self.x,
				y: (self.y as i64 - 1) as usize
			},
			Move::Down(_) => Position
			{
				x: self.x,
				y: (self.y as i64 + 1) as usize
			},
			Move::No => Position {x: self.x, y: self.y }
		}
	}

	pub fn moved_element(&self, movement: &Move) -> Position
	{
		match movement
		{
			Move::Left(_) => Position { x: self.x + 1, y: self.y },
			Move::Right(_) => Position { x: self.x - 1, y: self.y },
			Move::Up(_) => Position { x: self.x, y: self.y + 1 },
			Move::Down(_) => Position { x: self.x, y: self.y - 1 },
			Move::No => Position { x: self.x, y: self.y }
		}
	}

	#[inline]
	pub fn possible_moves(&self, size: usize) -> Vec<Move>
	{
		vec!
		[
			if self.x > 0 { Move::Left(-1) } else { Move::No },
			if self.x < size - 1 { Move::Right(1) } else { Move::No },
			if self.y > 0 { Move::Up(-(size as i64)) } else { Move::No },
			if self.y < size - 1 { Move::Down(size as i64) } else { Move::No }
		]
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Move
{
	Up(i64),
	Down(i64),
	Left(i64),
	Right(i64),
	No
}

impl Move
{
	pub fn do_move(&self, mut map: Map, pos: &Position, size: usize) -> Map
	{
		let pos = pos.as_index(size);
		let new_pos = (pos as i64 + self.get_offset()) as usize;
		let tmp = map[new_pos];
		map[new_pos] = map[pos];
		map[pos] = tmp;
		map
	}

	pub fn get_offset(&self) -> i64
	{
		match self
		{
			Move::Up(x) | Move::Down(x) | Move::Left(x) | Move::Right(x) => *x,
			Move::No => 0
		}
	}

	pub fn opposite(&self) -> Self
	{
		match self
		{
			Move::Left(x) => Move::Right(-x),
			Move::Right(x) => Move::Left(-x),
			Move::Up(x) => Move::Down(-x),
			Move::Down(x) => Move::Up(-x),
			Move::No => Move::No
		}
	}
}

impl fmt::Display for Move
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
    {
        let mut to_display = String::new();
        match self
		{
			Move::Left(_) => to_display.push_str("Left"),
			Move::Right(_) => to_display.push_str("Right"),
			Move::Up(_) => to_display.push_str("Up"),
			Move::Down(_) => to_display.push_str("Down"),
			Move::No => to_display.push_str("Start State"),
		};
        write!(f, "{}", to_display)
    }
}
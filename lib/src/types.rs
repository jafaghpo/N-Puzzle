use std::cmp::Ordering;

pub type Map = Vec<usize>;

pub type Parsed = (Map, Map, usize);

pub type CostFunc = Box<Fn(usize, usize) -> usize>;

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
	pub fn do_move(&self, map: &Map, pos: &Position, size: usize) -> Map
	{
		let mut map = map.clone();
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

pub struct Heuristic
{
	pub end: Map,
	pub size: usize,
	pub func: fn(&Map, &Map, usize) -> usize
}

impl Heuristic
{
	pub fn new(end: Map, size: usize, func: fn(&Map, &Map, usize) -> usize) -> Self
	{
		Self
		{
			end: end,
			size: size,
			func: func
		}
	}

	pub fn call(&self, current: &Map) -> usize
	{
		(self.func)(current, &self.end, self.size)
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node
{
	pub map: Map,
	pub pos: Position,
	pub movement: Move,
	pub h: usize,
	pub g: usize,
	pub f: usize
}

impl Node
{
	pub fn new(map: Map) -> Self
	{
		Self
		{
			map: map,
			pos: Position { x: 0, y: 0 },
			movement: Move::No,
			f: 0,
			g: 0,
			h: 0
		}
	}

	pub fn create_start(map: Map, size: usize, heuristic: &Heuristic) -> Self
	{
		let mut node = Node::new(map);
		let index = node.map.iter().position(|&x| x == 0).unwrap();
		node.pos.x = index % size;
		node.pos.y = index / size;
		node.h = heuristic.call(&node.map);
		node.f = node.h;
		node
	}

	pub fn generate_moves(&self, size: usize, possible_moves: &[Box<Fn(&Position, usize) -> Move>; 4]) -> Vec<Self>
	{
		let mut moves: Vec<Node> = vec![];
		for func in possible_moves
		{
			let movement = func(&self.pos, size);
			if movement == Move::No { continue }
			let map = movement.do_move(&self.map, &self.pos, size);
			let mut node = Node::new(map);
			node.pos = self.pos.update(&movement);
			node.movement = movement;
			node.g = self.g + 1;
			moves.push(node);
		}
		moves
	}
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Node) -> Option<Ordering>
	{
        Some(self.cmp(other))
    }
}

impl Ord for Node
{
    fn cmp(&self, other: &Node) -> Ordering
	{
        other.f.cmp(&self.f)
    }
}

pub struct State
{
	pub map: Map,
	pub movement: Move
}

pub struct Solution
{
	pub path: Vec<State>,
	pub moves: usize,
	pub selected_nodes: usize,
	pub total_nodes: usize,
}

impl Solution
{
	pub fn new() -> Self
	{
		Self
		{
			path: vec![],
			moves: 0,
			selected_nodes: 0,
			total_nodes: 0
		}
	}
}
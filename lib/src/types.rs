use std::cmp::Ordering;
use crate::heuristic;

pub type Map = Vec<usize>;

pub type Parsed = (Map, Map, usize);

pub struct Flag
{
	pub mem_limit: bool,
	pub display_bar: bool,
	pub verbosity: bool,
	pub debug: bool
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

pub struct Solver
{
	pub name: String,
	pub goal: Map,
	pub size: usize,
	pub first_cost: fn(Node, &Map, usize) -> Node,
	pub update_cost: fn(Node, &Map, usize) -> Node,
	pub uniform: bool,
	pub greedy: bool
}

impl Solver
{
	pub fn new(goal: Map, size: usize, name: &str, algo: &str) -> Self
	{
		let first_cost: fn(Node, &Map, usize) -> Node;
		let update_cost: fn(Node, &Map, usize) -> Node;
		match name
		{
			"misplaced_tiles" => 
			{
				first_cost = heuristic::misplaced_tiles;
				update_cost = heuristic::partial_misplaced;
			}
			"out_of_axes" =>
			{
				first_cost = heuristic::out_of_axes;
				update_cost = heuristic::partial_out_of_axes;
			}
			"linear_conflict" =>
			{
				first_cost = heuristic::linear_conflict;
				update_cost = heuristic::partial_conflict;
				// update_cost = heuristic::linear_conflict;
			}
			"manhattan" | _ =>
			{
				first_cost = heuristic::manhattan;
				update_cost = heuristic::partial_manhattan;
				// update_cost = heuristic::manhattan;
			}
		};

		Self
		{
			goal: goal,
			size: size,
			first_cost: first_cost,
			update_cost: update_cost,
			name: name.to_owned(),
			uniform: algo == "uniform",
			greedy: algo == "greedy"
		}
	}

	pub fn get_cost(&self, mut node: Node) -> Node
	{
		if self.uniform
		{
			node.f = node.g;
			return node;
		}
		node = (self.first_cost)(node, &self.goal, self.size);
		node.f = if self.greedy { node.h } else { node.h + node.g };
		node
	}

	pub fn update_cost(&self, mut node: Node) -> Node
	{
		if self.uniform
		{
			node.f = node.g;
			return node;
		}
		node = (self.update_cost)(node, &self.goal, self.size);
		node.f = if self.greedy { node.h } else { node.h + node.g };
		node
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node
{
	pub map: Map,
	pub cost: Map,
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
			cost: vec![0; map.len()],
			map: map,
			pos: Position { x: 0, y: 0 },
			movement: Move::No,
			f: 0,
			g: 0,
			h: 0
		}
	}

	pub fn find_position(&mut self, size: usize)
	{
		let index = self.map.iter().position(|&x| x == 0).unwrap();
		self.pos.x = index % size;
		self.pos.y = index / size;
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
			node.cost = self.cost.clone();
			node.pos = self.pos.update(&movement);
			node.movement = movement;
			node.g = self.g + 1;
			node.h = self.h;
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
	pub selected: usize,
	pub pending: usize,
	pub total: usize,
}

impl Solution
{
	pub fn new() -> Self
	{
		Self
		{
			path: vec![],
			moves: 0,
			pending: 0,
			selected: 0,
			total: 0
		}
	}
}
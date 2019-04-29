use std::cmp::Ordering;
use crate::heuristic;
use min_max_heap::MinMaxHeap;
use std::collections::HashMap;

pub type Map = Vec<usize>;

pub type Parsed = (Map, Map, usize);

pub struct Flag
{
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

	#[inline]
	pub fn possible_moves(&self, size: usize) -> [Move; 4]
	{
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
	pub end: Map,
	pub size: usize,
	pub first_cost: fn(Node, &Map, usize) -> Node,
	pub update_cost: fn(Node, &Map, usize) -> Node,
	pub uniform: bool,
	pub greedy: bool
}

impl Solver
{
	pub fn new(end: Map, goal: Map, size: usize, name: &str, algo: &str) -> Self
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
			}
			"manhattan" | _ =>
			{
				first_cost = heuristic::manhattan;
				update_cost = heuristic::partial_manhattan;
			}
		};

		Self
		{
			goal: goal,
			end: end,
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
			node.h = 1;
			return node;
		}
		node = (self.first_cost)(node, &self.end, self.size);
		match self.greedy
		{
			true => { node.f = node.h; node.t = node.g },
			false => { node.f = node.h + node.g; node.t = node.h }
		};
		node
	}

	pub fn update_cost(&self, mut node: Node) -> Node
	{
		if self.uniform
		{
			node.f = node.g;
			if node.map == self.goal { node.h = 0 }
			return node;
		}
		node = (self.update_cost)(node, &self.end, self.size);
		match self.greedy
		{
			true => { node.f = node.h; node.t = node.g },
			false => { node.f = node.h + node.g; node.t = node.h }
		};
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
	pub f: usize,
	pub t: usize
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
			h: 0,
			t: 0
		}
	}

	pub fn find_position(&mut self, size: usize)
	{
		let index = self.map.iter().position(|&x| x == 0).unwrap();
		self.pos.x = index % size;
		self.pos.y = index / size;
	}

	pub fn generate_moves(&self, size: usize) -> Vec<Self>
	{
		let possible_moves = self.pos.possible_moves(size);
		let mut moves: Vec<Node> = vec![];
		for movement in &possible_moves
		{
			if *movement == Move::No { continue }
			let map = movement.do_move(&self.map, &self.pos, size);
			let mut node = Node::new(map);
			node.cost = self.cost.clone();
			node.pos = self.pos.update(&movement);
			node.movement = movement.clone();
			node.g = self.g + 1;
			node.h = self.h;
			moves.push(node);
		}
		moves
	}

	pub fn get_solution(&self, end: Map, size: usize, open_set: &MinMaxHeap<Node>, closed_set: &HashMap<Map, Move>) -> Solution
	{
		let mut solution = Solution::new();
		solution.selected = closed_set.len();
		solution.pending = open_set.len();
		solution.total = open_set.len() + closed_set.len();

		let mut last_pos = self.pos.clone();
		solution.path = vec![State { map: end, movement: self.movement.clone() }];
		loop
		{
			let last = solution.path.last().unwrap();
			if last.movement == Move::No { break }
			let map = last.movement.opposite().do_move(&last.map, &last_pos, size);
			let movement = closed_set.get(&map).unwrap();
			last_pos = last_pos.update(&last.movement.opposite());
			solution.path.push(State {map: map, movement: movement.clone() });
		}
		solution.moves = solution.path.len() - 1;
		solution
	}
}

impl Ord for Node
{
    fn cmp(&self, other: &Node) -> Ordering
	{
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Node) -> Option<Ordering>
	{
		Some(other.f.cmp(&self.f).then(other.t.cmp(&self.t)))
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
use std::cmp::Ordering;
use crate::{Map, Move, Position};

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
			let map = movement.do_move(self.map.clone(), &self.pos, size);
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
		// Some(other.f.cmp(&self.f))
		Some(other.f.cmp(&self.f).then(other.t.cmp(&self.t)))
    }
}
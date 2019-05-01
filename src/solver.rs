use crate::{Map, Flag};
use crate::heuristic;
use crate::node::{Node, Move};
use std::time::{Instant};

pub struct Solver
{
	pub name: String,
	pub goal: Map,
	pub end: Map,
	pub size: usize,
	pub first_cost: fn(Node, &Map, usize) -> Node,
	pub update_cost: fn(Node, &Map, usize) -> Node,
	pub flag: Flag,
	pub time: Instant
}

impl Solver
{
	pub fn new(goal: Map, size: usize, name: &str, flag: Flag, time: Instant) -> Self
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
		}

		Self
		{
			end: Solver::swap_indexes(&goal),
			goal: goal,
			size: size,
			first_cost: first_cost,
			update_cost: update_cost,
			name: name.to_owned(),
			flag: flag,
			time: time
		}
	}

	// The solvability of a puzzle is explaned here (including inversions):
	// http://www.cs.bham.ac.uk/~mdr/teaching/modules04/java2/TilesSolvability.html
	pub fn is_solvable(&self, start: &Map) -> Result<(), String>
	{
		// Returns the sum of inversions for each tiles except the empty one
		fn get_inversions(map: &Map) -> usize
		{
			let mut inversions = 0;
			for i in 0..map.len() - 1
			{
				for j in i + 1..map.len()
				{
					if map[i] == 0 || map[j] == 0 { continue }
					if map[i] > map[j] { inversions += 1 }
				}
			}
			return inversions;
		}

		let mut start_inv = get_inversions(start);
		let mut end_inv = get_inversions(&self.goal);

		// If the size is even, we take into account the position of the empty tile
		if self.size % 2 == 0
		{
			start_inv += start.iter().position(|x| *x == 0).unwrap() / self.size;
			end_inv += self.goal.iter().position(|x| *x == 0).unwrap() / self.size;
		}
		// The "total" polarity (depending on the polarity of the size)
		// of a solvable puzzle MUST be the same as that of its final state
		match start_inv % 2 == end_inv % 2
		{
			true => Ok(()),
			false => Err("unsolvable puzzle".to_owned())
		}
	}

	// Swap indexes of a vector with their respective values
	pub fn swap_indexes(vec: &Map) -> Map
	{
		vec
			.iter()
			.enumerate()
			.fold(vec![0; vec.len()], | mut acc, (i, x) | { acc[*x] = i; acc } )
	}

	pub fn get_cost(&self, mut node: Node) -> Node
	{
		if self.flag.uniform
		{
			node.f = node.g;
			node.h = 1;
			return node;
		}
		node = (self.first_cost)(node, &self.end, self.size);
		match self.flag.greedy
		{
			true => { node.f = node.h; node.t = node.g },
			false => { node.f = node.h + node.g; node.t = node.h }
		};
		node
	}

	pub fn update_cost(&self, mut node: Node) -> Node
	{
		if self.flag.uniform
		{
			node.f = node.g;
			if node.map == self.goal { node.h = 0 }
			return node;
		}
		node = (self.update_cost)(node, &self.end, self.size);
		match self.flag.greedy
		{
			true => { node.f = node.h; node.t = node.g },
			false => { node.f = node.h + node.g; node.t = node.h }
		};
		node
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
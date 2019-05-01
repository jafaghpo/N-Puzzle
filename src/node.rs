use min_max_heap::MinMaxHeap;
use std::collections::HashMap;
use colored::Colorize;
use std::cmp::Ordering;
use crate::{Map, Move, Position};
use crate::solver::{Solver, Solution, State};

fn display_map(map: &Map, size: usize)
{
	for i in 0..(size * size)
	{
		if i != 0 && i % size == 0 { println!("\n") }
		print!("    {}\t", map[i]);
	}
	println!("\n\n------------------------------------");
}

fn display_path(mut path: Vec<State>, size: usize)
{
	while let Some(state) = path.pop()
	{
		match state.movement
		{
			Move::Left(_) => println!("\t[Left]"),
			Move::Right(_) => println!("\t[Right]"),
			Move::Up(_) => println!("\t[Up]"),
			Move::Down(_) => println!("\t[Down]"),
			Move::No => println!("\t[Start State]"),
		};
		display_map(&state.map, size);
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
			let map = movement.do_move(&self.map.clone(), &self.pos, size);
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

	pub fn get_solution(self, solver: Solver, open_set: MinMaxHeap<Node>, mut closed_set: HashMap<Map, Move>)
	{
		let mut solution = Solution::new();
		solution.selected = closed_set.len();
		solution.pending = open_set.len();
		solution.total = open_set.len() + closed_set.len();

		let mut last_pos = self.pos;
		solution.path = vec![State { map: solver.goal, movement: self.movement }];
		loop
		{
			let last = solution.path.last().unwrap();
			if last.movement == Move::No { break }
			let map = last.movement.opposite().do_move(&last.map.clone(), &last_pos, solver.size);
			let movement = closed_set.remove(&map).unwrap();
			last_pos = last_pos.update(&last.movement.opposite());
			solution.path.push(State {map: map, movement: movement });
		}
		solution.moves = solution.path.len() - 1;
		if solver.flag.verbosity 
		{
			display_path(solution.path, solver.size);
			println!("Number of pending states (open set): {}", solution.pending.to_string().green());
			println!("Number of selected states (closed set): {}", solution.selected.to_string().red());
			println!("Number of states ever represented in memory: {}", solution.total.to_string().cyan());
		}
		println!("Number of moves: {}", solution.moves.to_string().yellow());
		println!("Execution time: {}", &format!("{:?}",solver.time.elapsed()).bright_blue().bold());
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
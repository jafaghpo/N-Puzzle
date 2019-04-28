use std::collections::HashMap;
use min_max_heap::MinMaxHeap;
use crate::types::{Flag, Map, Node, Solver, Move, State, Solution};

fn display_map(map: &Map, size: usize)
{
	for i in 0..(size * size)
	{
		if i != 0 && i % size == 0 { println!("\n") }
		print!(" {}\t", map[i]);
	}
	println!("\n");
}

fn debug_parent(node: &Node, index: usize, size: usize, open_set: &MinMaxHeap<Node>, closed_set: &HashMap<Map, Move>)
{
	println!("Parent N°{}", index);
	display_map(&node.map, size);
	println!("Costs: f({}) g({}) h({})", node.f, node.g, node.h);
	println!("Position: x({}) y({})", node.pos.x, node.pos.y);
	println!("Move: {:?}", node.movement);
	println!("Open set: {} | Closed set: {}", open_set.len(), closed_set.len());
	println!("Open set capacity: {} | Closed set capacity: {}", open_set.capacity(), closed_set.capacity());
	println!("");
}

fn debug_child(node: &Node, index: usize)
{
	println!("   Child N°{}", index);
	println!("   Costs: f({}) g({}) h({})", node.f, node.g, node.h);
	println!("   Position: x({}) y({})", node.pos.x, node.pos.y);
	println!("   Move: {:?}", node.movement);
	println!("");
}

pub fn solve(start: Map, size: usize, solver: &Solver, flag: &Flag) -> Solution
{
	let mut start = Node::new(start);
	start.find_position(size);
	start = solver.get_cost(start);

	let mut open_set: MinMaxHeap<Node> = MinMaxHeap::new();
	let mut closed_set: HashMap<Map, Move> = HashMap::new();

	let mut best_h = start.h;
	let max_h: f32 = start.h as f32;
	let mut i: f32 = 0.0;
	let mut _percent: f32 = 0.0;

	open_set.push(start);

	let mut _debug_parent = 1;

	let (last_map, last_move, mut last_pos) = loop
	{
		// Get the node with the lowest f cost
		let current = open_set.pop_max().unwrap();

		// Add the current node to the closed set
		closed_set.insert(current.map.clone(), current.movement.clone());

		if flag.debug == false && flag.display_bar && current.h < best_h
		{
			i += (best_h - current.h) as f32;
			_percent = i / max_h * 100.0;
			println!("progress: {:} of {:}, capacity: {} open set: {}, closed set {}, percent = {:.2}%",
				i, max_h, open_set.capacity(), open_set.len(), closed_set.len(), _percent);
			best_h = current.h
		}

		// If the end node is found
		if current.h == 0 { break (current.map, current.movement, current.pos) }

		if flag.debug
		{
			debug_parent(&current, _debug_parent, size, &open_set, &closed_set);
			_debug_parent += 1;
		}

		// Get the list of possible moves
		let moves: Vec<Node> = current.generate_moves(size);
		let mut _debug_child = 1;
		for mut node in moves
		{
			if closed_set.contains_key(&node.map) { continue }
			node = solver.update_cost(node);

			if flag.debug
			{
				debug_child(&node, _debug_child);
				_debug_child += 1;
			}

			open_set.push(node);
		}
	};

	let mut solution = Solution::new();
	solution.selected = closed_set.len();
	solution.pending = open_set.len();
	solution.total = open_set.len() + closed_set.len();

	// Get solution path
	solution.path = vec![State { map: last_map, movement: last_move }];
	loop
	{
		let last = solution.path.last().unwrap();
		if last.movement == Move::No { break }
		let map = last.movement.opposite().do_move(&last.map, &last_pos, size);
		let movement = closed_set.remove(&map).unwrap();
		last_pos = last_pos.update(&last.movement.opposite());
		solution.path.push(State {map: map, movement: movement });
	}

	solution.moves = solution.path.len() - 1;
	solution
}
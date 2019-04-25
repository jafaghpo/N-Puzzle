use std::collections::{HashMap, BinaryHeap};
use std::{thread, time};
use crate::types::{Map, Node, Heuristic, CostFunc, Position, Move, State, Solution};
//use indicatif::{ ProgressBar, ProgressStyle };

// fn create_progress_bar(size: usize) -> ProgressBar
// {
// 	let bar = ProgressBar::new(size as u64);
// 	bar.set_style(ProgressStyle::default_bar()
// 		.template("[{bar:100.cyan}] {eta_precise:.magenta} | Open set: {pos:.green} Closed set: {msg:.red}")
// 		.progress_chars(" ✈ "));
// 	bar
// }

pub fn solve(start: Map, size: usize, heuristic: &Heuristic, get_cost: &CostFunc) -> Solution
{
	let possible_moves: [Box<Fn(&Position, usize) -> Move>; 4] =
	[
		Box::new(|pos, _size| if pos.x > 0 { Move::Left(-1) } else { Move::No }),
		Box::new(|pos, size| if pos.x < size - 1 { Move::Right(1) } else { Move::No }),
		Box::new(|pos, size| if pos.y > 0 { Move::Up(-(size as i64)) } else { Move::No }),
		Box::new(|pos, size| if pos.y < size - 1 { Move::Down(size as i64) } else { Move::No })
	];
	let ten_millis = time::Duration::from_millis(10);
	let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
	let mut closed_set: HashMap<Map, Move> = HashMap::new();
	let start_node = Node::create_start(start, size, heuristic);
	let mut best_h = start_node.h;
	let max: f32 = start_node.h as f32;
	let mut i: f32 = 0.0;
	open_set.push(start_node);

	// let bar = ProgressBar::new(best_h as u64);
	// bar.set_style(ProgressStyle::default_bar()
	// 	.template("[{bar:100.cyan}] {eta_precise:.magenta} | Open set: {pos:.green} Closed set: {msg:.red}")
	// 	.progress_chars(" i "));
	let mut percent: f32 = 0.0;
	let (last_map, last_move, mut last_pos) = loop
	{
		// Get the node with the lowest f cost
		let current_node = open_set.pop().unwrap();
	
		// Add the current node to the closed set
		closed_set.insert(current_node.map.clone(), current_node.movement.clone());

		// Update progress bar
		// bar.set_position(open_set.len() as u64);
		// bar.set_message(&format!("{}", closed_set.len()));
		if current_node.h < best_h
		{
			i += (best_h - current_node.h) as f32;
			percent = i / max * 100.0;
			println!("progress: {:} of {:}, percent = {:.2}%", i, max, percent);
			thread::sleep(ten_millis);
			//bar.inc((best_h - current_node.h) as u64);
			best_h = current_node.h
		}

		// If the end node is found
		if current_node.h == 0 { break (current_node.map, current_node.movement, current_node.pos) }

		// Get the list of possible moves
		let moves: Vec<Node> = current_node.generate_moves(size, &possible_moves);
		for mut node in moves
		{
			if closed_set.contains_key(&node.map) { continue }
			node.h = heuristic.call(&node.map);
			node.f = get_cost(node.h, node.g);
			open_set.push(node);
		}
	};

	//bar.finish_and_clear();

	let mut solution = Solution::new();
	solution.selected_nodes = closed_set.len();
	solution.total_nodes = open_set.len() + closed_set.len();

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

	solution.moves = solution.path.len();
	solution
}
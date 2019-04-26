use std::collections::HashMap;
use min_max_heap::MinMaxHeap;
use crate::types::{Flag, Map, Node, Solver, Position, Move, State, Solution};
use crate::heuristic::manhattan;

fn get_capacity(h_value: usize, mem_limit: bool, name: &str) -> usize
{
	let mut power = if h_value < 9
	{
		1 + h_value as u32
	}
	else
	{
		11 + 2 * (h_value / 5 - 3) as u32
	};
	power = match name
	{
		"linear_conflict" => power,
		"manhattan" => power + 1,
		"out_of_axes" => power + 2,
		"misplaced_tiles" => power + 3
	};
	let capacity = 2usize.pow(power);
	if mem_limit { println!("Open set limited to 2^{} nodes ({} nodes)", power, capacity) }
	capacity
}

pub fn solve(start: Map, size: usize, solver: &Solver, flag: &Flag) -> Solution
{
	let possible_moves: [Box<Fn(&Position, usize) -> Move>; 4] =
	[
		Box::new(|pos, _size| if pos.x > 0 { Move::Left(-1) } else { Move::No }),
		Box::new(|pos, size| if pos.x < size - 1 { Move::Right(1) } else { Move::No }),
		Box::new(|pos, size| if pos.y > 0 { Move::Up(-(size as i64)) } else { Move::No }),
		Box::new(|pos, size| if pos.y < size - 1 { Move::Down(size as i64) } else { Move::No })
	];

	let mut start = Node::new(start);
	start.find_position(size);
	start = solver.get_cost(start);

	let capacity = get_capacity(start.h, flag.mem_limit, &solver.name);
	let mut open_set: MinMaxHeap<Node> = MinMaxHeap::with_capacity(capacity);
	let mut closed_set: HashMap<Map, Move> = HashMap::with_capacity(capacity);

	let mut best_h = start.h;
	let max_h: f32 = start.h as f32;
	let mut i: f32 = 0.0;
	let mut _percent: f32 = 0.0;

	open_set.push(start);

	let (last_map, last_move, mut last_pos) = loop
	{
		// Get the node with the lowest f cost (or highest for uniform)
		let current = match solver.uniform
		{
			true => open_set.pop_max().unwrap(),
			false => open_set.pop_min().unwrap(),
		};
	
		// Add the current node to the closed set
		closed_set.insert(current.map.clone(), current.movement.clone());


		if flag.display_bar && current.h < best_h
		{
			i += (best_h - current.h) as f32;
			_percent = i / max_h * 100.0;
			println!("progress: {:} of {:}, capacity: {} open set: {}, closed set {}, percent = {:.2}%",
				i, max_h, open_set.capacity(), open_set.len(), closed_set.len(), _percent);
			best_h = current.h
		}

		// If the end node is found
		if current.h == 0 { break (current.map, current.movement, current.pos) }

		// Get the list of possible moves
		let moves: Vec<Node> = current.generate_moves(size, &possible_moves);
		for mut node in moves
		{
			if closed_set.contains_key(&node.map) { continue }
			node = solver.get_cost(node);

			// Get rid of the lastest priority node if the size is reaching max capacity
			if flag.mem_limit && open_set.len() == capacity
			{
				open_set.push_pop_max(node);
			}
			else
			{
				open_set.push(node);
			}
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
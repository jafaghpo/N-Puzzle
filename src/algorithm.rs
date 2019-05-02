use std::collections::{HashMap, BinaryHeap};
use crate::{Map, Move};
use crate::node::Node;
use crate::solver::Solver;
use crate::display::{Info, Debug, Solution, State};

pub fn astar(start: Map, solver: Solver)
{
	let mut start = Node::new(start);
	start.find_position(solver.size);
	start = solver.get_cost(start);

	let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
	let mut closed_set: HashMap<Map, Move> = HashMap::new();

	let mut info = Info::new(start.h);
	let mut debug = Debug { parent_count: 1, child_count: 1 };

	open_set.push(start);

	let (mut last_pos, last_move) = loop
	{
		// Get the node with the lowest f cost
		let mut current = open_set.pop().unwrap();

		if solver.flag.debug { current = debug.parent(current, solver.size, &open_set, &closed_set) }

		// Get the list of possible moves
		let moves: Vec<Node> = current.generate_moves(solver.size);

		if solver.flag.debug == false && current.h < info.min_h
		{
			info.update(current.h, open_set.len(), closed_set.len());
		}

		// If the solution is found
		if current.h == 0
		{
			let end_pos = current.pos.clone();
			let end_move = current.movement.clone();
			closed_set.insert(current.map, current.movement.clone());
			if solver.flag.debug == false { info.bar.unwrap().finish() }
			break (end_pos, end_move)
		}
		else
		{
			closed_set.insert(current.map, current.movement.clone());
		}

		// Get the costs of child nodes and push them in the open set
		for mut node in moves
		{
			if closed_set.contains_key(&node.map) { continue }
			node = solver.update_cost(node);

			if solver.flag.debug { debug.child(&node) }

			open_set.push(node);
		}
	};

	let mut solution = Solution::new(open_set.len(), closed_set.len());
	solution.path = vec![State { map: solver.goal, movement: last_move }];
	loop
	{
		let last = solution.path.last().unwrap();
		if last.movement == Move::No { break }
		let map = last.movement.opposite().do_move(last.map.clone(), &last_pos, solver.size);
		let movement = closed_set.remove(&map).unwrap();
		last_pos = last_pos.update(&last.movement.opposite());
		solution.path.push(State {map: map, movement: movement });
	}
	solution.moves = solution.path.len() - 1;
	solution.display_all(solver.size, solver.flag.verbosity, solver.time);
}
use std::collections::{HashSet, BinaryHeap};
use crate::{Map, Move};
use crate::node::Node;
use crate::solver::Solver;
use crate::display::{Info, Debug, Solution, State};

pub fn solve(start: Map, solver: Solver) -> Result<(), String>
{
	let max_iter = 10;
	let mut iter = 1;
	let mut start = Node::new(start);
	start.find_position(solver.size);
	start = solver.get_cost(start);
	start.move_list.push(Move::No);

	let mut info = Info::new(start.h);
	let mut debug = Debug { parent_count: 1, child_count: 1 };
	let mut open_max = 0;
	let mut closed_max = 0;

	let mut limit = start.h;
    let mut best_node = start;

	let mut end_node = loop
	{
		if iter > max_iter
		{
			return Err(format!("Search exceeded the iteration limit ({}) without finding a solution", max_iter));
		}

        let mut closed_set: HashSet<Map> = HashSet::new();
        let mut list = expand_node(best_node, iter, limit, &mut closed_set, &solver, &mut debug);

        let lowest = list.peek().unwrap();

		// let lowest = list.peek().unwrap();

		if open_max < list.len() { open_max = list.len() }
		if closed_max < closed_set.len() { closed_max = closed_set.len() }
		// println!("iter: {} open size: {} closed size: {} lowest h: {} limit: {}", iter, open_max, closed_max, lowest.h, limit);
		if solver.flag.debug == false && lowest.h < info.min_h { info.update(lowest.h, open_max, closed_max) }

		if lowest.h == 0
		{
            let mut end_node = list.pop().unwrap();
            if solver.flag.debug { end_node = debug.parent(end_node, solver.size, list.len() + 1, closed_set.len()) }
			if solver.flag.debug == false { info.bar.unwrap().finish() }
			break end_node
		}
		limit = lowest.f;
		best_node = list.pop().unwrap();
        if solver.flag.debug { best_node = debug.parent(best_node, solver.size, list.len() + 1, closed_set.len()) }
		iter += 1;
	};

	let mut solution = Solution::new(open_max, closed_max);
	let mut pos = end_node.pos;
	let mut map;
	let mut state = State { map: end_node.map, movement: Move::No };
	while let Some(movement) = end_node.move_list.pop()
	{
		let opposite_move = movement.opposite();
		state.movement = movement;
		map = state.map.clone();
		solution.path.push(state);
		map = opposite_move.do_move(map, &pos, solver.size);
		pos = pos.update(&opposite_move);
		state = State { map: map, movement: Move::No };
	}
	solution.moves = solution.path.len() - 1;
	solution.display_all(solver.size, solver.flag.verbosity, solver.time);
	Ok(())
}

pub fn expand_node(node: Node, iter: usize, limit: usize, closed_set: &mut HashSet<Map>, solver: &Solver, debug: &mut Debug) -> BinaryHeap<Node>
{
	let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
	let mut node_list: BinaryHeap<Node> = BinaryHeap::new();

	open_set.push(node);
	loop
	{
		let current = open_set.pop();
		if current.is_none() { break }
		let mut current = current.unwrap();
		if current.f > limit
		{
			// if current.depth < iter { continue }
			node_list.push(current);
			continue
		}

		// if solver.flag.debug { current = debug.parent(current, solver.size, open_set.len(), closed_set.len()) }

		// If the solution is found
		if current.h == 0
		{
			current.f = 0;
			node_list.push(current);
			break
		}
        // println!("here");

		// Get the list of possible moves
		let moves: Vec<Node> = current.generate_moves(solver.size);

		// Get the costs of child nodes and push them in the open set
		for mut node in moves
		{
			if closed_set.contains(&node.map) { continue }
			node = solver.update_cost(node);
			node.move_list = current.move_list.clone();
			node.move_list.push(node.movement.clone());
			node.depth = iter;

			// if solver.flag.debug { debug.child(&node) }

			if limit < node.f { node_list.push(node) }
			else { open_set.push(node) }
		}
		
		closed_set.insert(current.map);
	}
	node_list
}
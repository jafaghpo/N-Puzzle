use std::collections::{HashSet, BinaryHeap};
use crate::{Map, Move};
use crate::node::Node;
use crate::solver::Solver;
use crate::display::{Info, Solution, State};

pub fn solve(start: Map, solver: Solver) -> Result<(), String>
{
	let max_iter = 1000;
	let mut iter = 1;
	let mut start = Node::new(start);
	start.find_position(solver.size);
	start = solver.get_cost(start);
	start.move_list.push(Move::No);

	let mut info = Info::new(start.h);
	let mut open_max = 0;
	let mut closed_max = 0;

	let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
	let mut limit = start.h;

	open_set.push(start);

	let mut end_node = loop
	{
		let mut list: BinaryHeap<Node> = BinaryHeap::new();
		let mut closed_set: HashSet<Map> = HashSet::new();

		if iter > max_iter
		{
			return Err(format!("Search exceeded the iteration limit ({}) without finding a solution", max_iter));
		}
		while let Some(node) = open_set.pop()
		{
			list.append(&mut expand_node(node, iter, limit, &mut closed_set, &solver));
			let lowest = list.peek().unwrap();
			if lowest.h == 0 { break }
		}

		let lowest = list.peek().unwrap();

		if open_max < list.len() { open_max = list.len() }
		if closed_max < closed_set.len() { closed_max = closed_set.len() }
		if solver.flag.uniform == false && lowest.h < info.min_h { info.update(lowest.h, open_max, closed_max) }

		if lowest.h == 0 { break list.pop().unwrap() }
		limit = lowest.f;

		open_set = list;
		iter += 1;
	};

	if solver.flag.uniform == false { info.bar.unwrap().finish() }

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
	solution.display(solver.size, solver.flag.verbosity, solver.time, solver.flag.uniform);
	Ok(())
}

pub fn expand_node(node: Node, iter: usize, limit: usize, closed_set: &mut HashSet<Map>, solver: &Solver) -> BinaryHeap<Node>
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
			if current.depth < iter { continue }
			node_list.push(current);
			continue
		}

		// If the solution is found
		if current.h == 0
		{
			current.f = 0;
			node_list.push(current);
			break
		}

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

			if limit < node.f { node_list.push(node) }
			else { open_set.push(node) }
		}
		
		closed_set.insert(current.map);
	}
	node_list
}
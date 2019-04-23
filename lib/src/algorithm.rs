use std::collections::{HashMap, BinaryHeap};
use crate::types::{Map, Node, Heuristic, CostFunc, ClosedSet, Solution};
use indicatif::{ ProgressBar, ProgressStyle };


pub fn solve(start: Map, heuristic: &Heuristic, get_cost: &CostFunc) -> Solution
{

	let start_node = Node::create_start(start);
	let mut open_set: BinaryHeap<Node> = BinaryHeap::new();
	start_node.h = heuristic.call(start_node.map);
	open_set.push(start_node);

	let mut closed_set = ClosedSet::new();

	let bar = ProgressBar::new(start_node.h);
	bar.set_style(ProgressStyle::default_bar()
    .template("[{bar:100.cyan}] {eta_precise:.magenta} | Open set: {pos:.green} Closed set: {msg:.red}")
    .progress_chars(" ✈ "));
    let mut best_h = <usize>::max_value();

	let end_node = loop
	{
		// Get the node with the lowest f cost
		let current_node = open_set.pop().unwrap();

		// Add the current node to the closed set
		closed_set.insert(current_node.id.clone(), current_node.parent_id.clone());

		// If the end node is found
		if current_node.h == 0 { break current_node }

		// Get the list of possible moves with their costs already generated
		let children = current_node.generate_children(&closed_set, heuristic, get_cost);

		// Add all possible nodes in the open set
		for node in children { open_set.push(node); }

		bar.set_position(open_set.len() as u64);
		bar.set_message(&format!("{}", closed_set.len()));

		current_node.h
		bar.inc(1);
	};

	bar.finish_and_clear();

	// Get solution path
	let mut path = vec![ end_node.id.clone() ];
	let mut id = &end_node.id;
	while let Some(x) = closed_set.get(id)
	{
		path.push(x.to_string());
		id = &x;
	}
	
	Solution
	{
		moves: path.len(),
		path: path,
		selected_nodes: closed_set.len(),
		total_nodes: open_set.len() + closed_set.len(),
	}
}
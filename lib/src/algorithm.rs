use crate::types::{Map, Node, Heuristic, CostFunc, OpenSet, ClosedSet, Solution};

fn print_puzzle(puzzle: &Map, size: usize)
{
	for i in 0..(size * size)
	{
		if i % size == 0 { println!("") }
		print!("{}\t", puzzle[i]);
	}
	println!("");
}

pub fn solve(start: Map, heuristic: &Heuristic, get_cost: &CostFunc) -> Solution
{
	let start_node = Node::create_start(start);
	let mut open_set = OpenSet::new(start_node);
	let mut closed_set = ClosedSet::new();

	let end_node = loop
	{
		// Get the node with the lowest f cost
		let current_node = open_set.pop_lowest();

		// Add the current node to the closed set
		closed_set.insert(current_node.id.clone(), current_node.parent_id.clone());

		// If the end node is found
		if current_node.cost.h == 0 { break current_node }

		// Get the list of possible moves with their costs already generated
		let node_list = current_node.generate_children(&closed_set, heuristic, get_cost);

		// Add all possible nodes in the open set
		open_set.insert_list(node_list);
	};

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
		total_nodes: open_set.list.len() + closed_set.len(),
	}
}
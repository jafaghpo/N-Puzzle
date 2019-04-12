use types;
use std::collections::HashMap;

fn resolve(start: Map, end: Map, heuristic: Heuristic, size: usize, f_cost: fn(usize, usize) -> usize) -> Solution
{
	// let f_cost = match algo
	// {
	// 	"a_start"		=> | h, g | h + g,
	// 	"uniform_cost"	=> | h, g | g,
	// 	"greedy"		=> | h, g | h
	// }

	let open_set = OpenSet::new();
	let closed_set: HashMap<String, Node> = HashMap::new();
	let start_node = 

}
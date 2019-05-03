use crate::Map;
use crate::node::Node;
use crate::solver::Solver;
use crate::display::{Info, Solution, State};

pub struct Results
{
    pub path: Option<Vec<State>>,
    pub bound: usize,
    pub expanded: usize,
    pub best_h: usize
}

pub fn solve(start: Map, mut solver: Solver) -> Result<(), String>
{
    solver.flag.greedy = false;
	let mut start = Node::new(start);
	start.find_position(solver.size);
	start = solver.get_cost(start);

    let mut info = Info::new(start.h);
    let max_bound = <usize>::max_value();
	let mut bound = start.h;
    let mut total_expanded = 0;
    let mut max_expanded = 0;
    let result = loop
    {
        let res = find_path(&start, 0, bound, &solver);
        if res.expanded > max_expanded { max_expanded = res.expanded }
        total_expanded += res.expanded;
        if solver.flag.uniform == false { info.update_ida(res.best_h, max_expanded, total_expanded) }
        if res.path.is_some() { break res }
        if res.bound == max_bound { return Err("solution not found".to_owned()) }
        bound = res.bound;
    };

    if solver.flag.uniform == false { info.bar.unwrap().finish() }
    let mut solution = Solution::new(max_expanded, total_expanded);
    solution.path = result.path.unwrap();
    solution.moves = solution.path.len() - 1;
    solution.display_ida(solver.size, solver.flag.verbosity, solver.time, solver.flag.uniform);
    Ok(())
}

pub fn find_path(current: &Node, g_cost: usize, bound: usize, solver: &Solver) -> Results
{
    let mut next_bound = <usize>::max_value();
    let mut best_h = current.h;
    
    let mut expanded = 0;
    if current.f > bound
    {
        return Results {path: None, bound: current.f, expanded: expanded, best_h: best_h };
    }
    if current.h == 0
    {
        return Results { path: Some(vec![]), bound: current.f, expanded: expanded + 1, best_h: 0 }
    }

    let moves: Vec<Node> = current.generate_moves(solver.size);
    expanded += 1;
    for mut node in moves
    {
        node = solver.update_cost(node);
        let result = find_path(&node, g_cost + 1, bound, solver);
        expanded += result.expanded;
        if expanded > 10000000
        {
            return Results {path: None, bound: <usize>::max_value(), expanded, best_h: node.h }
        }
        if result.path.is_some()
        {
           let mut path = result.path.unwrap();
           path.push(State {map: node.map, movement: node.movement });
           return Results { path: Some(path), bound: bound, expanded: expanded, best_h: 0 };
        }
        if result.bound < next_bound { next_bound = result.bound }
        if result.best_h < best_h { best_h = result.best_h }
    }
    Results { path: None, bound: next_bound, expanded: expanded, best_h: best_h }
}
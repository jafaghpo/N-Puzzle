use indicatif::{ProgressBar, ProgressStyle};
use colored::*;
use std::collections::{HashMap, BinaryHeap};
use std::time::Instant;

use crate::node::Node;
use crate::{Container, Map, Move};

pub struct Info
{
    pub bar: Option<ProgressBar>,
    pub max_h: usize,
    pub min_h: usize,
    pub count: f32
}

impl Info
{
    pub fn new(max_h: usize) -> Self
    {
        Self
        {
            bar: None,
            max_h: max_h,
            min_h: max_h,
            count: 0.0
        }
    }

    pub fn update(&mut self, current_h: usize, open_size: usize, closed_size: usize)
    {
        if self.bar.is_none()
        {
            let bar = ProgressBar::new(self.max_h as u64);
            bar.set_style(ProgressStyle::default_bar()
                .template(&format!("{{pos:}} of {:} | {{msg:}}", self.max_h)));
            self.bar = Some(bar);
        }

        match self.bar
        {
            Some(ref bar) =>
            {
                bar.set_position(self.count as u64);
                self.count += (self.min_h - current_h) as f32;
                let percent = self.count / (self.max_h as f32) * 100.0;
                bar.set_message(&format!("{} | open states: {} | closed states: {} | total states: {}",
                    &format!("{:.2}%", percent).magenta(),
                    open_size.to_string().green(),
                    closed_size.to_string().red(),
                    (open_size + closed_size).to_string().cyan()));
                self.min_h = current_h;
            },
            None => ()
        };
    }
}

pub struct Debug
{
    pub parent_count: usize,
    pub child_count: usize
}

impl Debug
{

    pub fn parent(&mut self, mut node: Node, size: usize, open_set: &BinaryHeap<Node>, closed_set: &HashMap<Map, Move>) -> Node
    {
        let container = Container(node.map, size);
        println!("Parent N°{}", self.parent_count);
        println!("{}", container);
        println!("Costs: f({}) g({}) h({})", node.f, node.g, node.h);
        println!("Position: x({}) y({})", node.pos.x, node.pos.y);
        println!("Move: {:?}", node.movement);
        println!("Open set: {} | Closed set: {}", open_set.len(), closed_set.len());
        println!("Open set capacity: {} | Closed set capacity: {}\n", open_set.capacity(), closed_set.capacity());
        self.parent_count += 1;
        self.child_count = 1;
        node.map = container.0;
        node
    }

    pub fn child(&mut self, node: &Node)
    {
        println!("   Child N°{}", self.child_count);
        println!("   Costs: f({}) g({}) h({})", node.f, node.g, node.h);
        println!("   Position: x({}) y({})", node.pos.x, node.pos.y);
        println!("   Move: {:?}\n", node.movement);
        self.child_count += 1;
    }
}

pub struct State
{
	pub map: Map,
	pub movement: Move
}

pub struct Solution
{
	pub path: Vec<State>,
	pub moves: usize,
	pub selected: usize,
	pub pending: usize,
	pub total: usize
}

impl Solution
{
	pub fn new(open_size: usize, closed_size: usize) -> Self
	{
		Self
		{
			path: vec![],
			moves: 0,
			pending: open_size,
			selected: closed_size,
			total: open_size + closed_size
		}
	}

    pub fn display_all(&mut self, size: usize, verbosity: bool, time: Instant)
    {
        if verbosity == true
        {
            while let Some(state) = self.path.pop()
            {
                println!("[{}]", state.movement);
                println!("{}", Container(state.map, size));
            }
            println!("Number of pending states (open set): {}", self.pending.to_string().green());
            println!("Number of selected states (closed set): {}", self.selected.to_string().red());
            println!("Number of states ever represented in memory: {}", self.total.to_string().cyan());
        }
        println!("Number of moves: {}", self.moves.to_string().yellow());
        println!("Execution time: {}", &format!("{:?}", time.elapsed()).bright_blue().bold());
    }
}
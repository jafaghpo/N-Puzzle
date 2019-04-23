use std::collections::HashMap;
use std::cmp::Ordering;

pub type Map = Vec<usize>;

pub type Parsed = (Map, Map, usize);

pub type CostFunc = Box<Fn(usize, usize) -> usize>;

// pub struct Position
// {
// 	x: usize,
// 	y: usize
// }

// pub enum Move
// {
// 	Up,
// 	Down,
// 	Left,
// 	Right,
// 	No
// }

// impl Move
// {
// 	pub is_possible(&self, pos: Position, size: usize) -> bool
// 	{
// 		match self
// 		{

// 		}
// 	}
// }

pub struct Heuristic
{
	pub end: Map,
	pub size: usize,
	pub func: fn(&Map, &Map, usize) -> usize
}

impl Heuristic
{
	pub fn new(end: Map, size: usize, func: fn(&Map, &Map, usize) -> usize) -> Heuristic
	{
		Heuristic
		{
			end: end,
			size: size,
			func: func
		}
	}

	#[inline]
	pub fn call(&self, current: &Map) -> usize
	{
		(self.func)(current, &self.end, self.size)
	}
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node
{
	pub map: Map,
	pub id: String,
	pub parent_id: String,
	pub pos: usize,
	pub h: usize,
	pub g: usize,
	pub f: usize
}

impl Node
{

	pub fn new(map: Map) -> Node
	{
		Self
		{
			map: map,
			id: String::new(),
			parent_id: String::new(),
			pos: 0,
			f: 0,
			g: 0,
			h: <usize>::max_value()
		}
	}

	pub fn create_start(map: Map) -> Node
	{
		let mut node = Node::new(map);
		node.id = format!("{:?}", node.map);
		node.pos = node.map.iter().position(|&x| x == 0).unwrap();
		node
	}

	#[inline]
	pub fn swap_position(&self, new_pos: usize) -> Map
	{
		let mut map = self.map.clone();
		let tmp = map[new_pos];

		map[new_pos] = map[self.pos];
		map[self.pos] = tmp;
		map
	}

	pub fn generate_children(&self, closed_set: &ClosedSet, heuristic: &Heuristic, get_cost: &CostFunc) -> Vec<Self>
	{
		let size = heuristic.size;
		let mut children: Vec<Self> = vec![];
		// println!("size: {}, pos: {}", size, self.pos);

		// Check up move
		if 0 <= (self.pos as i64) - (size as i64)
		{
			// println!("before swap");
			let new_map = self.swap_position(self.pos - size);
			// println!("after swap");
			let mut node = Self::new(new_map);
			node.id = format!("{:?}", node.map);
			
			if !closed_set.contains_key(&node.id)
			{
				node.parent_id = self.id.clone();
				node.pos = self.pos - size;
				node.g = self.g + 1;
				node.h = heuristic.call(&node.map);
				node.f = get_cost(node.h, node.g);
				children.push(node);
			}
		}
		
		// Check down move
		if self.map.len() > self.pos + size
		{
			let new_map = self.swap_position(self.pos + size);
			let mut node = Self::new(new_map);
			node.id = format!("{:?}", node.map);
			
			if !closed_set.contains_key(&node.id)
			{
				node.parent_id = self.id.clone();
				node.pos = self.pos + size;
				node.g = self.g + 1;
				node.h = heuristic.call(&node.map);
				node.f = get_cost(node.h, node.g);
				children.push(node);
			}
		}

		// Check left move
		if self.pos % size != 0
		{
			let new_map = self.swap_position(self.pos - 1);
			let mut node = Self::new(new_map);
			node.id = format!("{:?}", node.map);
			
			if !closed_set.contains_key(&node.id)
			{
				node.parent_id = self.id.clone();
				node.pos = self.pos - 1;
				node.g = self.g + 1;
				node.h = heuristic.call(&node.map);
				node.f = get_cost(node.h, node.g);
				children.push(node);
			}
		}

		// Check right move
		if ((self.pos % size) as i64) != (size as i64) - 1
		{
			// println!("before swap");
			let new_map = self.swap_position(self.pos + 1);
			// println!("after swap");
			let mut node = Self::new(new_map);
			node.id = format!("{:?}", node.map);
			
			if !closed_set.contains_key(&node.id)
			{
				node.parent_id = self.id.clone();
				node.pos = self.pos + 1;
				node.g = self.g + 1;
				node.h = heuristic.call(&node.map);
				node.f = get_cost(node.h, node.g);
				children.push(node);
			}
		}
		children
	}
}

impl PartialOrd for Node
{
    fn partial_cmp(&self, other: &Node) -> Option<Ordering>
	{
        Some(self.cmp(other))
    }
}

impl Ord for Node
{
    fn cmp(&self, other: &Node) -> Ordering
	{
        other.f.cmp(&self.f)
    }
}

pub type ClosedSet = HashMap<String, String>;

pub struct Solution
{
	pub path: Vec<String>,
	pub moves: usize,
	pub selected_nodes: usize,
	pub total_nodes: usize,
}
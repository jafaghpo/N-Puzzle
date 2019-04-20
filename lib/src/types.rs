use std::collections::HashMap;

pub type Map = Vec<usize>;

pub struct Parsed
{
	pub start: Map,
	pub end: Map,
	pub size: usize
}

pub type CostFunc = Box<Fn(usize, usize) -> usize>;

pub struct Cost
{
	pub f: usize,
	pub g: usize,
	pub h: usize
}

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

	pub fn call(&self, current: &Map) -> usize
	{
		(self.func)(current, &self.end, self.size)
	}
}

pub struct Node
{
	pub map: Map,
	pub id: String,
	pub parent_id: String,
	pub pos: usize,
	pub cost: Cost
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
			cost: Cost {f: 0, g: 0, h: <usize>::max_value() }
		}
	}

	pub fn create_start(map: Map) -> Node
	{
		let mut node = Node::new(map);
		node.id = format!("{:?}", node.map);
		node.pos = node.map.iter().position(|&x| x == 0).unwrap();
		node
	}

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
				node.cost.g = self.cost.g + 1;
				node.cost.h = heuristic.call(&node.map);
				node.cost.f = get_cost(node.cost.h, node.cost.g);
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
				node.cost.g = self.cost.g + 1;
				node.cost.h = heuristic.call(&node.map);
				node.cost.f = get_cost(node.cost.h, node.cost.g);
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
				node.cost.g = self.cost.g + 1;
				node.cost.h = heuristic.call(&node.map);
				node.cost.f = get_cost(node.cost.h, node.cost.g);
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
				node.cost.g = self.cost.g + 1;
				node.cost.h = heuristic.call(&node.map);
				node.cost.f = get_cost(node.cost.h, node.cost.g);
				children.push(node);
			}
		}
		children
	}
}

pub type ClosedSet = HashMap<String, String>;

pub struct OpenSet
{
    pub list: Vec<Node>
}

impl OpenSet 
{
	pub fn new(start_node: Node) -> OpenSet
	{
		OpenSet { list: vec![start_node] }
	}

    pub fn insert(&mut self, to_insert: Node)
    {
    	match self.list.iter().position(|node| node.cost.f <= to_insert.cost.f)
    	{
    		Some(index) => self.list.insert(index, to_insert),
    		None => self.list.push(to_insert)
    	}
    }

	pub fn insert_list(&mut self, list: Vec<Node>)
	{
		for node in list { self.insert(node) }
	}

	pub fn pop_lowest(&mut self) -> Node
	{
		self.list.pop().unwrap()
	}
}

pub struct Solution
{
	pub moves: usize,
	pub path: Vec<String>,
	pub selected_nodes: usize,
	pub total_nodes: usize,
}
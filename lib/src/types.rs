pub type Map = Vec<usize>;

pub struct Node
{
	pub map: Map,
	pub id: String,
	pub pos: usize,
	pub f: usize,
	pub g: usize,
	pub h: usize,
}

// impl Node
// {
// 	fn new(map: Map, _size: usize, gcost: usize) -> Node
// 	{
// 		let mut node = Node
// 		{
// 			map: map,
// 			id: String::new(),
// 			pos: 0,
// 			f: 0,
// 			g: gcost,
// 			h: 0
// 		};
// 		node.id = node.get_id();
// 		node
// 	}

// 	fn get_id(&self) -> String
// 	{
// 		format!("{:?}", self.map)
// 	}
// }



// pub struct OpenSet
// {
//     list: Vec<Node>
// }

// impl OpenSet 
// {
// 	pub fn new() -> OpenSet
// 	{
// 		OpenSet { list: vec![] }
// 	}

//     pub fn insert(&mut self, to_insert : Node) 
//     {
//     	match self.list.iter().position(|node| node.g <= to_insert.g)
//     	{
//     		Some(index) => self.list.insert(index, to_insert),
//     		None => self.list.push(to_insert)
//     	}
    	
//     }
// }

// fn main() 
// {
//     let node1 = Node::new(vec![7,8,4,5,1,3,6,2,0], 3, 5);
//     let node2 = Node::new(vec![7,8,4,5,1,3,6,2,0], 3, 3);
//     let node3 = Node::new(vec![7,8,4,5,1,3,6,2,0], 3, 3);
//     let node4 = Node::new(vec![7,8,4,5,1,3,6,2,0], 3, 2);

//     let to_insert = Node::new(vec![0,0,0,0,0,0,0,0,0], 3, 3);

//     let mut open_set = OpenSet::new();
//     open_set.list.push(node1);
//     open_set.list.push(node2);
//     open_set.list.push(node3);
//     open_set.list.push(node4);
//     open_set.insert(to_insert);
//     open_set.list.iter().for_each(|node| println!("{} , {:?}", node.g, node.map));
// }

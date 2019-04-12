// - (MT) For close set, a hash function to identify state with formulae
// 	=> for state 'S' of length 'N' as [7, 3, 2, ..., X] and hash 'H' as 0
// 	=> for i=0...N -> H = (X * H) + S[i]
pub type Map = Vec<usize>;

pub struct Node
{
	pub map: Map,
	pub id: usize,
	pub pos: usize,
	pub f: usize,
	pub g: usize,
	pub h: usize,
}

impl Node
{
	fn new(map: Map, size: usize, gcost: usize) -> Node
	{
		let mut node = Node
		{
			map: map,
			id: 0,
			pos: 0,
			f: 0,
			g: gcost,
			h: 0
		};
		node.id = node.get_id(size);

	}
	fn get_id(&self, size: size) -> usize
	{
		(0...size).fold(0, |sum, i| self.map[size] * sum + map[i])
	}
}



pub struct OpenSet
{
    list: Vec<Node>
}

impl OpenSet 
{
    pub fn insert(&mut self, to_insert : Node) 
    {
    	let index = self.list.iter().position(|&node| node.f <= to_insert.h);
    	self.list.insert(index, to_insert); 
    }
}

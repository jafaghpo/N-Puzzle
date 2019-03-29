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
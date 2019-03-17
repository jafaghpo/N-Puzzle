pub type Tile = usize;
pub type Puzzle = Vec<Tile>;

pub struct State
{
	pub puzzle: Puzzle,
	pub id: u64,
	pub pos: usize,
	pub f: u64,
	pub g: u64,
	pub h: u64,
}
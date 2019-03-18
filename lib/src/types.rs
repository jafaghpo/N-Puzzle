pub type Puzzle = Vec<usize>;

pub struct State
{
	pub puzzle: Puzzle,
	pub id: usize,
	pub pos: usize,
	pub f: usize,
	pub g: usize,
	pub h: usize,
}
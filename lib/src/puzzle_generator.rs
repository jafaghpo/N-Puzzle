extern crate rand;

use crate::generator::classic;
use rand::Rng;

pub struct Puzzle
{
	map: Vec<usize>,
	empty: usize
}

fn shuffle_puzzle (mut puzzle: Puzzle, interations: usize, size: usize) -> Puzzle
{
	for _ in 0..interations
	{
		puzzle = random_swap(puzzle, size);
	}
	return puzzle;
}

fn random_swap(mut puzzle: Puzzle, size: usize) -> Puzzle
{

	let mut moves: Vec<usize> = vec![];
	// ⬇️
	if puzzle.empty > size
	{
		moves.push(puzzle.empty - size);
	}
	// ➡️
	if puzzle.empty % size != 0
	{
		moves.push(puzzle.empty - 1);
	}
	// ⬅️
	if puzzle.empty % size != size - 1
	{
		moves.push(puzzle.empty + 1);
	}
	// ⬆️
	if puzzle.empty < (size * size) - size
	{
		moves.push(puzzle.empty + size);
	}
	//println!("moves : {:?}", moves);

	puzzle = swap_tiles(puzzle, *rand::thread_rng().choose(&moves).unwrap());
	return puzzle;

}

fn swap_tiles(mut puzzle: Puzzle, tile: usize) -> Puzzle
{
	//println!("tile :{}, empty: {}", tile, puzzle.empty);
	let tmp: usize = puzzle.map[puzzle.empty];
	puzzle.map[puzzle.empty] = puzzle.map[tile];
	puzzle.map[tile] = tmp;
	puzzle.empty = tile;
	return puzzle; 
}

pub fn get_iterations(difficulty: &str) -> usize
{
	let iterations: usize = match difficulty
	{
		"easy" => 15,
		"normal" => 50,
		"hard" => 140,
		_ => 1

	};
	iterations
}

pub fn puzzle_to_str(puzzle: Vec<usize>, size: usize) -> String
{
	let mut result: String = String::new();
	result.push_str(&format!("{}", size));
	for i in 0..(size * size)
	{
		if i % size == 0 { result.push_str("\n") }
		result.push_str(&format!("{}\t", puzzle[i]));
	}
	result.push_str("\n");
	result
}

pub fn print_puzzle(puzzle: Vec<usize>, size: usize)
{
	for i in 0..(size * size)
	{
		if i % size == 0 { println!("") }
		print!("{}\t", puzzle[i]);
	}
}

pub fn generate_puzzle(size: usize, iterations: usize) -> Vec<usize>
{
	let mut puzzle = Puzzle
	{
		map: classic(size),
		empty: size.pow(2) - 1
	}; 
	puzzle = shuffle_puzzle(puzzle, iterations, size);
	puzzle.map
}
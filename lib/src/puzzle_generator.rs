extern crate rand;

use rand::thread_rng;
use rand::seq::SliceRandom;

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
	let mut rng = thread_rng();
	puzzle = swap_tiles(puzzle, *moves.choose(&mut rng).unwrap());
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
		"normal" => 51,
		"hard" => 141,
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

pub fn print_puzzle(puzzle: &Vec<usize>, size: usize)
{
	for i in 0..(size * size)
	{
		if i % size == 0 { println!("") }
		print!("{}\t", puzzle[i]);
	}
}

pub fn generate_puzzle(size: usize, iterations: usize, f: fn(usize)->Vec<usize>) -> Vec<usize>
{
	let mut puzzle = Puzzle
	{
		map: f(size),
		empty: 0
	};
	puzzle.empty = puzzle.map.iter().position(|&x| x == 0).unwrap();
	puzzle = shuffle_puzzle(puzzle, iterations, size);
	puzzle.map
}
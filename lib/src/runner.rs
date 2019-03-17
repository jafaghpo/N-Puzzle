use std::fs;
use crate::checker;
use crate::generator;
use crate::types::Puzzle;

pub fn run_program(filename: &str, goal_mode: &str, _algo: &str, _heuristic: &str) -> Result<String, String>
{
	// Open an copy the file content into a string
    let file: String;
	match fs::read_to_string(filename)
	{
		Ok(f) => file = f,
		Err(_) => return Err(format!("invalid file path"))
	}

	if file.is_empty() { return Err(format!("file is empty")) }

	// Filter comments and empty lines from file
	let mut lines: Vec<&str> = file
		.lines()
		.map(|line| line.split("#").nth(0).unwrap())
		.filter(|&line| line != "")
		.map(|line| line.trim())
		.collect();

	// Get size and check if size is valid
	let size: usize;
	match lines.remove(0).parse()
	{
		Ok(s) if s >= 3 && s <= 100 => size = s,
		Err(_) => return Err(format!("invalid puzzle size")),
		_ => return Err(format!("size not in range, must be between 3 and 100 to be valid"))
	}

	// Divide each lines into words
	let lines: Vec<Vec<&str>> = lines
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();
	
	// Check puzzle validity and store the tiles in a vector
	let mut puzzle: Puzzle;
	match checker::check_puzzle(lines, size)
	{
		Ok(p) => puzzle = p,
		Err(e) => return Err(e)
	}

	// Generate goal state depending on style (snail, ascending or descending)
	let goal: Puzzle = match goal_mode
	{
		"classic" => generator::classic(size),
		"reversed" => generator::reversed(size),
		"snail" | _ => generator::snail(size)
	};

	if checker::is_solvable(&puzzle, &goal, size) == false
	{
		return Err(format!("unsolvable puzzle"));
	}

	// DEBUG
	// println!("size: {}", size);
	// println!("puzzle: {:?}", puzzle);
	// for n in 0..(size * size)
	// {
	// 	if n % size == 0 { println!("") }
	// 	print!("{}\t", goal[n]);
	// }
	// println!("");
	Ok(format!("End of run_program"))
}
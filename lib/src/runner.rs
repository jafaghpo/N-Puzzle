use std::fs;
use crate::checker;
use crate::goal_generator::{classic, snail, reversed};
use crate::types::Map;

fn get_file_content(filename: &str) -> Result<String, String>
{
	match fs::read_to_string(filename)
	{
		Ok(file) =>
		{
			match file.is_empty()
			{
				true => Err(format!("file is empty")),
				false => Ok(file)
			}
		}
		Err(_) => Err(format!("invalid file path"))
	}
}

fn file_to_map(file: String) -> Result<(usize, Vec<usize>), String>
{
	// Filter comments and empty lines
	let mut lines: Vec<&str> = file
		.lines()
		.map(|line| line.split("#").nth(0).unwrap())
		.filter(|&line| line != "")
		.map(|line| line.trim())
		.collect();

	// Get size and check if size is valid
	let size = match lines.remove(0).parse()
	{
		Ok(s) =>
		{
			match (s >= 3, s <= 10)
			{
				(false, true) => Err(format!("size is too small, must be at least 3")),
				(true, false) => Err(format!("size is too big, must be equal or below 10")),
				(true, true) | _ => Ok(s)
			}
		}
		Err(_) => Err(format!("invalid puzzle size")),
	}?;

	// Divide each lines into words
	let lines: Vec<Vec<&str>> = lines
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();
	// Check if the number of rows and colums is equal to the puzzle size
	if lines.len() != size { return Err(format!("invalid number of rows")) }
	if lines.iter().any(|line| line.len() != size) { return Err(format!("invalid number of columns")) }
	
	// Check if the tiles are positive numbers and are in range
	let mut map: Vec<usize> = vec![];
	for line in lines
	{
		for number in line
		{
			match number.parse()
			{
				Ok(n) if (n < size * size) => { map.push(n) },
				Err(_) => { return Err(format!("invalid tile number: '{}'", number)) },
				_ => { return Err(format!("tile number not in range: '{}'", number)) }
			}
		}
	}

	// Check if there is duplicate numbers
	let mut number_list: Vec<bool> = vec![false; size * size];
	for number in &map
	{
		match number_list[*number]
		{
			true =>	{ return Err(format!("duplicate tile of '{}'", number)) },
			false => { number_list[*number] = true }
		}
	}
	Ok((size, map))
}

pub fn run_program(filename: &str, goal_mode: &str, _algo: &str, _heuristic: &str) -> Result<String, String>
{
	// Open an copy the file content into a string
    let file = get_file_content(filename)?;

	let (size, map) = file_to_map(file)?;

	// Generate end node depending on style (snail, ascending or descending)
	let end: Map = match goal_mode
	{
		"classic" => classic(size),
		"reversed" => reversed(size),
		"snail" | _ => snail(size)
	};

	if checker::is_solvable(&map, &end, size) == false
	{
		return Err(format!("unsolvable puzzle"));
	}

	// Swapped 
	let _end = end
		.iter()
		.enumerate()
		.fold(vec![0; size * size], | mut acc, (i, x) | { acc[*x] = i; acc } );

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
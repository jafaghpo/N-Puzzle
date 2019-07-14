use std::fs;

use crate::Container;

// Open an copy the file content into a string
fn get_file_content(filename: &str) -> Result<String, String>
{
	let max_size = 10000000;

	let metadata = match fs::metadata(filename)
	{
		Ok(m) => Ok(m),
		Err(_) => Err(format!("'{}' doesn't exist", filename))
	}?;

	if metadata.is_file() == false { return Err(format!("'{}' is not a file", filename)) }
	match metadata.len()
	{
		size if size >= max_size => return Err(format!("'{}' is over 10MB and thus cannot be accepted", filename)),
		0 => return Err(format!("'{}' has a size of 0", filename)),
		_ => ()
	};

	match fs::read_to_string(filename)
	{
		Ok(file) => Ok(file),
		Err(_) => Err(format!("unable to read '{}'", filename))
	}
}

fn file_to_map(file: String) -> Result<(usize, Vec<usize>), String>
{
	// Filter comments and empty lines
	let mut lines: Vec<&str> = file
		.lines()
		.map(|line| line.split("#").nth(0).unwrap())
		.map(|line| line.split("//").nth(0).unwrap())
		.filter(|&line| line != "")
		.map(|line| line.trim())
		.collect();

	// Get size and check if size is valid
	let size = match lines.remove(0).parse()
	{
		Ok(s) =>
		{
			match (s >= 3, s <= 20)
			{
				(false, true) => Err(format!("size is too small, must be at least 3")),
				(true, false) => Err(format!("size is too big, must be equal or below 20")),
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

pub fn get_map(filename: &str) -> Result<Container, String>
{
    let file = get_file_content(filename)?;

	let (size, start) = file_to_map(file)?;

	Ok(Container(start, size))
}
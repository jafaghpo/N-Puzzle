// Check the puzzle content and returns a vector containing all the tiles
pub fn check_puzzle(lines: Vec<Vec<&str>>, size: usize) -> Result<Vec<usize>, String>
{
	// Check if the number of rows and colums is equal to the puzzle size
	if lines.len() != size { return Err(format!("invalid number of rows")) }
	if lines.iter().any(|line| line.len() != size) { return Err(format!("invalid number of colums")) }

	// Check if the tiles are positive numbers and are in range
	let mut tiles: Vec<usize> = vec![];
	for line in lines
	{
		for number in line
		{
			match number.parse()
			{
				Ok(n) if (n < size * size) => { tiles.push(n) },
				Err(_) => { return Err(format!("invalid number '{}'", number)) },
				_ => { return Err(format!("number not in range '{}'", number)) }
			}
		}
	}

	// Check if there is duplicate numbers
	let mut number_list: Vec<bool> = vec![false; size * size];
	for number in &tiles
	{
		match number_list[*number]
		{
			true =>	{ return Err(format!("duplicate of '{}'", number)) },
			false => { number_list[*number] = true }
		}
	}
	return Ok(tiles);
}
/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jafaghpo <jafaghpo@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/03/13 15:45:44 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/14 19:03:15 by jafaghpo         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
use clap::App;
use std::fs;
use std::process::exit;

// TODO:
// Check if the puzzle is solvable 

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("error: {}", message);
	exit(1);
}

// Check the puzzle content and returns a vector containing all the tiles
fn check_puzzle(lines: Vec<Vec<&str>>, size: usize) -> Result<Vec<usize>, String>
{
	// Check if the number of rows and colums is equal to the puzzle size
	if lines.len() != size { return Err("invalid number of rows".to_string()); }
	if !lines.iter().all(|line| line.len() == size) { return Err("invalid number of colums".to_string()); }

	// Check if the tiles are positive numbers and are in range
	let mut tiles: Vec<usize> = vec![];
	for line in lines
	{
		for number in line
		{
			let n: String = number.to_string();
			match number.parse()
			{
				Ok(n) if n <= size.pow(2) - 1 => { tiles.push(n) },
				Err(_) => { return Err(format!("invalid number '{}'", n)) },
				_ => { return Err(format!("number not in range '{}'", n)); }
			}
		}
	}

	// Check if there is duplicate numbers
	let mut number_list: Vec<bool> = vec![false; size.pow(2)];
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

fn main() 
{
	// Command-line argument parser
	let yaml = load_yaml!("man.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();
	let _goal = matches.value_of("goal_state").unwrap();
	let _algo = matches.value_of("algorithm").unwrap();
	let _heuristic = matches.value_of("heuristic_function").unwrap();

	// File reader
    let mut file = String::new();
	match fs::read_to_string(filename)
	{
		Ok(f)	=> file = f,
		Err(_)	=> exit_program("invalid file path")
	}

	// Filter comments and empty lines from file
	let mut lines: Vec<&str> = file
		.lines()
		.map(|line| line.split("#").nth(0).unwrap())
		.filter(|&line| line != "")
		.collect();

	// Get size and check if size is valid
	let mut size = 0usize;
	match lines.remove(0).parse()
	{
		Ok(s) if s >= 2	=> size = s,
		Err(_) | _		=> exit_program("invalid puzzle size")
	}

	// Divide each lines into words
	let lines: Vec<Vec<&str>> = lines
		.iter()
		.map(|line| line.split_whitespace().collect())
		.collect();
	
	// Check puzzle validity and store the tiles in a vector
	let mut tiles: Vec<usize> = vec![];
	match check_puzzle(lines, size)
	{
		Ok(s) => tiles = s,
		Err(e) => exit_program(&e)
	}

	println!("size: {}", size);
	println!("tiles: {:?}", tiles);
}

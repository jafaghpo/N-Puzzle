/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: john <john@student.42.fr>                  +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/03/13 15:45:44 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/15 13:25:11 by john             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
use clap::App;
use std::fs;
use std::process::exit;

mod checker;
use checker::{check_puzzle};

// TODO:
// Generate goal state depending on goal mode
// Check if the puzzle is solvable 

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("error: {}", message);
	exit(1);
}

fn main()
{
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();
	let _goal = matches.value_of("goal_style").unwrap();
	let _algo = matches.value_of("algorithm").unwrap();
	let _heuristic = matches.value_of("heuristic_function").unwrap();

	// Open an copy the file content into a string
    let mut file = String::new();
	match fs::read_to_string(filename)
	{
		Ok(f) => file = f,
		Err(_) => exit_program("invalid file path")
	}

	if file.is_empty() { exit_program("file is empty") }

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
		Ok(s) if (s >= 3 && s <= 100) => size = s,
		Err(_) | _ => exit_program("invalid puzzle size")
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

	// DEBUG
	println!("size: {}", size);
	println!("tiles: {:?}", tiles);
}

extern crate colored;

use npuzzle_lib::*;

use clap::{App, load_yaml};
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use runner;
use goal_generator::{classic, snail, reversed};
use colored::*;
use puzzle_generator::{generate_puzzle, get_iterations, puzzle_to_str};

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("error: {}", message.red());
	exit(1);
}

fn display_result(message: String)
{
	println!("{}", message);
}

fn main()
{
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();
	let goal_mode = matches.value_of("goal_mode").unwrap();
	let algo = matches.value_of("algorithm").unwrap();
	let heuristic = matches.value_of("heuristic_function").unwrap();
	let generator_size = matches.value_of("generator").unwrap();
	let difficulty = matches.value_of("difficulty").unwrap();
	let iterations = matches.value_of("iterations");
	if generator_size != "None"
	{
		let goal_function = match goal_mode 
		{
			"classic" => classic,
			"reversed" => reversed,
			"snail" | _ => snail
		};
		let size: usize = match generator_size.parse()
		{
			Ok(n) => n,
			Err(e) => { exit_program(&e.to_string()); 0 }
		};
		let puzzle = match iterations
		{
			None => generate_puzzle(size, get_iterations(difficulty), goal_function),
			_ =>
			{
				let iter: usize = match iterations.unwrap().parse()
				{
					Ok(n) => n,
					Err(e) => { exit_program(&e.to_string()); 0 }
				};
				generate_puzzle(size, iter, goal_function)
			}
		};
		let name = match iterations
		{
			None => format!("{}/{}_{}_{3}x{3}", filename, goal_mode, difficulty, size.to_string()),
			_ => format!("{}/{}_{}_{3}x{3}", filename, goal_mode, iterations.unwrap().to_string(), size.to_string())
		};
		let mut file : File;
		match File::create(name)
		{
			Ok(n) => file = n,
			Err(e) => { return exit_program(&e.to_string()) }
		};
		match file.write_all(puzzle_to_str(puzzle, size).as_bytes())
		{
			Ok(_) => (),
			Err(e) => exit_program(&e.to_string())
		};
	}
	else 
	{
		match runner::run_program(filename, goal_mode, algo, heuristic)
		{
			Ok(result) => display_result(result),
			Err(e) => exit_program(&e)
		}
	}
}

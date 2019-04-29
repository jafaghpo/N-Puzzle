extern crate colored;

use clap::{App, load_yaml};
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use colored::*;
use std::time::{Instant};

use npuzzle_lib::*;
use types::{Flag, Map, Solver, Parsed};
use parser;
use algorithm;
use goal_generator::{classic, snail, reversed};
use puzzle_generator::{generate_puzzle, get_iterations, puzzle_to_str};

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("error: {}", message.red());
	exit(1);
}

fn create_generated_puzzle(dirpath: &str, size: &str, level: &str, end_mode: &str) -> Result<String, String>
{
	fn parse_number(number: &str) -> Result<usize, String>
	{
		match number.parse()
		{
			Ok(n) => Ok(n),
			Err(_) => Err(format!("'{}' must be a valid number", number))
		}
	}

	fn create_file(filepath: &str, puzzle: Vec<usize>, size: usize) -> Result<(), String>
	{
		let mut file = match File::create(filepath)
		{
			Ok(f) => Ok(f),
			Err(e) => Err(e.to_string())
		}?;

		if let Err(e) = file.write_all(puzzle_to_str(puzzle, size).as_bytes())
		{
			return Err(e.to_string())
		};
		Ok(())
	}

	let mode = match end_mode 
	{
		"classic" => classic,
		"reversed" => reversed,
		"snail" | _ => snail
	};

	let size = parse_number(size)?;

	let puzzle = match level
	{
		"easy" | "normal" | "hard" | "epic" => generate_puzzle(size, get_iterations(level), mode),
		_ => generate_puzzle(size, parse_number(level)?, mode)
	};

	let filepath = match level
	{
		"easy" | "normal" | "hard" => format!("{}/{}_{}_{3}x{3}", dirpath, end_mode, level, size),
		iter => format!("{}/{}_{}_{3}x{3}", dirpath, end_mode, iter, size)
	};

	create_file(&filepath, puzzle, size)?;
	Ok(filepath)
}

// Swap indexes of a vector with their respective values
pub fn swap_indexes(vec: &Map) -> Map
{
	vec
		.iter()
		.enumerate()
		.fold(vec![0; vec.len()], | mut acc, (i, x) | { acc[*x] = i; acc } )
}

fn main()
{
	let start_time = Instant::now();
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();
	let end_mode = matches.value_of("end_mode").unwrap();
	let generator_size = matches.value_of("generator").unwrap();
	let level = matches.value_of("level").unwrap();
	let iterations = matches.value_of("iterations");
	let heuristic = matches.value_of("heuristic_function").unwrap();
	let algo = matches.value_of("algorithm").unwrap();

	let file = if generator_size == "None" { filename.to_owned() } else
	{
		let result = match iterations
		{
			Some(iter) => create_generated_puzzle(filename, generator_size, iter, end_mode),
			None => create_generated_puzzle(filename, generator_size, level, end_mode),
		};
		match result
		{
			Ok(res) => res,
			Err(e) => { exit_program(&e); String::new() }
		}
	};

	// Get start map, end map & map size inside parsed
	let parsed: Result<Parsed, String> = parser::get_map(&file, end_mode);
	if let Err(e) = &parsed { exit_program(&e) }
	let (start, end, size) = parsed.unwrap();

	let solver = Solver::new(swap_indexes(&end), end, size, heuristic, algo);
	let flag = Flag
	{
		display_bar: if solver.uniform { false } else { matches.is_present("bar") },
		verbosity: matches.is_present("verbosity"),
		debug: matches.is_present("debug")
	};

	algorithm::solve(start, size, solver, &flag, start_time);
}

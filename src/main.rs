extern crate colored;

use clap::{App, load_yaml};
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use colored::*;
use std::time::{Instant};

use npuzzle_lib::*;
use types::{Flag, Map, Solver, Parsed, Move, State};
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

fn display_map(map: &Map, size: usize)
{
	for i in 0..(size * size)
	{
		if i != 0 && i % size == 0 { println!("\n") }
		print!(" {}\t", map[i]);
	}
	println!("\n\n------------------------------------\n");
}

fn display_path(mut path: Vec<State>, size: usize)
{
	while let Some(state) = path.pop()
	{
		match state.movement
		{
			Move::Left(_) => println!("[Left]"),
			Move::Right(_) => println!("[Right]"),
			Move::Up(_) => println!("[Up]"),
			Move::Down(_) => println!("[Down]"),
			Move::No => println!("[Start State]"),
		};
		display_map(&state.map, size);
	}
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
	let level = matches.value_of("difficulty").unwrap();
	let iterations = matches.value_of("iterations");
	let heuristic = matches.value_of("heuristic_function").unwrap();
	let algo = matches.value_of("algorithm").unwrap();

	let flag = Flag
	{
		mem_limit: matches.is_present("memory"),
		display_bar: matches.is_present("bar"),
		verbosity: matches.is_present("verbosity")
	};

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

	let solver = Solver::new(end, size, heuristic, algo);
	let solution = algorithm::solve(start, size, &solver, &flag);

	if flag.verbosity { display_path(solution.path, size) }
	println!("Number of moves: {}", solution.moves);
	println!("Number of pending states (open set): {}", solution.pending);
	println!("Number of selected states (closed set): {}", solution.selected);
	println!("Number of states ever represented in memory: {}", solution.total);
	println!("Execution time: {:?}", start_time.elapsed());
}

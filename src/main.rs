use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use colored::*;
use clap::{App, load_yaml};
use std::time::{Instant};

use npuzzle::{Flag, Args, Parsed};
use npuzzle::solver::Solver;
use npuzzle::parser;
use npuzzle::algorithm;
use npuzzle::goal::{classic, snail, reversed};
use npuzzle::generator::{generate_puzzle, get_iterations, puzzle_to_str};

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("Error: {}", message.red());
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

fn run_program(args: Args, time: Instant) -> Result<(), String>
{
	let file: String = if args.g_size == "None" { args.file } else
	{
		match args.iter
		{
			Some(iter) => create_generated_puzzle(&args.file, &args.g_size, &iter, &args.goal),
			None => create_generated_puzzle(&args.file, &args.g_size, &args.level, &args.goal),
		}?
	};

	// Get start map, end map & map size inside parsed
	let parsed: Parsed = parser::get_map(&file, &args.goal)?;
	let (start, end, size) = parsed;

	let solver = Solver::new(end, size, &args.heuristic, args.flag, time);
	solver.is_solvable(&start)?;

	algorithm::solve(start, size, solver);
	Ok(())
}

fn main()
{
	let time = Instant::now();
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let algo = matches.value_of("algorithm").unwrap();

	let args = Args
	{
		file: matches.value_of("file").unwrap().to_owned(),
		goal: matches.value_of("end_mode").unwrap().to_owned(),
		g_size: matches.value_of("generator").unwrap().to_owned(),
		level: matches.value_of("level").unwrap().to_owned(),
		iter: match matches.value_of("iterations")
		{
			Some(i) => Some(i.to_owned()),
			None => None
		},
		heuristic: matches.value_of("heuristic_function").unwrap().to_owned(),
		flag: Flag
		{
			verbosity: matches.is_present("verbosity"),
			debug: matches.is_present("debug"),
			greedy: algo == "greedy",
			uniform: algo == "uniform"
		}
	};
	if let Err(ref message) = run_program(args, time)
	{
		exit_program(message);
	}
	
}

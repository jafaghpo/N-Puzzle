extern crate colored;

use npuzzle_lib::*;

use clap::{App, load_yaml};
use std::process::exit;
use std::fs::File;
use std::io::prelude::*;
use parser;
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
		"easy" | "normal" | "hard" => generate_puzzle(size, get_iterations(level), mode),
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

fn main()
{
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();
	let end_mode = matches.value_of("end_mode").unwrap();
	let algo = matches.value_of("algorithm").unwrap();
	let heuristic = matches.value_of("heuristic_function").unwrap();
	let generator_size = matches.value_of("generator").unwrap();
	let difficulty = matches.value_of("difficulty").unwrap();
	let iterations = matches.value_of("iterations");

	let mode = match end_mode 
	{
		"classic" => classic,
		"reversed" => reversed,
		"snail" | _ => snail
	};

	if generator_size != "None"
	{

		let size: usize;
		match generator_size.parse()
		{
			Ok(n) => size = n,
			Err(_) => { return exit_program(&format!("'{}' must be a valid number", generator_size)) }
		};

		let puzzle = match iterations
		{
			None => generate_puzzle(size, get_iterations(difficulty), mode),
			Some(iter) =>
			{
				match iter.parse()
				{
					Ok(n) => generate_puzzle(size, n, mode),
					Err(_) => { return exit_program(&format!("'{}' must be a valid number", iter)) }
				}
			}
		};

		let name = match iterations
		{
			None => format!("{}/{}_{}_{3}x{3}", filename, end_mode, difficulty, size.to_owned()),
			Some(iter) => format!("{}/{}_{}_{3}x{3}", filename, end_mode, iter.to_owned(), size.to_owned())
		};


		let mut file: File;
		match File::create(name)
		{
			Ok(n) => file = n,
			Err(e) => { return exit_program(&e.to_string()) }
		};

		if let Err(e) = file.write_all(puzzle_to_str(puzzle, size).as_bytes())
		{
			exit_program(&e.to_string());
		};
	}

	// match parser::get_map(filename, end_mode)
	// {
	// 	Ok(result) => display_result(result),
	// 	Err(e) => exit_program(&e)
	// };
}

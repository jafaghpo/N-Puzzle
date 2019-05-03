use std::process::exit;
use colored::*;
use clap::{App, load_yaml};
use std::time::{Instant};

use npuzzle::{Flag, Container};
use npuzzle::solver::Solver;
use npuzzle::generator::Generator;
use npuzzle::parser;
use npuzzle::{astar, astar_iterative, astar_genetic};

struct Args
{
	pub file: String,
	pub goal: String,
	pub g_size: String,
	pub level: String,
	pub iter: Option<String>,
	pub algo: String,
	pub heuristic: String,
	pub solvable: bool,
	pub flag: Flag
}

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("Error: {}", message.red());
	exit(1);
}

fn parse_number(number: &str) -> Result<usize, String>
{
	match number.parse()
	{
		Ok(n) => Ok(n),
		Err(_) => Err(format!("'{}' must be a valid number", number))
	}
}

fn run_program(args: Args, time: Instant) -> Result<(), String>
{
	let file = if args.g_size == "None" { args.file } else
	{
		let g_size = parse_number(&args.g_size)?;
		if g_size < 3 || g_size > 1000
		{
			return Err("generated puzzle size must be between 3 and 1000".to_owned());
		}
		let iter = match args.iter
		{
			Some(i) => Some(parse_number(&i)?),
			None => None
		};
		let mut generator = Generator::new(g_size, iter, &args.level, &args.goal, &args.file);
		generator.generate_map(&args.goal, args.solvable)?
	};

	// Get start map & size inside Container
	let Container(start, size) = parser::get_map(&file)?;
	let end = Generator::generate_goal(&args.goal, size);
	if start == end { return Err("the puzzle is already solved...".to_owned()) }
	let solver = Solver::new(end, size, &args.heuristic, args.flag, time);
	solver.is_solvable(&start)?;

	match args.algo.as_ref()
	{
		"IDA*" => astar_genetic::solve(start, solver),
		"IA*" => astar_iterative::solve(start, solver),
		"GA*" => astar_genetic::solve(start, solver),
		"A*" | _ => astar::solve(start, solver),
	}
}

fn main()
{
	let time = Instant::now();
	// Read syntax from cli.yml (Command Line Interpretor)
	// parse the command line arguments and return the matches
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();

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
		solvable: matches.value_of("solvability").unwrap() == "solvable",
		algo: matches.value_of("algorithm").unwrap().to_owned(),
		heuristic: matches.value_of("heuristic_function").unwrap().to_owned(),
		flag: Flag
		{
			verbosity: matches.is_present("verbosity"),
			debug: matches.is_present("debug"),
			greedy: matches.is_present("greedy"),
			uniform: matches.is_present("uniform")
		}
	};
	if let Err(ref message) = run_program(args, time)
	{
		exit_program(message);
	}
}

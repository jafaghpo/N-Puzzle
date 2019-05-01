use std::process::exit;
use colored::*;
use clap::{App, load_yaml};
use std::time::{Instant};

use npuzzle::{Flag, Args, Container};
use npuzzle::solver::Solver;
use npuzzle::generator::Generator;
use npuzzle::parser;
use npuzzle::algorithm;

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
		let iter = match args.iter
		{
			Some(i) => Some(parse_number(&i)?),
			None => None
		};
		let generator = Generator::new(g_size, iter, &args.level, &args.goal, &args.file);
		generator.generate_map(&args.goal)?
	};

	// Get start map & size inside Container
	let Container(start, size) = parser::get_map(&file)?;
	let end = Generator::generate_goal(&args.goal, size);
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

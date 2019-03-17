/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: john <john@student.42.fr>                  +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/03/13 15:45:44 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/16 18:36:42 by john             ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

use npuzzle_lib::*;

use clap::{App, load_yaml};
use std::process::exit;
use runner;

// Display error message on standard error and exit program
fn exit_program(message: &str)
{
	eprintln!("error: {}", message);
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

	match runner::run_program(filename, goal_mode, algo, heuristic)
	{
		Ok(result) => display_result(result),
		Err(e) => exit_program(&e)
	}
}

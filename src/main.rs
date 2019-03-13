/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: ggregoir <ggregoir@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/03/13 15:45:44 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/13 15:53:53 by ggregoir         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#[macro_use]
extern crate clap;
use clap::App;
use std::fs;
use std::process::exit;

fn get_file(filename: &str) -> String
{
	fs::read_to_string(filename)
		.expect("Something went wrong reading the file")
}

fn exit_prog(message: &str) -> i32
{
	println!("{}", message);
	exit(0);
}

fn get_number(s: &str) -> i32
{
	match s.parse()
	{
		Ok(val)	if val > 0		=> val,
		Err(_)					=> exit_prog(&format!("'{}' is invalid", s)),
		_						=> exit_prog(&format!("'{}' cannot be negative", s))
	}
}

fn main() 
{
	let yaml = load_yaml!("man.yml");
	let matches = App::from_yaml(yaml).get_matches();
	println!("{:#?}", matches);

	let filename = "test";
    println!("In file {}", filename);

    let file = get_file(filename);

    let mut initial_state: Vec<Vec<i32>> = vec![vec![]];
    let mut size: i32 = -1;
    let lines: Vec<&str> = file.lines().collect();
    for (i, line) in lines.iter().enumerate()
    {
    	let words: Vec<&str> = line.split_whitespace().collect();
    	for word in words
    	{
    		if word.chars().next() == Some('#') { break }
    		// print!("{} ", word);
    		if size == -1
    		{
    			size = get_number(word);
    			println!("size = {}", size);
    		}
    		else
    		{
    			initial_state[i].push(get_number(word));
    		}
    	}
    	if (size != -1) && (initial_state[i].len() != size as usize)
    	{
    		exit_prog(&format!("invalid number of args at line {}", i));
    	}
    	// println!("content: '{:?}' len: '{}'", initial_state[i], initial_state[i].len());
    }
 	println!("initial_state: {:?}", initial_state);
}

/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   main.rs                                            :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: jafaghpo <jafaghpo@student.42.fr>          +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2019/03/13 15:45:44 by ggregoir          #+#    #+#             */
/*   Updated: 2019/03/13 18:10:25 by jafaghpo         ###   ########.fr       */
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
	eprintln!("{}", message);
	exit(1);
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

fn get_puzzle_size(file: &str) -> i32
{
	let lines: Vec<&str> = file.lines().collect();
	for line in lines.iter()
    {
    	let words: Vec<&str> = line.split_whitespace().collect();
    	for word in words
    	{
    		if word.chars().next() == Some('#') { break }
    		// print!("{} ", word);
			return get_number(word);
    	}
	}
	return 0;
}

fn get_puzzle_content(file: String, size: i32) -> Vec<i32>
{
	let lines: Vec<&str> = file.lines().collect();
	let mut content: Vec<i32> = vec![];
	let mut current_size: i32 = -1;
	let max_number: i32 = size.pow(2) - 1;

	for line in lines.iter()
    {
    	let words: Vec<&str> = line.split_whitespace().collect();
    	for word in words
    	{
			println!("word: {}", word);
    		if word.chars().next() == Some('#')
			{
				if (current_size != 0) && (current_size != size) && (current_size != -1)
				{
					exit_prog(&format!("Invalid number of tile per line. Expected {} got {}", size, current_size));
				}
				break;
			}
			match get_number(word)
			{
				x if x > 0 && x <= max_number	=> { content.push(x); }
				_ if current_size == -1			=> { break }
				x								=> { exit_prog(&format!("'{}' is not in range", x)); }
			}
			current_size += 1;
    	}
		println!("content: {:?}", content);
		println!("current_size: {}", current_size);
		if (current_size != 0) && (current_size != size) && (current_size != -1)
		{
			exit_prog(&format!("Invalid number of tile per line. Expected {} got {}", size, current_size));
		}
		current_size = 0;
	}
	return content;
}


fn main() 
{
	let yaml = load_yaml!("man.yml");
	let matches = App::from_yaml(yaml).get_matches();
	// println!("{:#?}", matches);
	let filename = matches.value_of("file").unwrap();

    let file = get_file(filename);

    let mut _initial_state: Vec<i32> = vec![];
	let size = get_puzzle_size(&file);
	let content = get_puzzle_content(file, size);
	println!("size: {}", size);
	println!("content: {:?}", content);
}

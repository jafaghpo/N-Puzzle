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

fn exit_prog(message: &str) -> i32
{
	eprintln!("error: {}", message);
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

fn check_in_range(x: i32, size: i32) -> i32
{
	if x > size.pow(2) - 1
	{
		return exit_prog(&format!("'{}' is not in range", x));
	}
	return x;
}

fn main() 
{
	let yaml = load_yaml!("man.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let filename = matches.value_of("file").unwrap();

    let file = fs::read_to_string(filename).expect("invalid filename");
	let mut lines: Vec<&str> = file
		.lines()
		.map(|x| x.split("#").nth(0).unwrap())
		.filter(|&x| x != "")
		.collect();

	let size: i32 = get_number(lines.remove(0));
	println!("size: {}", size);
	println!("{:#?}", lines);
}

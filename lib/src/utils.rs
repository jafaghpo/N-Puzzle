use std::fs::read_dir;

pub fn get_files_from(dir: &str) -> Vec<String>
{
	let paths;
	match read_dir(dir)
	{
		Ok(s)	=> paths = s,
		Err(_)	=> panic!(format!("[{}]: no such file or directory", dir))
	}

	let list: Vec<_> = paths.map(|x| x.unwrap().path()).collect();
	let mut tests: Vec<String> = vec![];
	for file in &list
	{
		let name = file.to_str().unwrap().to_string();

		tests.push(name);
	}
	tests
}
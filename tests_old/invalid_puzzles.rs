use npuzzle_lib::*;
use runner;

#[test]
fn invalid_puzzle()
{
	let tests = utils::get_files_from("puzzles/invalid");

	for test in &tests
	{
		let result = runner::run_program(
			test,
			"snail",
			"a_star",
			"manhattan"
		);

		match result
		{
			Ok(_)	=> assert!(false, "[{}] should return an error", test),
			Err(e)	=>
			{
				let keyword = test.rsplit("_").next().unwrap();
				assert!(
					e.contains(keyword),
					"expected an error with keyword [{}] instead of [{}] for file [{}]",
					keyword, e, test
				);
			}
		}
	}
}
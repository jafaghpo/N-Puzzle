use npuzzle_lib::*;
use runner;

#[test]
fn solvable_puzzle()
{
	let tests = utils::get_files_from("puzzles/unsolvable");
	let expected = "unsolvable puzzle";

	for test in &tests
	{
		let mode = test.split("_").next().unwrap();
		let result = runner::run_program(
			test,
			mode,
			"a_star",
			"manhattan"
		);

		match result
		{
			Ok(_)	=> assert!(false, "[{}] should be unsolvable", test),
			Err(e)	=> assert!(e == expected, "expected [{}] instead of [{}]", expected, e)
		}
	}
}
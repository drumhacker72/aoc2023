use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
	let file = File::open("day1.txt")
		.expect("day1.txt should be openable");
	let lines = BufReader::new(file).lines();
	let mut total = 0;
	for line in lines {
		let l = line
			.expect("each line should be readable");
		let calibration = first_digit(&l) * 10 + last_digit(&l);
		//println!("{calibration}");
		total += calibration;
	}
	println!("{total}");
}

fn first_digit(line: &str) -> u32 {
	let d = line.chars().find(|c| c.is_digit(10))
		.expect("line should have at least one digit");
	d.to_digit(10)
		.expect("found digit character should be a number")
}

fn last_digit(line: &str) -> u32 {
	let rev: String = line.chars().rev().collect();
	first_digit(&rev)
}

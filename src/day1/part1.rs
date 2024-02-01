use std::fs::File;
use std::io::{BufRead, BufReader};

fn read_file(path: &str) -> Result<Vec<String>, std::io::Error> {
	let 	file: File;
	let 	reader: BufReader<File>;
	let mut result: Vec<String>;

	file = File::open(path)?;
	reader = BufReader::new(file);
	result = Vec::new();
	for line in reader.lines() {
		let line = line.expect("Failed to read line");
		match line.is_empty() {
			true => {
				continue;
			},
			false => {
				result.push(line);
			}
		}
	}
	Ok(result)
}

fn get_digits(vec: &Vec<String>) -> Vec<usize> {
	let result: Vec<usize> = Vec::new();

	for str in vec.iter() {
		println!("{}", str);
	}

	result
}

pub fn get_calibration_digit() -> usize {
	let mut str_vec: Vec<String>;
	let 	num_vec: Vec<usize>;

	str_vec = Vec::new();
	if let Ok(res) = read_file("inputs/day1.txt") {
		str_vec = res;
	}

	num_vec = get_digits(&str_vec);


	let sum: usize = num_vec.iter().sum();

	// TODO: Get the digits into a Vec<usize>

	// TODO: Return the sum of the Vec<>
	sum
}

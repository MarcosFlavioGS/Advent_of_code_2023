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

fn get_digits(vec: &Vec<String>) -> Vec<String> {
	let mut result: Vec<String> = Vec::new();

	for str in vec.iter() {
		let mut string: String = String::new();

		for char in str.chars() {
			if char.is_numeric() {
				string.push(char);
				break;
			}
		}
		for char in str.chars().rev() {
			if char.is_numeric() {
				string.push(char);
				break;
			}
		}
		result.push(string);
	}

	result
}

pub fn get_calibration_digit() -> usize {
	if let Ok(res) = read_file("inputs/day1.txt") {
		let num_vec = get_digits(&res);
		num_vec.iter().map(|str| str.parse::<usize>().unwrap()).sum()
	} else {
		42
	}
}

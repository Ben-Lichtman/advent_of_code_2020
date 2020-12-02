use regex::Regex;

use std::fs::read_to_string;

struct Password {
	num_a: u32,
	num_b: u32,
	c: char,
	pw: String,
}

impl Password {
	fn is_valid_1(&self) -> bool {
		let count = self.pw.chars().filter(|c| *c == self.c).count() as u32;
		self.num_a <= count && count <= self.num_b
	}

	fn is_valid_2(&self) -> bool {
		let check_1 = self.pw.chars().nth(self.num_a as usize - 1).unwrap() == self.c;
		let check_2 = self.pw.chars().nth(self.num_b as usize - 1).unwrap() == self.c;
		check_1 ^ check_2
	}
}

fn parse_input(file: &str) -> Vec<Password> {
	let input = read_to_string(file).unwrap();

	let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)").unwrap();

	let processed = input
		.lines()
		.map(|l| {
			let caps = re.captures(l).unwrap();
			let num_a = caps[1].parse().unwrap();
			let num_b = caps[2].parse().unwrap();
			let c = caps[3].chars().next().unwrap();
			let pw = String::from(&caps[4]);
			Password {
				num_a,
				num_b,
				c,
				pw,
			}
		})
		.collect::<Vec<_>>();

	processed
}

fn main() {
	let passwords = parse_input("input/day2/1.txt");

	let count = passwords.iter().filter(|p| p.is_valid_1()).count();

	println!("Part 1: {}", count);

	let count = passwords.iter().filter(|p| p.is_valid_2()).count();

	println!("Part 2: {}", count);
}

use parse_display::FromStr;

use std::fs::read_to_string;

#[derive(FromStr)]
#[display(r"{num_a}-{num_b} {c}: {pw}")]
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

	let processed = input
		.lines()
		.map(|l| l.parse::<Password>().unwrap())
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

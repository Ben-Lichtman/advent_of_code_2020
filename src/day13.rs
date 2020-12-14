use std::fs::read_to_string;

fn find_next(from: u64, period: u64) -> u64 { (1 + (from - 1) / period) * period }

fn simple_chinese_remainder(pairs: &[(u64, u64)]) -> u64 {
	let mut x = 0;
	let mut step_by = 1;
	for (modulo, residue) in pairs.iter().copied() {
		while x % modulo != residue {
			x += step_by;
		}
		step_by *= modulo;
	}
	x
}

fn main() {
	let input = read_to_string("input/day13/1.txt").unwrap();
	let mut lines = input.lines();
	let num1 = lines.next().unwrap().parse::<u64>().unwrap();
	let line2 = lines.next().unwrap();

	let buses = line2
		.split(',')
		.filter(|w| w != &"x")
		.map(|n| n.parse::<u64>().unwrap())
		.collect::<Vec<_>>();

	let next = buses
		.iter()
		.copied()
		.map(|i| (find_next(num1, i), i))
		.min()
		.unwrap();

	println!("Part 1: {}", (next.0 - num1) * next.1);

	let buses = line2
		.split(',')
		.enumerate()
		.filter(|(_, w)| w != &"x")
		.map(|(i, n)| (i as u64, n.parse::<u64>().unwrap()))
		.map(|(i, n)| (n, (n - (i % n)) % n))
		.collect::<Vec<_>>();

	let crt = simple_chinese_remainder(&buses);

	println!("Part 2: {}", crt);
}

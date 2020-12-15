use std::{collections::HashMap, fs::read_to_string};

fn calculate(finish: usize, starters: &[usize]) -> usize {
	let mut lookup = HashMap::new();
	let mut nums = Vec::new();
	let mut candidate;

	// Initialise data
	for num in starters[..starters.len() - 1].iter().copied() {
		lookup.insert(num, nums.len());
		nums.push(num);
	}
	candidate = starters[starters.len() - 1];

	loop {
		let index = nums.len();
		if index + 1 == finish {
			break candidate;
		}

		let next_num = match lookup.get(&candidate) {
			Some(prev) => index - prev,
			None => 0,
		};
		lookup.insert(candidate, index);
		nums.push(candidate);
		candidate = next_num;
	}
}

fn main() {
	let input = read_to_string("input/day15/1.txt").unwrap();
	let starting_nums = input
		.trim()
		.split(',')
		.map(|n| n.parse::<usize>().unwrap())
		.collect::<Vec<_>>();

	let part_1 = calculate(2020, &starting_nums);

	println!("Part 1: {}", part_1);

	let part_2 = calculate(30000000, &starting_nums);

	println!("Part 2: {}", part_2);
}

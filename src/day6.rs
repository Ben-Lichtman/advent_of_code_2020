use std::{collections::HashSet, fs::read_to_string};

fn parse_input(file: &str) -> Vec<Vec<Vec<char>>> {
	let input = read_to_string(file).unwrap();

	let processed = input
		.split("\n\n")
		.map(|group| {
			group
				.lines()
				.map(|person| person.chars().collect::<Vec<_>>())
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	processed
}

fn main() {
	let groups = parse_input("input/day6/1.txt");

	let groups_processed_1 = groups
		.iter()
		.map(|group| {
			group
				.iter()
				.map(|person| person.iter().collect::<HashSet<_>>())
				.fold(None, |acc, x| match acc {
					None => Some(x),
					Some(acc) => Some(&acc | &x),
				})
		})
		.collect::<Vec<_>>();

	let count_1 = groups_processed_1
		.iter()
		.map(|set| set.as_ref().unwrap().len())
		.sum::<usize>();

	println!("Part 1: {}", count_1);

	let groups_processed_2 = groups
		.iter()
		.map(|group| {
			group
				.iter()
				.map(|person| person.iter().collect::<HashSet<_>>())
				.fold(None, |acc, x| match acc {
					None => Some(x),
					Some(acc) => Some(&acc & &x),
				})
		})
		.collect::<Vec<_>>();

	let count_2 = groups_processed_2
		.iter()
		.map(|set| set.as_ref().unwrap().len())
		.sum::<usize>();

	println!("Part 2: {}", count_2);
}

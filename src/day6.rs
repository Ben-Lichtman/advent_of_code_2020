use std::fs::read_to_string;

fn parse_input(file: &str) -> Vec<Vec<u32>> {
	let input = read_to_string(file).unwrap();

	let processed = input
		.split("\n\n")
		.map(|group| {
			group
				.lines()
				.map(|person| {
					person
						.chars()
						.fold(0u32, |acc, c| acc | (1 << (c as u32 - 'a' as u32)))
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	processed
}

fn main() {
	let groups = parse_input("input/day6/1.txt");

	let groups_processed_1 = groups
		.iter()
		.map(|group| group.iter().fold(0, |acc, x| acc | x))
		.collect::<Vec<_>>();

	let count_1 = groups_processed_1
		.iter()
		.map(|set| set.count_ones() as usize)
		.sum::<usize>();

	println!("Part 1: {}", count_1);

	let groups_processed_2 = groups
		.iter()
		.map(|group| group.iter().fold(0x3ffffff, |acc, x| acc & x))
		.collect::<Vec<_>>();

	let count_2 = groups_processed_2
		.iter()
		.map(|set| set.count_ones() as usize)
		.sum::<usize>();

	println!("Part 2: {}", count_2);
}

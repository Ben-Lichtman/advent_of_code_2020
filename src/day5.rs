use std::fs::read_to_string;

fn parse_input(file: &str) -> Vec<Vec<char>> {
	let input = read_to_string(file).unwrap();

	let processed = input
		.lines()
		.map(|l| l.chars().collect::<Vec<_>>())
		.collect::<Vec<_>>();

	processed
}

fn main() {
	let tickets = parse_input("input/day5/1.txt");
	let mut nums = tickets
		.iter()
		.map(|v| {
			v.iter()
				.map(|c| match c {
					'F' => false,
					'B' => true,
					'L' => false,
					'R' => true,
					_ => panic!("Invalid ticket"),
				})
				.fold(0u16, |acc, new| (acc << 1) | new as u16)
		})
		.collect::<Vec<_>>();
	let max = nums.iter().max().unwrap();

	println!("Part 1: {}", max);

	nums.sort_unstable();

	nums.windows(2).for_each(|w| {
		if w[1] == w[0] + 2 {
			println!("Part 2: {}", w[0] + 1);
		}
	})
}

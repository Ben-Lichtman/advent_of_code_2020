use std::fs::read_to_string;

fn parse_input(file: &str) -> Vec<Vec<bool>> {
	let input = read_to_string(file).unwrap();

	let processed = input
		.lines()
		.map(|l| l.chars().map(|c| c == '#').collect::<Vec<_>>())
		.collect::<Vec<_>>();

	processed
}

fn check_slope(trees: &[Vec<bool>], slope: (usize, usize)) -> u32 {
	let width = trees[0].len();

	let mut coord = (0, 0);
	let mut count = 0;
	while coord.1 < trees.len() {
		if trees[coord.1][coord.0 % width] {
			count += 1;
		}
		coord.0 += slope.0;
		coord.1 += slope.1;
	}
	count
}

fn main() {
	let trees = parse_input("input/day3/1.txt");

	// dbg!(&trees);

	let count = check_slope(&trees, (3, 1));

	println!("Part 1: {}", count);

	let total = check_slope(&trees, (1, 1))
		* check_slope(&trees, (3, 1))
		* check_slope(&trees, (5, 1))
		* check_slope(&trees, (7, 1))
		* check_slope(&trees, (1, 2));

	println!("Part 2: {}", total);
}

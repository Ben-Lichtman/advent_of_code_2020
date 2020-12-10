use std::fs::read_to_string;

fn parse_to_vec(i: &str) -> Vec<u32> { i.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>() }

fn main() {
	let input = read_to_string("input/day10/1.txt").unwrap();
	let mut nums = parse_to_vec(&input);
	nums.push(0);
	nums.sort_unstable();
	nums.push(nums[nums.len() - 1] + 3);

	let differences = nums.windows(2).map(|window| window[1] - window[0]).fold(
		(0, 0, 0),
		|(sum_1, sum_2, sum_3), new| match new {
			1 => (sum_1 + 1, sum_2, sum_3),
			2 => (sum_1, sum_2 + 1, sum_3),
			3 => (sum_1, sum_2, sum_3 + 1),
			_ => panic!("Invalid difference"),
		},
	);

	println!("Part 1: {}", differences.0 * differences.2);

	let mut paths = vec![0u64; nums.len()];
	paths[0] = 1;
	for i in 0..nums.len() {
		let current_num = nums[i];
		let current_paths = paths[i];

		for offset in 1..=3 {
			if let Some(&x) = nums.get(i + offset) {
				if x <= current_num + 3 {
					paths[i + offset] += current_paths;
				}
			}
		}
	}

	println!("Part 2: {}", paths.last().unwrap());
}

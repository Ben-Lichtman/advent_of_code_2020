use std::fs::read_to_string;

const INPUT_FILE: &str = "input/day1/1.txt";

fn find_pair(nums: &[u32], target: u32) -> Option<(u32, u32)> {
	let mut min = 0;
	let mut max = nums.len() - 1;

	while min != max {
		let sum = nums[min] + nums[max];

		match sum {
			x if x > target => max -= 1,
			x if x < target => min += 1,
			_ => return Some((nums[min], nums[max])),
		}
	}
	None
}

fn main() {
	let input = read_to_string(INPUT_FILE).unwrap();

	let mut nums = input
		.lines()
		.map(|l| l.parse::<u32>().unwrap())
		.collect::<Vec<_>>();

	nums.sort_unstable();

	let (a, b) = find_pair(&nums, 2020).unwrap();

	println!("Part 1: {}", a * b);

	for a in nums.iter() {
		if let Some((b, c)) = find_pair(&nums, 2020 - a) {
			println!("Part 2: {}", a * b * c);
			break;
		}
	}
}

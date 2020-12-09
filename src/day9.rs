use std::fs::read_to_string;

fn parse_to_vec(i: &str) -> Vec<u64> { i.lines().map(|l| l.parse().unwrap()).collect::<Vec<_>>() }

fn find_pair(nums_sorted: &[u64], target: u64) -> Option<(u64, u64)> {
	let mut current_slice = nums_sorted;

	loop {
		let (num1, num2) = match current_slice {
			[] => break None,
			[val] => (*val, *val),
			[v1, .., v2] => (*v1, *v2),
		};
		match num1 + num2 {
			x if x > target => current_slice = &current_slice[..current_slice.len() - 1],
			x if x < target => current_slice = &current_slice[1..],
			_ => break Some((num1, num2)),
		}
	}
}

fn scan_windows(input: &[u64], window_size: usize) -> Vec<(usize, u64)> {
	input
		.windows(window_size)
		.enumerate()
		.filter(|(_, window)| {
			let before = &window[0..window.len() - 1];
			let end = *window.last().unwrap();

			let mut before_sorted = before.to_vec();
			before_sorted.sort_unstable();

			find_pair(&before_sorted, end).is_none()
		})
		.map(|(n, window)| (n + window_size, *window.last().unwrap()))
		.collect::<Vec<_>>()
}

fn find_sum_subslice(slice: &[u64], target: u64) -> &[u64] {
	let mut start = 0;
	let mut end = 0;
	let mut total = 0;
	loop {
		if total == target {
			break &slice[start..end + 1];
		}
		else if total < target {
			total += slice[end];
			end += 1;
		}
		else if total > target {
			total -= slice[start];
			start += 1;
		}
	}
}

fn main() {
	let input = read_to_string("input/day9/1.txt").unwrap();

	let nums = parse_to_vec(&input);

	let prelude_size = 25;

	let found = scan_windows(&nums, prelude_size + 1);

	let part_1 = found.first().unwrap().1;

	println!("Part 1: {}", part_1);

	let subslice = find_sum_subslice(&nums, part_1);

	let mut subslice_sorted = subslice.to_vec();
	subslice_sorted.sort_unstable();
	if let [start, .., end] = subslice_sorted[..] {
		let sum = start + end;
		println!("Part 2: {}", sum);
	}
}

use regex::Regex;

use std::collections::HashMap;

use std::fs::read_to_string;

#[derive(Clone, Copy, Debug)]
enum Cmd {
	Mask((u64, u64)),
	Mem((u64, u64)),
}

fn parse_to_vec(i: &str) -> Vec<Cmd> {
	let mask_re = Regex::new(r"mask = ([01X]*)").unwrap();
	let mem_re = Regex::new(r"mem\[(\d*)\] = (\d*)").unwrap();
	i.lines()
		.map(|l| {
			if &l[..4] == "mask" {
				let caps = mask_re.captures(l).unwrap();
				let mask =
					caps[1]
						.chars()
						.fold((u64::MAX, 0), |(and_mask, or_mask), new| match new {
							'X' => ((and_mask << 1) | 1, or_mask << 1),
							'0' => (and_mask << 1, or_mask << 1),
							'1' => ((and_mask << 1) | 1, (or_mask << 1) | 1),
							_ => panic!("Invalid character"),
						});
				Cmd::Mask(mask)
			}
			else if &l[..3] == "mem" {
				let caps = mem_re.captures(l).unwrap();
				let addr = caps[1].parse().unwrap();
				let val = caps[2].parse().unwrap();
				Cmd::Mem((addr, val))
			}
			else {
				panic!("Invalid command");
			}
		})
		.collect::<Vec<_>>()
}

fn main() {
	let input = read_to_string("input/day14/1.txt").unwrap();
	let cmds = parse_to_vec(&input);

	let mut memory = vec![0; 0x1000000];
	let mut current_mask = (u64::MAX, 0);
	for cmd in cmds.iter().copied() {
		match cmd {
			Cmd::Mask(m) => current_mask = m,
			Cmd::Mem((addr, val)) => {
				let masked_val = (val & current_mask.0) | current_mask.1;
				memory[addr as usize] = masked_val;
			}
		}
	}

	let part_1 = memory.iter().copied().sum::<u64>();

	println!("Part 1: {}", part_1);

	let mut memory = HashMap::<u64, u64>::new();
	let mut current_mask = (u64::MAX, 0);
	for cmd in cmds.iter().copied() {
		match cmd {
			Cmd::Mask(m) => current_mask = m,
			Cmd::Mem((addr, val)) => {
				let masked_addr = addr | current_mask.1;
				let permuted_bits = current_mask.0 ^ current_mask.1;
				let mut permuted_addrs = vec![masked_addr];
				for bit_num in 0..36 {
					let bit = 1 << bit_num;
					if permuted_bits & bit != 0 {
						for addr in permuted_addrs.iter_mut() {
							*addr &= !bit;
						}

						let mut permuted_addrs_ones = permuted_addrs.clone();
						for addr in permuted_addrs_ones.iter_mut() {
							*addr |= bit;
						}
						permuted_addrs.extend_from_slice(&permuted_addrs_ones);
					}
				}
				for addr in permuted_addrs {
					memory.insert(addr, val);
				}
			}
		}
	}

	let part_2 = memory.iter().map(|(_, v)| v).copied().sum::<u64>();

	println!("Part 2: {}", part_2);
}

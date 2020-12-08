use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Instruction {
	Acc(i32),
	Jmp(isize),
	Nop(isize),
}

fn parse_to_vec(i: &str) -> Vec<Instruction> {
	i.lines()
		.map(|l| l.split_whitespace().collect::<Vec<_>>())
		.map(|words| match words[0] {
			"acc" => Instruction::Acc(words[1].parse().unwrap()),
			"jmp" => Instruction::Jmp(words[1].parse().unwrap()),
			"nop" => Instruction::Nop(words[1].parse().unwrap()),
			_ => panic!("Invalid instrucion"),
		})
		.collect::<Vec<_>>()
}

fn run_machine(instructions: &[Instruction], max: u8) -> (i32, isize) {
	let mut run = vec![0u8; instructions.len()];

	let mut ip = 0isize;
	let mut acc = 0i32;

	loop {
		// Exit if run twice
		let run_n_times = run.get(ip as usize);
		if run_n_times == None || run_n_times == Some(&max) {
			break;
		}
		run[ip as usize] += 1;

		let curr = instructions.get(ip as usize);
		match curr {
			Some(Instruction::Acc(num)) => acc += num,
			Some(Instruction::Jmp(num)) => {
				ip += num;
				continue;
			}
			Some(Instruction::Nop(_)) => (),
			None => break,
		}
		ip += 1;
	}

	(acc, ip)
}

fn try_run_machine(instructions: &[Instruction], max: u8) -> (i32, bool) {
	let (acc, ip) = run_machine(instructions, max);
	(acc, ip == instructions.len() as isize)
}

fn main() {
	let input = read_to_string("input/day8/1.txt").unwrap();
	let instructions = parse_to_vec(&input);

	let (part_1, _) = run_machine(&instructions, 1);

	println!("Part 1: {}", part_1);

	let mut final_acc = 0;
	for test in 0..instructions.len() {
		let mut modified = instructions.clone();
		match modified[test] {
			Instruction::Jmp(num) => modified[test] = Instruction::Nop(num),
			Instruction::Nop(num) => modified[test] = Instruction::Jmp(num),
			_ => continue,
		}

		let (acc, succ) = try_run_machine(&modified, 255);
		if succ {
			final_acc = acc;
			break;
		}
	}

	println!("Part 2: {}", final_acc);
}

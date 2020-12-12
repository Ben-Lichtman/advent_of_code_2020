use std::fs::read_to_string;

#[derive(Clone, Copy)]
enum Command {
	F(i32),
	L(i32),
	R(i32),
	N(i32),
	S(i32),
	E(i32),
	W(i32),
}

struct Ship1 {
	x: i32,
	y: i32,
	dir: u32,
}

impl Ship1 {
	fn do_command(&mut self, cmd: Command) {
		let mut rotate = |num| {
			self.dir = (self.dir + num as u32) % 360;
		};

		match cmd {
			Command::F(num) => match self.dir {
				0 => self.y += num,
				90 => self.x += num,
				180 => self.y -= num,
				270 => self.x -= num,
				_ => panic!("Invalid rotation"),
			},
			Command::L(num) => rotate(360 - num),
			Command::R(num) => rotate(num),
			Command::N(num) => self.y += num,
			Command::S(num) => self.y -= num,
			Command::E(num) => self.x += num,
			Command::W(num) => self.x -= num,
		}
	}
}

struct Ship2 {
	x: i32,
	y: i32,
	w_x: i32,
	w_y: i32,
}

impl Ship2 {
	fn do_command(&mut self, cmd: Command) {
		let mut rotate = |num| match num {
			0 => (),
			90 => {
				let (x, y) = (self.w_y, -self.w_x);
				self.w_x = x;
				self.w_y = y;
			}
			180 => {
				let (x, y) = (-self.w_x, -self.w_y);
				self.w_x = x;
				self.w_y = y;
			}
			270 => {
				let (x, y) = (-self.w_y, self.w_x);
				self.w_x = x;
				self.w_y = y;
			}
			_ => panic!("Invalid rotation"),
		};

		match cmd {
			Command::F(num) => {
				self.x += num * self.w_x;
				self.y += num * self.w_y;
			}
			Command::L(num) => rotate(360 - num),
			Command::R(num) => rotate(num),
			Command::N(num) => self.w_y += num,
			Command::S(num) => self.w_y -= num,
			Command::E(num) => self.w_x += num,
			Command::W(num) => self.w_x -= num,
		}
	}
}

fn parse_to_vec(i: &str) -> Vec<Command> {
	i.lines()
		.map(|l| {
			let num = l[1..].parse().unwrap();
			match &l[..1] {
				"F" => Command::F(num),
				"L" => Command::L(num),
				"R" => Command::R(num),
				"N" => Command::N(num),
				"S" => Command::S(num),
				"E" => Command::E(num),
				"W" => Command::W(num),
				_ => panic!("Invalid input"),
			}
		})
		.collect::<Vec<_>>()
}

fn main() {
	let input = read_to_string("input/day12/1.txt").unwrap();
	let cmds = parse_to_vec(&input);

	let mut ship = Ship1 {
		dir: 90,
		x: 0,
		y: 0,
	};

	cmds.iter().for_each(|c| ship.do_command(*c));

	let manhattan = ship.x.abs() + ship.y.abs();

	println!("Part 1: {}", manhattan);

	let mut ship = Ship2 {
		x: 0,
		y: 0,
		w_x: 10,
		w_y: 1,
	};

	cmds.iter().for_each(|c| ship.do_command(*c));

	let manhattan = ship.x.abs() + ship.y.abs();

	println!("Part 2: {}", manhattan);
}

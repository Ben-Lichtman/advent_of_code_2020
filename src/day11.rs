use std::{fs::read_to_string, mem::swap};

#[derive(Clone, Copy, Debug)]
enum Cell {
	Floor,
	Empty,
	Occupied,
}

#[derive(Debug)]
struct Automata {
	x: usize,
	y: usize,
	state: Box<[Cell]>,
	next_state: Box<[Cell]>,
}

impl Automata {
	fn new(input: Vec<Vec<Cell>>) -> Self {
		let y = input.len();
		let x = input[0].len();
		let mut state = Vec::with_capacity(x * y);

		input
			.into_iter()
			.flat_map(|v| v.into_iter())
			.for_each(|s| state.push(s));

		let state = state.into_boxed_slice();
		let next_state = state.clone();

		Self {
			x,
			y,
			state,
			next_state,
		}
	}

	fn coord_to_index(&self, x: usize, y: usize) -> usize { y * self.x + x }

	fn verify_coord(&self, x: usize, y: usize) -> bool { x < self.x && y < self.y }

	fn kernel_index_1(&self, x: usize, y: usize, i: usize) -> Option<(usize, usize)> {
		// 0 <= i < 8
		let x = x + i % 3;
		let y = y + i / 3;
		match (x.checked_sub(1), y.checked_sub(1)) {
			(Some(x), Some(y)) if self.verify_coord(x, y) => Some((x, y)),
			_ => None,
		}
	}

	fn get_kernel_1(&self, x: usize, y: usize) -> [Option<Cell>; 9] {
		let mut table = [None; 9];
		(0..9).for_each(|i| {
			table[i] = self
				.kernel_index_1(x, y, i)
				.map(|(x, y)| self.coord_to_index(x, y))
				.map(|i| self.state[i])
		});
		table
	}

	fn print(&self) {
		for y in 0..self.y {
			for x in 0..self.x {
				let c = match self.state[self.coord_to_index(x, y)] {
					Cell::Floor => '.',
					Cell::Empty => 'L',
					Cell::Occupied => '#',
				};
				print!("{}", c)
			}
			println!("");
		}
	}

	fn next_1(&mut self) -> (u32, u32, u32, u32) {
		let mut changed = 0;
		for y in 0..self.y {
			for x in 0..self.x {
				let target_index = self.coord_to_index(x, y);
				let current_cell = self.state[target_index];

				let get_adjacent_occupied = || {
					let mut table = self.get_kernel_1(x, y);
					// Ignore center of kernel
					table[4] = None;

					table
						.iter()
						.filter(|x| match x {
							Some(Cell::Occupied) => true,
							_ => false,
						})
						.count()
				};

				self.next_state[target_index] = match current_cell {
					Cell::Floor => Cell::Floor,
					Cell::Empty => {
						if get_adjacent_occupied() == 0 {
							changed += 1;
							Cell::Occupied
						}
						else {
							Cell::Empty
						}
					}
					Cell::Occupied => {
						if get_adjacent_occupied() >= 4 {
							changed += 1;
							Cell::Empty
						}
						else {
							Cell::Occupied
						}
					}
				}
			}
		}
		let (floor, empty, occupied) =
			self.next_state
				.iter()
				.fold((0, 0, 0), |(floor, empty, occupied), new| match new {
					Cell::Floor => (floor + 1, empty, occupied),
					Cell::Empty => (floor, empty + 1, occupied),
					Cell::Occupied => (floor, empty, occupied + 1),
				});

		swap(&mut self.state, &mut self.next_state);
		(changed, floor, empty, occupied)
	}

	fn next_2(&mut self) -> (u32, u32, u32, u32) {
		let mut changed = 0;
		for y in 0..self.y {
			for x in 0..self.x {
				let target_index = self.coord_to_index(x, y);
				let current_cell = self.state[target_index];

				// println!("Checking {:?}", (x, y));

				let get_view_occupied = || {
					let directions = [
						(-1, -1),
						(-1, 0),
						(-1, 1),
						(0, -1),
						(0, 1),
						(1, -1),
						(1, 0),
						(1, 1),
					];
					directions
						.iter()
						.filter(|(x_move, y_move)| {
							let mut x = x as isize;
							let mut y = y as isize;

							loop {
								x += x_move;
								y += y_move;

								// Check for occupied seats
								if x < 0 || y < 0 {
									break false;
								}

								let x = x as usize;
								let y = y as usize;

								if !self.verify_coord(x, y) {
									break false;
								}

								match self.state[self.coord_to_index(x, y)] {
									Cell::Floor => (),
									Cell::Empty => break false,
									Cell::Occupied => break true,
								}
							}
						})
						.count()
				};

				self.next_state[target_index] = match current_cell {
					Cell::Floor => Cell::Floor,
					Cell::Empty => {
						if get_view_occupied() == 0 {
							changed += 1;
							Cell::Occupied
						}
						else {
							Cell::Empty
						}
					}
					Cell::Occupied => {
						if get_view_occupied() >= 5 {
							changed += 1;
							Cell::Empty
						}
						else {
							Cell::Occupied
						}
					}
				}
			}
		}
		let (floor, empty, occupied) =
			self.next_state
				.iter()
				.fold((0, 0, 0), |(floor, empty, occupied), new| match new {
					Cell::Floor => (floor + 1, empty, occupied),
					Cell::Empty => (floor, empty + 1, occupied),
					Cell::Occupied => (floor, empty, occupied + 1),
				});

		swap(&mut self.state, &mut self.next_state);
		(changed, floor, empty, occupied)
	}
}

fn parse_to_vec(i: &str) -> Vec<Vec<Cell>> {
	i.lines()
		.map(|l| {
			l.chars()
				.map(|c| match c {
					'.' => Cell::Floor,
					'L' => Cell::Empty,
					'#' => Cell::Occupied,
					_ => panic!("Invalid character"),
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn main() {
	let input = read_to_string("input/day11/1.txt").unwrap();
	let nums = parse_to_vec(&input);

	let mut a = Automata::new(nums.clone());

	let occupied = loop {
		let (changed, _, _, occupied) = a.next_1();
		if changed == 0 {
			break occupied;
		}
	};

	println!("Part 1: {}", occupied);

	let mut a = Automata::new(nums.clone());

	let occupied = loop {
		println!("===============");
		a.print();
		let (changed, _, _, occupied) = a.next_2();
		if changed == 0 {
			break occupied;
		}
	};

	println!("===============");
	a.print();
	println!("===============");

	println!("Part 2: {}", occupied);
}

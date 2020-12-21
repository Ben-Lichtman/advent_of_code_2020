use std::{
	cmp::{max, min},
	collections::HashSet,
	fs::read_to_string,
	iter::FromIterator,
	mem::swap,
};

fn parse_to_vec(i: &str) -> Vec<(i32, i32)> {
	i.lines()
		.enumerate()
		.flat_map(|(y, l)| {
			l.chars()
				.enumerate()
				.filter(|(_, c)| c == &'#')
				.map(|(x, _)| (x as i32, y as i32))
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>()
}

fn active_neighbours_1(set: &HashSet<(i32, i32, i32)>, (x, y, z): (i32, i32, i32)) -> u32 {
	let mut ans = 0;

	for x_off in -1..=1 {
		for y_off in -1..=1 {
			for z_off in -1..=1 {
				if x_off == 0 && y_off == 0 && z_off == 0 {
					continue;
				}
				let checking = (x + x_off, y + y_off, z + z_off);
				if set.contains(&checking) {
					ans += 1;
				}
			}
		}
	}
	ans
}

fn active_neighbours_2(
	set: &HashSet<(i32, i32, i32, i32)>,
	(x, y, z, u): (i32, i32, i32, i32),
) -> u32 {
	let mut ans = 0;

	for x_off in -1..=1 {
		for y_off in -1..=1 {
			for z_off in -1..=1 {
				for u_off in -1..=1 {
					if x_off == 0 && y_off == 0 && z_off == 0 && u_off == 0 {
						continue;
					}
					let checking = (x + x_off, y + y_off, z + z_off, u + u_off);
					if set.contains(&checking) {
						ans += 1;
					}
				}
			}
		}
	}
	ans
}

fn main() {
	let input = read_to_string("input/day17/1.txt").unwrap();
	let coords = parse_to_vec(&input);

	let mut current = HashSet::from_iter(coords.iter().copied().map(|(x, y)| (x, y, 0)));
	let mut next = HashSet::new();

	for _ in 0..6 {
		let (mut min_x, mut min_y, mut min_z, mut max_x, mut max_y, mut max_z) = (0, 0, 0, 0, 0, 0);
		for (x, y, z) in current.iter().copied() {
			min_x = min(min_x, x);
			min_y = min(min_y, y);
			min_z = min(min_z, z);

			max_x = max(max_x, x);
			max_y = max(max_y, y);
			max_z = max(max_z, z);
		}
		min_x -= 1;
		min_y -= 1;
		min_z -= 1;

		max_x += 1;
		max_y += 1;
		max_z += 1;

		for x in min_x..=max_x {
			for y in min_y..=max_y {
				for z in min_z..=max_z {
					let n_neighbours = active_neighbours_1(&current, (x, y, z));
					let contained = current.contains(&(x, y, z));
					match contained {
						false => {
							if n_neighbours == 3 {
								next.insert((x, y, z));
							}
						}
						true => {
							if n_neighbours == 2 || n_neighbours == 3 {
								next.insert((x, y, z));
							}
						}
					}
				}
			}
		}

		swap(&mut current, &mut next);
		next.clear();
	}

	let part_1 = current.len();

	println!("Part 1: {}", part_1);

	let mut current = HashSet::from_iter(coords.iter().copied().map(|(x, y)| (x, y, 0, 0)));
	let mut next = HashSet::new();

	for _ in 0..6 {
		let (
			mut min_x,
			mut min_y,
			mut min_z,
			mut min_u,
			mut max_x,
			mut max_y,
			mut max_z,
			mut max_u,
		) = (0, 0, 0, 0, 0, 0, 0, 0);
		for (x, y, z, u) in current.iter().copied() {
			min_x = min(min_x, x);
			min_y = min(min_y, y);
			min_z = min(min_z, z);
			min_u = min(min_u, u);

			max_x = max(max_x, x);
			max_y = max(max_y, y);
			max_z = max(max_z, z);
			max_u = max(max_u, u);
		}
		min_x -= 1;
		min_y -= 1;
		min_z -= 1;
		min_u -= 1;

		max_x += 1;
		max_y += 1;
		max_z += 1;
		max_u += 1;

		for x in min_x..=max_x {
			for y in min_y..=max_y {
				for z in min_z..=max_z {
					for u in min_u..=max_u {
						let n_neighbours = active_neighbours_2(&current, (x, y, z, u));
						let contained = current.contains(&(x, y, z, u));
						match contained {
							false => {
								if n_neighbours == 3 {
									next.insert((x, y, z, u));
								}
							}
							true => {
								if n_neighbours == 2 || n_neighbours == 3 {
									next.insert((x, y, z, u));
								}
							}
						}
					}
				}
			}
		}

		swap(&mut current, &mut next);
		next.clear();
	}

	let part_2 = current.len();

	println!("Part 2: {}", part_2);
}

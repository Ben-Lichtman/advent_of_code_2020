use std::{fs::read_to_string, iter::once, ops::Range};

use regex::Regex;

use csv::{Reader, ReaderBuilder};

struct Field {
	name: String,
	range1: Range<u32>,
	range2: Range<u32>,
}

struct Tickets {
	fields: Vec<Field>,

	my_ticket: Vec<u32>,
	other_tickets: Vec<Vec<u32>>,
}

fn parse_to_struct(i: &str) -> Tickets {
	let fields_re = Regex::new(r"(.*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();

	let mut sections = i.split("\n\n");
	let fields = sections
		.next()
		.unwrap()
		.lines()
		.map(|l| {
			let caps = fields_re.captures(l).unwrap();
			Field {
				name: String::from(&caps[1]),
				range1: Range {
					start: caps[2].parse::<u32>().unwrap(),
					end: caps[3].parse::<u32>().unwrap() + 1,
				},
				range2: Range {
					start: caps[4].parse::<u32>().unwrap(),
					end: caps[5].parse::<u32>().unwrap() + 1,
				},
			}
		})
		.collect::<Vec<_>>();
	let my_ticket = sections
		.next()
		.unwrap()
		.lines()
		.skip(1)
		.flat_map(|l| {
			let mut rdr = ReaderBuilder::new()
				.has_headers(false)
				.from_reader(l.as_bytes());
			rdr.records()
				.flat_map(|r| {
					r.unwrap()
						.iter()
						.map(|entry| entry.parse::<u32>().unwrap())
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	let other_tickets = sections
		.next()
		.unwrap()
		.lines()
		.skip(1)
		.map(|l| {
			let mut rdr = ReaderBuilder::new()
				.has_headers(false)
				.from_reader(l.as_bytes());
			rdr.records()
				.flat_map(|r| {
					r.unwrap()
						.iter()
						.map(|entry| entry.parse::<u32>().unwrap())
						.collect::<Vec<_>>()
				})
				.collect::<Vec<_>>()
		})
		.collect::<Vec<_>>();

	Tickets {
		fields,
		my_ticket,
		other_tickets,
	}
}

fn main() {
	let input = read_to_string("input/day16/1.txt").unwrap();
	let tickets = parse_to_struct(&input);

	let invalid_tickets = tickets
		.other_tickets
		.iter()
		.map(|t| {
			t.iter().find(|num| {
				tickets
					.fields
					.iter()
					.all(|field| !field.range1.contains(num) && !field.range2.contains(num))
			})
		})
		.collect::<Vec<_>>();

	let error_rate = invalid_tickets.iter().filter_map(|x| *x).sum::<u32>();

	println!("Part 1: {}", error_rate);

	let valid_tickets = tickets
		.other_tickets
		.iter()
		.zip(invalid_tickets.iter())
		.filter(|(_, i)| i.is_none())
		.map(|(t, _)| t)
		.chain(once(&tickets.my_ticket))
		.collect::<Vec<_>>();

	let mut allowed = (0..tickets.my_ticket.len())
		.map(|i| {
			(
				i,
				tickets
					.fields
					.iter()
					.enumerate()
					.filter(|(_, field)| {
						valid_tickets.iter().all(|ticket| {
							field.range1.contains(&ticket[i]) || field.range2.contains(&ticket[i])
						})
					})
					.map(|(i, _)| i)
					.collect::<Vec<_>>(),
			)
		})
		.collect::<Vec<_>>();
	allowed.sort_unstable_by(|a, b| a.1.len().cmp(&b.1.len()));

	let mut item = Vec::new();
	let mut field = Vec::new();
	for (index, possibilities) in allowed {
		item.push(index);
		for p in possibilities {
			if !field.contains(&p) {
				field.push(p);
			}
		}
	}

	let mut associations = item.iter().zip(field.iter()).collect::<Vec<_>>();
	associations.sort_unstable();
	let associations = associations
		.iter()
		.map(|(_, field)| field)
		.collect::<Vec<_>>();

	println!("{:?}", associations);

	let part_2 = tickets
		.my_ticket
		.iter()
		.enumerate()
		.filter(|(i, &num)| {
			let &&index = associations[*i];
			tickets.fields[index as usize].name.starts_with("departure")
		})
		.map(|(_, &n)| n as usize)
		.product::<usize>();

	println!("Part 2: {}", part_2);
}

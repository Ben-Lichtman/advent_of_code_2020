use nom::{
	branch::alt,
	bytes::complete::tag,
	character::complete::{alpha0, char, digit1},
	combinator::{map, map_res, opt, recognize},
	multi::many0,
	sequence::{terminated, tuple},
	IResult,
};

use petgraph::{graphmap::DiGraphMap, EdgeDirection};

use std::{
	collections::{HashSet, VecDeque},
	fs::read_to_string,
	str::FromStr,
};

struct GraphWalker<'a> {
	g: &'a DiGraphMap<&'a str, u32>,
	q: VecDeque<(u32, &'a str)>,
	seen: HashSet<&'a str>,
}

impl<'a> GraphWalker<'a> {
	fn new(g: &'a DiGraphMap<&'a str, u32>, start: &'a str) -> Self {
		let mut q = VecDeque::new();
		q.push_front((1, start));
		Self {
			g,
			q,
			seen: HashSet::new(),
		}
	}

	fn solve_1(&mut self) -> u32 {
		while let Some((_, name)) = self.q.pop_front() {
			if self.seen.insert(name) {
				for n in self.g.neighbors_directed(name, EdgeDirection::Incoming) {
					self.q.push_back((0, n));
				}
			}
		}
		(self.seen.len() - 1) as u32
	}

	fn solve_2(&mut self) -> u32 {
		let mut count = 0;

		while let Some((n, name)) = self.q.pop_front() {
			count += n;
			for (_, to, w) in self.g.edges(name) {
				self.q.push_back((n * w, to));
			}
		}
		count - 1
	}
}

fn parse_integer<T>(input: &str) -> IResult<&str, T>
where
	T: FromStr,
{
	map_res(
		recognize(tuple((opt(char('-')), digit1))),
		FromStr::from_str,
	)(input)
}

fn bag_name(i: &str) -> IResult<&str, &str> {
	tuple((
		recognize(tuple((alpha0, char(' '), alpha0))),
		char(' '),
		tag("bag"),
		opt(char('s')),
	))(i)
	.and_then(|(i, (name, ..))| Ok((i, name)))
}

fn parse_contained(i: &str) -> IResult<&str, (u32, &str)> {
	tuple((parse_integer, char(' '), bag_name))(i)
		.and_then(|(i, (num, _, name))| Ok((i, (num, name))))
}

fn parse_bags(i: &str) -> IResult<&str, (&str, Vec<(u32, &str)>)> {
	tuple((
		bag_name,
		tag(" contain "),
		alt((
			map(tag("no other bags."), |_| Vec::new()),
			many0(terminated(parse_contained, alt((tag("."), tag(", "))))),
		)),
	))(i)
	.and_then(|(i, (bag, _, links))| Ok((i, (bag, links))))
}

fn parse_to_vectors(input: &str) -> Vec<(&str, Vec<(u32, &str)>)> {
	input
		.lines()
		.map(|l| {
			parse_bags(l)
				.map(|(_, (name, v))| (name, v.into_iter().collect::<Vec<_>>()))
				.unwrap()
		})
		.collect::<Vec<_>>()
}

fn main() {
	let input = read_to_string("input/day7/1.txt").unwrap();

	let bags = parse_to_vectors(&input);

	let edges = bags
		.into_iter()
		.flat_map(|(outside, v)| {
			v.into_iter()
				.map(move |(num, inside)| (outside, inside, num))
		})
		.collect::<Vec<_>>();

	let graph = DiGraphMap::<_, _>::from_edges(edges.into_iter());

	let mut walker = GraphWalker::new(&graph, "shiny gold");

	let sol_1 = walker.solve_1();

	println!("Part 1: {}", sol_1);

	let mut walker = GraphWalker::new(&graph, "shiny gold");

	let sol_2 = walker.solve_2();

	println!("Part 2: {}", sol_2);
}

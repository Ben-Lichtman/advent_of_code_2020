use nom::{
	branch::{alt, permutation},
	bytes::complete::tag,
	character::complete::{char, digit1, multispace0, none_of, one_of},
	combinator::{all_consuming, map, map_res, opt, recognize},
	multi::{count, many0},
	sequence::tuple,
	IResult,
};

use std::{fs::read_to_string, str::FromStr};

#[derive(Debug)]
enum Height {
	Cm(u32),
	Inch(u32),
}

#[derive(Debug)]
enum EyeColour {
	Amb,
	Blu,
	Brn,
	Gry,
	Grn,
	Hzl,
	Oth,
}

#[derive(Debug)]
struct Passport {
	// (Birth Year)
	byr: u32,
	// (Issue Year)
	iyr: u32,
	// (Expiration Year)
	eyr: u32,
	// (Height)
	hgt: Height,
	// (Hair Color)
	hcl: String,
	// (Eye Color)
	ecl: EyeColour,
	// (Passport ID)
	pid: String,
	// (Country ID)
	cid: Option<String>,
}

impl Passport {
	fn validate_2(&self) -> bool {
		1920 <= self.byr
			&& self.byr <= 2002
			&& 2010 <= self.iyr
			&& self.iyr <= 2020
			&& 2020 <= self.eyr
			&& self.eyr <= 2030
			&& match self.hgt {
				Height::Cm(h) => 150 <= h && h <= 193,
				Height::Inch(h) => 59 <= h && h <= 76,
			}
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

fn parse_value(key: &'static str) -> impl FnMut(&str) -> IResult<&str, &str> {
	move |i| {
		tuple((
			tag(key),
			char(':'),
			recognize(many0(none_of(" \n"))),
			multispace0,
		))(i)
		.and_then(|(i, (_, _, s, _))| Ok((i, s)))
	}
}

fn parse_passport_elements(
	i: &str,
) -> IResult<&str, (&str, &str, &str, &str, &str, &str, &str, Option<&str>)> {
	permutation((
		parse_value("byr"),
		parse_value("iyr"),
		parse_value("eyr"),
		parse_value("hgt"),
		parse_value("hcl"),
		parse_value("ecl"),
		parse_value("pid"),
		opt(parse_value("cid")),
	))(i)
}

fn parse_height(i: &str) -> IResult<&str, Height> {
	alt((
		map(tuple((parse_integer, tag("cm"))), |(num, _)| {
			Height::Cm(num)
		}),
		map(tuple((parse_integer, tag("in"))), |(num, _)| {
			Height::Inch(num)
		}),
	))(i)
}

fn parse_hair_colour(i: &str) -> IResult<&str, &str> {
	recognize(tuple((
		char('#'),
		all_consuming(count(one_of("0123456789abcdef"), 6)),
	)))(i)
}

fn parse_eye_colour(i: &str) -> IResult<&str, EyeColour> {
	alt((
		map(tag("amb"), |_| EyeColour::Amb),
		map(tag("blu"), |_| EyeColour::Blu),
		map(tag("brn"), |_| EyeColour::Brn),
		map(tag("gry"), |_| EyeColour::Gry),
		map(tag("grn"), |_| EyeColour::Grn),
		map(tag("hzl"), |_| EyeColour::Hzl),
		map(tag("oth"), |_| EyeColour::Oth),
	))(i)
}

fn parse_pid(i: &str) -> IResult<&str, &str> {
	all_consuming(recognize(count(one_of("0123456789"), 9)))(i)
}

fn parse_passport(i: &str) -> IResult<&str, Passport> {
	permutation((
		parse_value("byr"),
		parse_value("iyr"),
		parse_value("eyr"),
		parse_value("hgt"),
		parse_value("hcl"),
		parse_value("ecl"),
		parse_value("pid"),
		opt(parse_value("cid")),
	))(i)
	.and_then(|(input, (byr, iyr, eyr, hgt, hcl, ecl, pid, cid))| {
		let byr = parse_integer(&byr)?.1;
		let iyr = parse_integer(&iyr)?.1;
		let eyr = parse_integer(&eyr)?.1;
		let hgt = parse_height(&hgt)?.1;
		let hcl = parse_hair_colour(hcl)?.1.to_string();
		let ecl = parse_eye_colour(ecl)?.1;
		let pid = parse_pid(pid)?.1.to_string();
		let cid = cid.map(|s| s.to_string());

		Ok((
			input,
			Passport {
				// (Birth Year)
				byr,
				// (Issue Year)
				iyr,
				// (Expiration Year)
				eyr,
				// (Height)
				hgt,
				// (Hair Color)
				hcl,
				// (Eye Color)
				ecl,
				// (Passport ID)
				pid,
				// (Country ID)
				cid,
			},
		))
	})
}

fn parse_input(file: &str) -> Vec<String> {
	let input = read_to_string(file).unwrap();

	let processed = input
		.split("\n\n")
		.map(|x| x.to_string())
		.collect::<Vec<_>>();

	processed
}

fn main() {
	let entries = parse_input("input/day4/1.txt");

	let parsed_stage_1 = entries
		.iter()
		.map(|l| parse_passport_elements(l))
		.collect::<Vec<_>>();

	let count_1 = parsed_stage_1.iter().filter(|x| x.is_ok()).count();

	println!("Part 1: {}", count_1);

	let parsed_stage_2 = entries
		.iter()
		.map(|l| parse_passport(l))
		.collect::<Vec<_>>();

	let count_2 = parsed_stage_2
		.iter()
		.filter(|x| x.is_ok() && x.as_ref().unwrap().1.validate_2())
		.count();

	println!("Part 2: {}", count_2);
}

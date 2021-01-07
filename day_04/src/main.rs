use common::load_data_full;
use nom::bytes::complete::tag;
use nom::character::complete::multispace0;
use nom::combinator::map_parser;
use nom::error::{ErrorKind, ParseError};
use nom::multi::fold_many1;
use nom::sequence::{separated_pair, terminated};
use nom::{AsChar, IResult, InputTakeAtPosition};
use regex::Regex;
use std::collections::HashMap;
use std::fmt::Debug;

fn main() {
    let input: String = load_data_full("data/day_04.txt");
    println!("Day 04 Part 1: {}", part_1(&input));

    println!("Day 04 Part 2: {}", part_2(&input));
}

fn get_validator(key: &str) -> Box<dyn Validator> {
    match key {
        "byr" => Box::new(RangeValidator {
            low: 1920,
            high: 2002,
        }),
        "iyr" => Box::new(RangeValidator {
            low: 2010,
            high: 2020,
        }),
        "eyr" => Box::new(RangeValidator {
            low: 2020,
            high: 2030,
        }),
        "hgt" => Box::new(HeightValidator {}),
        "hcl" => Box::new(RegexValidator {
            regex: Regex::new(r"^#[0-9a-f]{6}$").unwrap(),
        }),
        "ecl" => Box::new(RegexValidator {
            regex: Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap(),
        }),
        "pid" => Box::new(RegexValidator {
            regex: Regex::new(r"^\d{9}$").unwrap(),
        }),
        "cid" => Box::new(StaticValidator { value: true }),
        _ => Box::new(StaticValidator { value: false }),
    }
}

fn part_1(input: &str) -> usize {
    let passwords = get_passwords(&input);
    passwords
        .iter()
        .filter(|&p| p.has_required_fields())
        .count()
}

fn part_2(input: &str) -> usize {
    let passwords = get_passwords(&input);
    passwords
        .iter()
        .filter(|&p| p.has_required_fields())
        .filter(|&p| p.is_valid())
        .count()
}

#[derive(Copy, Clone)]
struct Entry<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

trait Validator: Debug {
    fn is_valid(&self, value: &str) -> bool;
}

#[derive(Debug)]
struct StaticValidator {
    value: bool,
}

impl Validator for StaticValidator {
    fn is_valid(&self, _: &str) -> bool {
        return self.value;
    }
}

#[derive(Debug)]
struct RangeValidator {
    low: u32,
    high: u32,
}

impl Validator for RangeValidator {
    fn is_valid(&self, value: &str) -> bool {
        let re = Regex::new(r"\d{4}").unwrap();
        if !re.is_match(value) {
            return false;
        }
        let value: u32 = match value.parse() {
            Ok(v) => v,
            Err(_) => return false,
        };
        self.low <= value && value <= self.high
    }
}

#[derive(Debug)]
struct HeightValidator {}

impl Validator for HeightValidator {
    fn is_valid(&self, value: &str) -> bool {
        let re = Regex::new(r"(\d+)(cm|in)").unwrap();
        let captures = match re.captures(value) {
            Some(captures) => captures,
            None => return false,
        };
        let val = captures.get(1).unwrap().as_str().parse().unwrap();
        let (min, max) = match captures.get(2).unwrap().as_str() {
            "cm" => (150, 193),
            _ => (59, 76),
        };
        min <= val && val <= max
    }
}

#[derive(Debug)]
struct RegexValidator {
    regex: Regex,
}

impl Validator for RegexValidator {
    fn is_valid(&self, value: &str) -> bool {
        self.regex.is_match(value)
    }
}

fn key_value(input: &str) -> IResult<&str, Entry> {
    separated_pair(allowed_chars, tag(":"), allowed_chars)(input)
        .map(|(rest, (key, value))| (rest, Entry { key, value }))
}

fn non_empty_line(input: &str) -> IResult<&str, &str> {
    match input.find("\n\n") {
        Some(idx) => Ok((&input[idx + 2..], &input[..idx])),
        None => Ok(("", input)),
    }
}

fn batch(input: &str) -> IResult<&str, Vec<Entry>> {
    map_parser(non_empty_line, |batch| password_batch(batch))(input)
}

pub fn allowed_chars<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| {
            let char = item.as_char();
            !(char.is_alphanum() || char == '#')
        },
        ErrorKind::AlphaNumeric,
    )
}

fn password_batch(input: &str) -> IResult<&str, Vec<Entry>> {
    fold_many1(
        terminated(key_value, multispace0),
        Vec::new(),
        |mut acc, item| {
            acc.push(item);
            acc
        },
    )(input)
}

fn password_maps(input: &str) -> IResult<&str, Vec<Vec<Entry>>> {
    fold_many1(batch, Vec::new(), |mut acc, item| {
        acc.push(item);
        acc
    })(input)
}

#[derive(Debug)]
struct Password<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> Password<'a> {
    fn new(entries: Vec<Entry<'a>>) -> Self {
        let mut map = HashMap::new();
        for entry in entries {
            map.insert(entry.key, entry.value);
        }
        Self { fields: map }
    }

    fn has_required_fields(&self) -> bool {
        let expected_keys = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        expected_keys
            .into_iter()
            .all(|key| self.fields.contains_key(key))
    }

    fn is_valid(&self) -> bool {
        self.fields
            .iter()
            .all(|(&key, &value)| get_validator(key).is_valid(value))
    }
}

fn get_passwords(input: &str) -> Vec<Password> {
    let (rem, passwords) = password_maps(input).unwrap();
    assert_eq!(rem, "");

    passwords
        .into_iter()
        .map(|entries| Password::new(entries))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input_1() -> String {
        let data = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";
        data.to_owned()
    }

    fn get_input_2() -> String {
        let data = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007

pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        data.to_owned()
    }

    #[test]
    fn test_part_1() {
        let input = get_input_1();
        assert_eq!(part_1(&input), 2)
    }

    #[test]
    fn test_part_2() {
        let input = get_input_2();
        assert_eq!(part_2(&input), 4)
    }
}

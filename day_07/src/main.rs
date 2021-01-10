use anyhow::{anyhow, Result};
use common::load_data_full;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::{HashMap, HashSet};

fn main() {
    let input: String = load_data_full("data/day_07.txt");
    println!("Day 07 Part 1: {}", part_1(&input));
    println!("Day 07  Part 2: {}", part_2(&input));
}

fn parse_color(input: &str) -> Result<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) bags?").unwrap();
    }
    let captures: Captures = RE.captures(input).ok_or(anyhow!("Not a valid bag"))?;
    Ok(captures.get(1).unwrap().as_str())
}

fn parse_rule(input: &str) -> Result<(&str, Vec<(usize, &str)>)> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(.+) contain (.+)\.").unwrap();
        static ref CONTENT: Regex = Regex::new(r"(\d+) (.+)").unwrap();
    }
    let captures: Captures = RE.captures(input).ok_or(anyhow!("Not a valid rule"))?;
    let color = parse_color(captures.get(1).unwrap().as_str())?;
    let contents = captures.get(2).unwrap().as_str();
    if contents == "no other bags" {
        return Ok((color, vec![]));
    };
    let contents: Result<Vec<_>> = contents
        .split(", ")
        .map(|desc| {
            let desc = CONTENT
                .captures(desc)
                .ok_or(anyhow!("Invalid description"))?;
            Ok((
                desc.get(1).unwrap().as_str().parse().unwrap(),
                parse_color(desc.get(2).unwrap().as_str())?,
            ))
        })
        .collect();
    Ok((color, contents?))
}

fn parse_rules(input: &str) -> Result<HashMap<&str, Vec<(usize, &str)>>> {
    input.lines().map(|line| parse_rule(line)).collect()
}

fn part_1(input: &str) -> usize {
    let mut parsed = parse_rules(input).unwrap();
    let mut valid = HashSet::new();
    valid.insert("shiny gold");

    loop {
        let len_before = valid.len();
        parsed = parsed
            .into_iter()
            .filter(|(bag, contents)| {
                if contents
                    .into_iter()
                    .any(|&(_, color)| valid.contains(color))
                {
                    valid.insert(*bag);
                    return false;
                };
                true
            })
            .collect();

        if len_before == valid.len() {
            break;
        }
    }

    valid.len() - 1
}

fn count_recursive(color: &str, rules: &HashMap<&str, Vec<(usize, &str)>>) -> Result<usize> {
    let rule = rules.get(color).ok_or(anyhow!("Rule does not exist"))?;
    rule.into_iter()
        .map(|&(count, name)| Ok(count + count * count_recursive(name, rules)?))
        .sum()
}

fn part_2(input: &str) -> usize {
    let rules = parse_rules(input).unwrap();
    count_recursive("shiny gold", &rules).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        let data = "\
light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
        data.to_owned()
    }

    fn get_input_2() -> String {
        let data = "\
shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";
        data.to_owned()
    }

    #[test]
    fn test_parsing() {
        let input = get_input();
        let parsed = input
            .lines()
            .map(|line| parse_rule(line))
            .collect::<Result<Vec<_>>>()
            .unwrap();
        assert_eq!(
            parsed,
            vec![
                ("light red", vec![(1, "bright white"), (2, "muted yellow")]),
                (
                    "dark orange",
                    vec![(3, "bright white"), (4, "muted yellow")]
                ),
                ("bright white", vec![(1, "shiny gold")]),
                ("muted yellow", vec![(2, "shiny gold"), (9, "faded blue")]),
                ("shiny gold", vec![(1, "dark olive"), (2, "vibrant plum")]),
                ("dark olive", vec![(3, "faded blue"), (4, "dotted black")]),
                ("vibrant plum", vec![(5, "faded blue"), (6, "dotted black")]),
                ("faded blue", vec![]),
                ("dotted black", vec![]),
            ]
        )
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(part_1(&input), 4)
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        assert_eq!(
            count_recursive("shiny gold", &parse_rules(&input).unwrap()).unwrap(),
            32
        );
        let input_2 = get_input_2();
        assert_eq!(
            count_recursive("shiny gold", &parse_rules(&input_2).unwrap()).unwrap(),
            126
        );
    }
}

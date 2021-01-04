use anyhow::{anyhow, Result};
use itertools::Itertools;
use regex::Regex;

use common::load_data;

fn main() -> Result<()> {
    let data: Vec<String> = load_data("data/day_02.txt");
    println!("Part 1: {}", count_passwords(&data, count_matcher)?);
    println!("Part 2: {}", count_passwords(&data, position_matcher)?);
    Ok(())
}

fn count_matcher(low: usize, high: usize, needle: char, password: &str) -> bool {
    let matches = password.matches(needle).count();
    low <= matches && matches <= high
}

fn position_matcher(low: usize, high: usize, needle: char, password: &str) -> bool {
    password
        .chars()
        .enumerate()
        .filter(|&(i, _)| i + 1 == low || i + 1 == high)
        .filter(|&(_, c)| c == needle)
        .count()
        == 1
}

fn count_passwords<F>(lines: &Vec<String>, matcher: F) -> Result<usize>
where
    F: Fn(usize, usize, char, &str) -> bool,
{
    let re = Regex::new(r"(\d+)-(\d+) ([a-z]): ([a-z]+)").unwrap();
    lines
        .into_iter()
        .map(|v| re.captures(v).ok_or(anyhow!("Did not match regex")))
        .filter_map_ok(|c| {
            let low: usize = c.get(1).unwrap().as_str().parse().unwrap();
            let high: usize = c.get(2).unwrap().as_str().parse().unwrap();
            let character: char = c.get(3).unwrap().as_str().parse().unwrap();
            let password: String = c.get(4).unwrap().as_str().parse().unwrap();
            match matcher(low, high, character, &password) {
                false => None,
                _ => Some(()),
            }
        })
        .try_fold(0, |a, v| v.map(|_| a + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        let data = vec!["1-3 a: abcde", "1-3 b: cdefg", "2-9 c: ccccccccc"];
        data.into_iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(count_passwords(&input, count_matcher).unwrap(), 2);
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        assert_eq!(count_passwords(&input, position_matcher).unwrap(), 1);
    }
}

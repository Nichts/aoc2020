use common::{load_data_full, Blocks};
use std::collections::HashSet;
use std::iter::FromIterator;

fn main() {
    let input: String = load_data_full("data/day_06.txt");
    println!("Day 06 Part 1: {}", part_1(&input));
    println!("Day 06 Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> usize {
    input
        .blocks()
        .map(|block| {
            block.chars().fold(HashSet::new(), |mut acc, c| {
                if c.is_alphabetic() {
                    acc.insert(c);
                }
                acc
            })
        })
        .map(|set| set.len())
        .sum()
}

fn part_2(input: &str) -> usize {
    input
        .blocks()
        .map(|block| {
            let mut lines = block.lines();
            let mut all = HashSet::new();
            if let Some(line) = lines.next() {
                all.extend(line.chars())
            };
            lines.fold(all, |acc, line| {
                let chars = HashSet::from_iter(line.chars());
                acc.intersection(&chars).cloned().collect()
            })
        })
        .map(|set| set.len())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        let data = "\
abc

a
b
c

ab
ac

a
a
a
a

b";
        data.to_owned()
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(part_1(&input), 11)
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
        assert_eq!(part_2(&input), 6)
    }
}

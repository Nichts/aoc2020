use common::load_data;
use nom::bytes::complete::take_while;
use nom::combinator::map_parser;
use nom::IResult;

fn main() {
    let input: Vec<String> = load_data("data/day_05.txt");
    println!("Day 05 Part 1: {}", part_1(&input));
    println!("Day 05 Part 2: {}", part_2(&input));
}

fn part_1(input: &Vec<String>) -> u32 {
    input.iter().map(|line| parse(line)).max().unwrap()
}

fn part_2(input: &Vec<String>) -> u32 {
    let mut seats: Vec<_> = input.iter().map(|line| parse(line)).collect();
    seats.sort();
    seats
        .windows(2)
        .filter_map(|input| {
            if let [first, second] = input {
                if first + 2 == *second {
                    return Some(first + 1);
                };
            }
            None
        })
        .next()
        .unwrap()
}

fn parser(input: &str) -> IResult<&str, u32> {
    map_parser(
        take_while(move |c| c == 'F' || c == 'B' || c == 'L' || c == 'R'),
        move |input: &str| {
            let exp = input.len() - 1;
            let res = input.char_indices().fold(0, |acc, (index, char)| {
                if char == 'B' || char == 'R' {
                    acc + (1 << (exp - index))
                } else {
                    acc
                }
            });
            Ok(("", res))
        },
    )(input)
}

fn parse(input: &str) -> u32 {
    parser(input).unwrap().1
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        let data = "\
FBFBBFFRLR
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";
        data.lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
        assert_eq!(part_1(&input), 820)
    }
}

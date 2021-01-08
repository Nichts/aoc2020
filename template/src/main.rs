use common::load_data;

fn main() {
    // let input: String = load_data_full("data/day_0.txt");
    // println!("Day  Part 1: {}", part_1(&input));
    // println!("Day  Part 2: {}", part_2(&input));
}

fn part_1(input: &str) -> u32 {
    input.len() as u32
}

fn part_2(input: &str) -> u32 { input.len() as u32 }

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> String {
        let data = "";
        data.to_owned()
    }

    #[test]
    fn test_part_1() {
        let input = get_input();
    }

    #[test]
    fn test_part_2() {
        let input = get_input();
    }
}

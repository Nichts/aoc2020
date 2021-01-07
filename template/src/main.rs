fn main() {
    let input: String = load_data("data/day_0.txt");
    println!("Day  Part 1: {}", part_1(&input));
    println!("Day  Part 2: {}", part_2(&input));
}

fn part_1(_: &str) -> u32 {
    0
}

fn part_2(_: &str) -> u32 {
    0
}

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

use common::load_data;

fn main() {
    let input: Vec<String> = load_data("data/day_03.txt");
    let map = Map::new(input);

    println!("Part 1: {}", map.count_trees(Slope::new(3, 1)));

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let slopes = slopes
        .into_iter()
        .map(|(right, down)| Slope::new(right, down))
        .collect();

    println!(
        "Part 2: {}",
        map.count_all_trees(slopes).into_iter().product::<usize>()
    );
}

struct Map {
    rows: Vec<String>,
}

struct Slope {
    pub right: usize,
    pub down: usize,
}

impl Slope {
    fn new(right: usize, down: usize) -> Slope {
        Self { right, down }
    }
}

impl Map {
    fn new(rows: Vec<String>) -> Self {
        Self { rows }
    }

    fn is_tree_in_row(row: &str, y: usize) -> bool {
        let y = y % row.len();
        row.as_bytes()[y] == '#' as u8
    }

    fn count_trees(&self, slope: Slope) -> usize {
        self.rows
            .iter()
            .step_by(slope.down)
            .enumerate() // Enumerating before skip so we don't have to add 1 to the index later
            .skip(1)
            .filter(|(idx, row)| Self::is_tree_in_row(row, idx * slope.right))
            .count()
    }

    fn count_all_trees(&self, slopes: Vec<Slope>) -> Vec<usize> {
        slopes
            .into_iter()
            .map(|slope| self.count_trees(slope))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> Vec<String> {
        let data = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
        data.lines().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_part_01() {
        let map = Map::new(get_input());
        assert_eq!(map.count_trees(Slope::new(3, 1)), 7)
    }

    #[test]
    fn test_part_02() {
        let map = Map::new(get_input());
        let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
        let slopes = slopes
            .into_iter()
            .map(|(right, down)| Slope::new(right, down))
            .collect();
        assert_eq!(
            map.count_all_trees(slopes).into_iter().product::<usize>(),
            336
        )
    }
}

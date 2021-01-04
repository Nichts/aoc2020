use anyhow::{anyhow, Result};
use common::load_data;
use itertools::Itertools;

fn main() -> Result<()> {
    let mut data: Vec<u32> = load_data("data/day_01.txt");
    data.sort();
    let res = find_combination(&data, 2)?;
    println!("Result: {}", res.into_iter().product::<u32>());
    let res = find_combination(&data, 3)?;
    println!("Result: {}", res.into_iter().product::<u32>());
    Ok(())
}

fn find_combination(data: &Vec<u32>, n: usize) -> Result<Vec<u32>> {
    data.into_iter()
        .cloned()
        .combinations(n)
        .find(|e| e.into_iter().fold(0, |a, &b| -> u32 { a + b }) == 2020)
        .ok_or_else(|| anyhow!("Value not found"))
}

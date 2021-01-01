use common::load_data;
use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let mut data: Vec<i32> = load_data("data/day_01.txt");
    data.sort();
    let (a, b) = find_pair(data)?;
    println!("Result: {}", a * b);
    Ok(())
}

fn find_pair(data: Vec<i32>) -> Result<(i32, i32)> {
    let mut i = 0;
    let mut j = data.len() - 1;

    while i < j {
        let res = data[i] + data[j];
        if res == 2020 {
            return Ok((data[i], data[j]))
        } else if res > 2020 {
            j -= 1;
        } else {
            i += 1;
        }
    };
    Err(anyhow!("Value not found"))
}
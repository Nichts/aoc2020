use std::fmt::Debug;
use std::fs;
use std::str::FromStr;

pub fn load_data<F: FromStr>(file: &str) -> Vec<F>
where
    F::Err: Debug,
{
    let data = fs::read_to_string(file).unwrap();
    data.lines()
        .map(FromStr::from_str)
        .collect::<Result<Vec<F>, _>>()
        .unwrap()
}

pub fn load_data_full<F: FromStr>(file: &str) -> F
where
    F::Err: Debug,
{
    let data = fs::read_to_string(file).unwrap();
    FromStr::from_str(&data).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::load_data;

    #[test]
    fn it_works() {
        let data: Vec<String> = load_data("../data/day_01.txt");
        assert_eq!(data[3], "1791");
        let data: Vec<i32> = load_data("../data/day_01.txt");
        assert_eq!(data[3], 1791)
    }
}

use std::fmt::Debug;
use std::fs;
use std::str::{FromStr, SplitTerminator};

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

pub trait Blocks<'a> {
    fn blocks(&'a self) -> MapBlock<'a>;
}

fn map_block(block: &str) -> &str {
    let l = block.len();
    if l > 0 && block.as_bytes()[l - 1] == b'\n' {
        &block[0..l - 1]
    } else {
        block
    }
}

pub struct MapBlock<'a> {
    inner: SplitTerminator<'a, &'a str>,
}

impl<'a> Iterator for MapBlock<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(map_block)
    }
}

impl<'a> Blocks<'a> for str {
    fn blocks(&'a self) -> MapBlock<'a> {
        MapBlock {
            inner: self.split_terminator("\n\n"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{load_data, Blocks};

    #[test]
    fn test_load() {
        let data: Vec<String> = load_data("../data/day_01.txt");
        assert_eq!(data[3], "1791");
        let data: Vec<i32> = load_data("../data/day_01.txt");
        assert_eq!(data[3], 1791)
    }

    #[test]
    fn test_blocks() {
        let data = "\
abc
def

123
";
        let mut iter = data.blocks();
        assert_eq!(iter.next().unwrap(), "abc\ndef");
        assert_eq!(iter.next().unwrap(), "123");
        assert!(iter.next().is_none());
    }
}

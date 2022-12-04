use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    Solution::try_from(reader).context("reading from '{filename}'")
}

#[derive(Debug)]
pub struct Solution {}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        // Implement for problem
        Ok(0)
    }

    pub fn answer_part2(&self) -> Result<u64> {
        // Implement for problem
        Ok(0)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let solution = Self {};
        for _line in reader.lines() {
            // Implement for problem
        }
        Ok(solution)
    }
}

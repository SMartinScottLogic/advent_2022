use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Solution {}

impl utils::Solution for Solution {
    type Result = anyhow::Result<u64>;
    fn analyse(&mut self) {}

    fn answer_part1(&self) -> Self::Result {
        // Implement for problem
        Ok(0)
    }

    fn answer_part2(&self) -> Self::Result {
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

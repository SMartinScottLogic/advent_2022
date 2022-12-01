use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let solution = Solution::try_from(reader).context("")?;

    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    elves: Vec<Vec<u64>>,
}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        self.elves
            .iter()
            .map(|elf| elf.iter().sum::<u64>())
            .max()
            .ok_or_else(|| anyhow!("failed to get maximum"))
    }

    pub fn answer_part2(&self) -> Result<u64> {
        let answer = self
            .elves
            .iter()
            .map(|elf| elf.iter().sum::<u64>())
            .sorted()
            .rev()
            .take(3)
            .sum::<u64>();
        Ok(answer)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            elves: vec![Vec::new()],
        };
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.is_empty() {
                solution.elves.push(Vec::new());
            } else {
                let calories = line.parse().unwrap();
                solution.elves.last_mut().unwrap().push(calories);
            }
        }
        Ok(solution)
    }
}

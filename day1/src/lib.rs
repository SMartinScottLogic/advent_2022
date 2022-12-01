use anyhow::Result;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    let mut solution = Solution::new();
    for line in reader.lines() {
        let line = line.unwrap();
        solution.add_entry(&line);
    }

    Ok(solution)
}

#[derive(Debug)]
pub struct Solution {
    elves: Vec<Vec<u64>>,
}

impl Solution {
    fn new() -> Self {
        Self {
            elves: vec![Vec::new()],
        }
    }

    fn add_entry(&mut self, entry: &str) {
        if entry.trim().is_empty() {
            self.elves.push(Vec::new());
        } else {
            let calories = entry.parse().unwrap();
            self.elves.last_mut().unwrap().push(calories);
        }
    }

    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        let mut answer = 0;
        for elf in &self.elves {
            let a = elf.iter().sum();
            if a > answer {
                answer = a;
            }
        }
        Ok(answer)
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

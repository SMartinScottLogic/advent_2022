use anyhow::{Context, Result};
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    Solution::try_from(reader).context("reading from '{filename}'")
}

#[derive(Debug)]
pub struct Solution {
    assignments: Vec<(Sections, Sections)>,
}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        let mut answer = 0;
        for (a, b) in &self.assignments {
            if a.wholely_contains(b) || b.wholely_contains(a) {
                answer += 1;
            }
        }
        Ok(answer)
    }

    pub fn answer_part2(&self) -> Result<u64> {
        let mut answer = 0;
        for (a, b) in &self.assignments {
            if a.overlaps(b) {
                answer += 1;
            }
        }
        Ok(answer)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            assignments: Vec::new(),
        };
        for line in reader.lines() {
            let line = line?;
            let p = line
                .split(',')
                .take(2)
                .map(Sections::from)
                .collect_tuple()
                .unwrap();
            solution.assignments.push(p);
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Sections {
    start: u64,
    end: u64,
}

impl From<&str> for Sections {
    fn from(input: &str) -> Self {
        let (start, end) = input
            .split('-')
            .take(2)
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        Self { start, end }
    }
}

impl Sections {
    fn wholely_contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        (self.start <= other.start && other.start <= self.end)
            || (other.start <= self.start && self.start <= other.end)
    }
}

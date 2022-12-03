use anyhow::{Context, Result};
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    Solution::try_from(reader).context("reading from '{filename}'")
}

#[derive(Debug)]
pub struct Solution {
    rucksacks: Vec<String>,
}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        let mut errors = Vec::new();
        for rucksack in &self.rucksacks {
            let (pocket1, pocket2) = rucksack.split_at(rucksack.len() / 2);
            let items1: HashSet<_> = pocket1.chars().collect();
            let items2: HashSet<_> = pocket2.chars().collect();
            let common = items1.intersection(&items2).next().unwrap().to_owned();
            errors.push(common);
        }
        println!("errors: {errors:?}");
        let scores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let answer = errors
            .iter()
            .map(|c| scores.find(*c).unwrap() as u64 + 1)
            .sum();
        Ok(answer)
    }

    pub fn answer_part2(&self) -> Result<u64> {
        let mut badges = Vec::new();
        let scores = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
        for group in self.rucksacks.chunks(3) {
            let all_chars: HashSet<_> = scores.chars().collect();
            let common = group
                .iter()
                .map(|rucksack| rucksack.chars().collect::<HashSet<_>>())
                .fold(all_chars, |acc, r| {
                    acc.intersection(&r).map(|c| c.to_owned()).collect()
                });
            let common = common.iter().next().unwrap();
            badges.push(common.to_owned());
        }
        let answer = badges
            .iter()
            .map(|c| scores.find(*c).unwrap() as u64 + 1)
            .sum();
        Ok(answer)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            rucksacks: Vec::new(),
        };
        for line in reader.lines() {
            solution.rucksacks.push(line?);
        }
        Ok(solution)
    }
}

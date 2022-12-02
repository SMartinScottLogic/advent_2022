use anyhow::{Context, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load(filename: &str) -> anyhow::Result<Solution> {
    let file = File::open(filename)?;

    let reader = BufReader::new(file);
    Solution::try_from(reader).context("reading from '{filename}'")
}

#[derive(Debug)]
pub struct Solution {
    input: Vec<(String, String)>,
}

impl Solution {
    pub fn analyse(&mut self) {}

    pub fn answer_part1(&self) -> Result<u64> {
        let answer = self
            .input
            .iter()
            .map(|(a, b)| {
                let a = HandShape::from(a.as_str());
                let b = HandShape::from(b.as_str());
                Self::calculate_outcome_score(&a, &b) + Self::calculate_selected_score(&b)
            })
            .sum();
        // Implement for problem
        Ok(answer)
    }

    pub fn answer_part2(&self) -> Result<u64> {
        use HandShape::*;
        use Outcome::*;
        let answer = self
            .input
            .iter()
            .map(|(a, b)| {
                let a = HandShape::from(a.as_str());
                let b = Outcome::from(b.as_str());

                let selected = match (&a, b) {
                    (Rock, Lose) => Scissors,
                    (Rock, Draw) => Rock,
                    (Rock, Win) => Paper,
                    (Paper, Lose) => Rock,
                    (Paper, Draw) => Paper,
                    (Paper, Win) => Scissors,
                    (Scissors, Lose) => Paper,
                    (Scissors, Draw) => Scissors,
                    (Scissors, Win) => Rock,
                };
                Self::calculate_selected_score(&selected)
                    + Self::calculate_outcome_score(&a, &selected)
            })
            .sum();
        // Implement for problem
        Ok(answer)
    }

    fn calculate_selected_score(us: &HandShape) -> u64 {
        use HandShape::*;
        match us {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }
    fn calculate_outcome_score(opponent: &HandShape, us: &HandShape) -> u64 {
        use HandShape::*;
        match (opponent, us) {
            (Rock, Rock) => 3,
            (Rock, Paper) => 6,
            (Rock, Scissors) => 0,
            (Paper, Rock) => 0,
            (Paper, Paper) => 3,
            (Paper, Scissors) => 6,
            (Scissors, Rock) => 6,
            (Scissors, Paper) => 0,
            (Scissors, Scissors) => 3,
        }
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self { input: Vec::new() };
        for line in reader.lines() {
            let line = line?;
            let mut it = line.split_whitespace();
            let lhs = it.next().unwrap();
            let rhs = it.next().unwrap();
            solution.input.push((lhs.to_string(), rhs.to_string()));
        }
        Ok(solution)
    }
}

#[derive(Debug)]
enum HandShape {
    Rock,
    Scissors,
    Paper,
}

impl From<&str> for HandShape {
    fn from(i: &str) -> Self {
        match i {
            "A" => Self::Rock,
            "B" => Self::Paper,
            "C" => Self::Scissors,
            "X" => Self::Rock,
            "Y" => Self::Paper,
            "Z" => Self::Scissors,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    fn from(i: &str) -> Self {
        match i {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        }
    }
}

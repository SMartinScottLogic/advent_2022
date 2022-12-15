use std::{
    collections::{HashSet, VecDeque},
    io::{BufRead, BufReader},
};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Solution {
    input: String,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<usize>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        self.detect_distinct_window(4)
            .ok_or_else(|| anyhow!("nothing"))
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        self.detect_distinct_window(14)
            .ok_or_else(|| anyhow!("nothing"))
    }
}

impl Solution {
    fn detect_distinct_window(&self, window_len: usize) -> Option<usize> {
        let mut answer = None;
        let mut window = VecDeque::new();
        for (idx, c) in self.input.chars().enumerate() {
            window.push_back(c);
            if window.len() > window_len {
                window.pop_front();
            }
            let chars = window.iter().fold(HashSet::new(), |mut acc, v| {
                acc.insert(v);
                acc
            });
            if chars.len() == window_len && answer.is_none() {
                answer = Some(idx + 1);
                break;
            }
        }
        answer
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            input: "".to_string(),
        };
        for line in reader.lines() {
            solution.input = line?;
        }
        Ok(solution)
    }
}

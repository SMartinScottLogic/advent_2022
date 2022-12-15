use std::{
    collections::{BTreeMap, VecDeque},
    io::{BufRead, BufReader},
};

use log::debug;

#[derive(Debug)]
pub struct Solution {
    stacks: BTreeMap<usize, VecDeque<char>>,
    instructions: Vec<Instruction>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<String>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            for _i in 1..=instruction.count {
                let start_stack = stacks.entry(instruction.start).or_default();
                let c = start_stack.pop_front().unwrap();
                let end_stack = stacks.entry(instruction.end).or_default();
                end_stack.push_front(c);
            }
        }
        let answer = stacks
            .values()
            .map(|stack| stack.front().unwrap_or(&' '))
            .collect::<String>();
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut stacks = self.stacks.clone();
        for instruction in &self.instructions {
            let mut temp_stack = VecDeque::new();
            for _i in 1..=instruction.count {
                let start_stack = stacks.entry(instruction.start).or_default();
                let c = start_stack.pop_front().unwrap();
                temp_stack.push_front(c);
            }
            for _i in 1..=instruction.count {
                let c = temp_stack.pop_front().unwrap();
                let end_stack = stacks.entry(instruction.end).or_default();
                end_stack.push_front(c);
            }
        }
        let answer = stacks
            .values()
            .map(|stack| stack.front().unwrap_or(&' '))
            .collect::<String>();
        Ok(answer)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            stacks: BTreeMap::new(),
            instructions: Vec::new(),
        };
        let mut stage = 0;
        for line in reader.lines() {
            let line = line?;
            if line.trim().is_empty() {
                stage += 1;
                continue;
            }
            if stage == 0 {
                for (i, c) in line.chars().enumerate() {
                    if c.is_numeric() || c.is_whitespace() {
                        continue;
                    }
                    if i > 0 && (i - 1) % 4 == 0 {
                        let j = 1 + ((i - 1) / 4);
                        debug!("{j} {c} from {i}");

                        let stack = solution.stacks.entry(j).or_default();
                        stack.push_back(c);
                    }
                }
            } else {
                debug!("{line}");
                let mut i = line
                    .split_ascii_whitespace()
                    .filter(|s| s.starts_with(|c: char| c.is_numeric()));
                let count = i.next().unwrap().parse::<u64>().unwrap();
                let start = i.next().unwrap().parse::<usize>().unwrap();
                let end = i.next().unwrap().parse::<usize>().unwrap();
                solution
                    .instructions
                    .push(Instruction { count, start, end });
            }
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Instruction {
    count: u64,
    start: usize,
    end: usize,
}

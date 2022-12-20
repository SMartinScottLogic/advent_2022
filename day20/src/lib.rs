use std::{
    fmt::Debug,
    io::{BufRead, BufReader},
};

use log::debug;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    input: Vec<ResultType>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let num_inputs = self.input.len() as ResultType;
        let mut input = self
            .input
            .iter()
            .enumerate()
            .map(|(i, v)| (i, *v, false))
            .collect::<Vec<_>>();
        loop {
            let found = input
                .iter()
                .enumerate()
                .filter(|(_, (_, _, f))| !f)
                .min_by_key(|(_, (i, _, _))| i);
            if found.is_none() {
                break;
            }
            let (idx, (original_idx, value, f)) = found.unwrap();
            let (idx, (original_idx, value, _)) = (idx, (*original_idx, *value, *f));
            let mut target = idx as ResultType + value;
            while target <= 0 {
                target += num_inputs - 1;
            }
            target %= num_inputs;
            let target = target as usize;
            debug!("{:?} {}", found, target);
            input.remove(idx);

            let mut next_input = Vec::new();
            debug!("  {input:?} {target} {idx}");
            for i in input.iter().take(target) {
                next_input.push(*i);
            }
            debug!("    {next_input:?}");
            let d = (original_idx, value, true);
            next_input.push(d);
            debug!("    {next_input:?}");
            for i in input.iter().skip(target) {
                next_input.push(*i);
            }
            debug!("    {next_input:?}");
            input.clear();
            input.append(&mut next_input);
            debug!(
                "next input: {:?}",
                input.iter().map(|(_, v, _)| v).collect::<Vec<_>>()
            );
        }
        debug!(
            "input: {:?}",
            input.iter().map(|(_, v, _)| v).collect::<Vec<_>>()
        );
        let s = input
            .iter()
            .enumerate()
            .find(|(_, (_, v, _))| *v == 0)
            .unwrap()
            .0;
        let idx1000 = (s + 1000) % input.len();
        let idx2000 = (s + 2000) % input.len();
        let idx3000 = (s + 3000) % input.len();
        debug!("{} {} {}", idx1000, idx2000, idx3000);
        debug!(
            "{} {} {}",
            input.get(idx1000).unwrap().1,
            input.get(idx2000).unwrap().1,
            input.get(idx3000).unwrap().1
        );

        let mut answer = 0;
        answer += input.get(idx1000).unwrap().1;
        answer += input.get(idx2000).unwrap().1;
        answer += input.get(idx3000).unwrap().1;
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let num_passes = 10;
        let mut input = self
            .input
            .iter()
            .map(|v| 811589153 * *v)
            .enumerate()
            .collect();
        debug!("input: {:?}", input);
        for _ in 0..num_passes {
            input = Self::mix(input);
            debug!("input: {:?}", input);
        }
        let s = input.iter().enumerate().find(|(_, v)| v.1 == 0).unwrap().0;
        let idx1000 = (s + 1000) % input.len();
        let idx2000 = (s + 2000) % input.len();
        let idx3000 = (s + 3000) % input.len();
        debug!("{} {} {}", idx1000, idx2000, idx3000);
        let idx1000 = input.get(idx1000).unwrap().1;
        let idx2000 = input.get(idx2000).unwrap().1;
        let idx3000 = input.get(idx3000).unwrap().1;
        debug!("{} {} {}", idx1000, idx2000, idx3000);

        let answer = idx1000 + idx2000 + idx3000;
        Ok(answer)
    }
}

impl Solution {
    fn mix(input: Vec<(usize, ResultType)>) -> Vec<(usize, ResultType)> {
        let num_inputs = input.len() as ResultType;
        debug!("first input {input:?} {}", num_inputs);
        let mut input = input
            .iter()
            .map(|(i, v)| (*i, *v, false))
            .collect::<Vec<_>>();
        loop {
            let found = input
                .iter()
                .enumerate()
                .filter(|(_, (_, _, f))| !f)
                .min_by_key(|(_, (i, _, _))| i);
            if found.is_none() {
                break;
            }
            let (idx, (_, value, _)) = found.unwrap();
            input = Self::shift(input.clone(), idx, *value, |mut v| {
                debug!("setting {v:?} to visited");
                v.2 = true;
            });

            debug!(
                "next input: {:?}",
                input.iter().map(|(_, v, _)| v).collect::<Vec<_>>()
            );
        }
        input.into_iter().map(|(i, v, _)| (i, v)).collect()
    }

    fn shift<T, F>(input: Vec<T>, idx: usize, shift: i64, h: F) -> Vec<T>
    where
        T: Debug + Clone,
        F: Fn(&mut T),
    {
        let input_len = input.len();
        let mut input = input;
        let mut p = input.remove(idx);
        h(&mut p);

        let mut target = idx as i64 + shift;
        target += target.abs() * (input_len - 1) as i64;
        target %= (input_len - 1) as i64;
        // let mut target = idx as i64 + shift;
        // target += target.abs() * input_len as i64;
        // target %= input_len as i64;
        let target = target as usize;

        let mut next_input = Vec::new();
        for i in input.iter().take(target) {
            next_input.push(i.clone());
        }
        debug!("    {next_input:?}");
        next_input.push(p);
        debug!("    {next_input:?}");
        for i in input.iter().skip(target) {
            next_input.push(i.clone());
        }
        debug!("    {next_input:?}");
        next_input
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            let num = line.parse().unwrap();
            solution.input.push(num);
        }
        Ok(solution)
    }
}

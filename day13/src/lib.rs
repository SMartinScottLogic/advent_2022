use std::io::{BufRead, BufReader};

use itertools::Itertools;
use log::debug;

type ResultType = usize;

#[derive(Debug, Default)]
pub struct Solution {
    pairs: Vec<(String, String)>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut answer = 0;
        for (i, (lhs, rhs)) in self.pairs.iter().enumerate() {
            let r = Self::compare(lhs, rhs);
            if let Some(true) = r {
                answer += i + 1;
            }
            debug!("{}: Compare '{}' vs '{}': {:?}", i, lhs, rhs, r);
        }
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut packets = vec!["[[2]]", "[[6]]"];
        for (a, b) in &self.pairs {
            packets.push(a);
            packets.push(b);
        }
        packets.sort_by(|a, b| match Self::compare(a, b) {
            None => std::cmp::Ordering::Equal,
            Some(true) => std::cmp::Ordering::Less,
            Some(false) => std::cmp::Ordering::Greater,
        });
        debug!("{:#?}", packets);

        let answer = packets
            .iter()
            .enumerate()
            .filter(|(_, packet)| *packet == &"[[2]]" || *packet == &"[[6]]")
            .map(|(idx, _)| idx + 1)
            .product();
        Ok(answer)
    }
}

impl Solution {
    fn compare(lhs: &str, rhs: &str) -> Option<bool> {
        debug!("Compare {:?} vs {:?}", lhs, rhs);
        let lhs_elements = Self::elements(lhs);
        let rhs_elements = Self::elements(rhs);

        let mut idx = 0;
        loop {
            if idx == lhs_elements.len() && idx == rhs_elements.len() {
                return None;
            }
            if idx == lhs_elements.len() {
                return Some(true);
            }
            if idx == rhs_elements.len() {
                return Some(false);
            }
            let lhs_element = lhs_elements.get(idx).unwrap();
            let rhs_element = rhs_elements.get(idx).unwrap();
            let lhs_list = lhs_element.starts_with('[');
            let rhs_list = rhs_element.starts_with('[');
            debug!(
                "{}: {} vs {} ({},{})",
                idx, lhs_element, rhs_element, lhs_list, rhs_list
            );
            if !lhs_list && !rhs_list {
                let lhs_value = lhs_element.parse::<i64>().unwrap();
                let rhs_value = rhs_element.parse::<i64>().unwrap();

                if lhs_value < rhs_value {
                    return Some(true);
                }
                if lhs_value > rhs_value {
                    return Some(false);
                }
            } else if lhs_list && rhs_list {
                let r = Self::compare(lhs_element, rhs_element);
                if r.is_some() {
                    return r;
                }
            } else if !lhs_list {
                let r = Self::compare(&format!("[{}]", lhs_element), rhs_element);
                if r.is_some() {
                    return r;
                }
            } else if !rhs_list {
                let r = Self::compare(lhs_element, &format!("[{}]", rhs_element));
                if r.is_some() {
                    return r;
                }
            }
            idx += 1;
        }
    }

    fn elements(s: &str) -> Vec<String> {
        let mut elements = Vec::new();

        let mut level = 0;
        let mut cur_element = String::new();
        for c in s.chars() {
            match c {
                '[' if level < 1 => level += 1,
                '[' => {
                    cur_element.push(c);
                    level += 1;
                }
                ']' if level == 1 => level -= 1,
                ']' => {
                    cur_element.push(c);
                    level -= 1;
                }
                ',' if level == 1 => {
                    elements.push(cur_element);
                    cur_element = String::new();
                }
                _ => cur_element.push(c),
            }
        }
        if !cur_element.is_empty() {
            elements.push(cur_element);
        }
        elements
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for mut pair in &reader.lines().chunks(3) {
            let lhs = pair.next().unwrap().unwrap();
            let rhs = pair.next().unwrap().unwrap();
            solution.pairs.push((lhs, rhs));
        }
        Ok(solution)
    }
}

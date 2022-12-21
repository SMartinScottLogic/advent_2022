use std::collections::HashMap;
use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use log::debug;

pub type ResultType = f64;

#[derive(Debug, Default)]
pub struct Solution {
    monkeys: HashMap<String, Operation>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut eqn = {
            let r = match self.monkeys.get("root").unwrap() {
                Operation::Value(v) => v.to_string(),
                Operation::Sum(lhs, rhs) => format!("({lhs} + {rhs})"),
                Operation::Diff(lhs, rhs) => format!("({lhs} - {rhs})"),
                Operation::Product(lhs, rhs) => format!("({lhs} * {rhs})"),
                Operation::Div(lhs, rhs) => format!("({lhs} / {rhs})"),
            };
            format!("root = {r}")
        };
        // expand
        loop {
            let mut has_changed = false;
            for (id, op) in &self.monkeys {
                let op = if id == "root" {
                    continue;
                } else {
                    match op {
                        Operation::Value(v) => v.to_string(),
                        Operation::Sum(lhs, rhs) => format!("({lhs} + {rhs})"),
                        Operation::Diff(lhs, rhs) => format!("({lhs} - {rhs})"),
                        Operation::Product(lhs, rhs) => format!("({lhs} * {rhs})"),
                        Operation::Div(lhs, rhs) => format!("({lhs} / {rhs})"),
                    }
                };
                let next_eqn = eqn.replace(id, &op);
                if next_eqn != eqn {
                    has_changed = true;
                }
                eqn = next_eqn;
            }
            if !has_changed {
                break;
            }
        }
        debug!("post expand: {eqn}");
        let (lhs, rhs) = eqn.split_once(" = ").unwrap();

        let lhs = Self::reduce(lhs);
        let rhs = Self::reduce(rhs);

        debug!("{lhs} = {rhs}");

        let answer = Self::solve("root", &lhs, &rhs);
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut eqn = "root".to_string();
        // expand
        loop {
            let mut has_changed = false;
            for (id, op) in &self.monkeys {
                if id == "humn" {
                    continue;
                }
                let op = if id == "root" {
                    match op {
                        Operation::Value(v) => v.to_string(),
                        Operation::Sum(lhs, rhs) => format!("{lhs} = {rhs}"),
                        Operation::Diff(lhs, rhs) => format!("{lhs} = {rhs}"),
                        Operation::Product(lhs, rhs) => format!("{lhs} = {rhs}"),
                        Operation::Div(lhs, rhs) => format!("{lhs} = {rhs}"),
                    }
                } else {
                    match op {
                        Operation::Value(v) => v.to_string(),
                        Operation::Sum(lhs, rhs) => format!("({lhs} + {rhs})"),
                        Operation::Diff(lhs, rhs) => format!("({lhs} - {rhs})"),
                        Operation::Product(lhs, rhs) => format!("({lhs} * {rhs})"),
                        Operation::Div(lhs, rhs) => format!("({lhs} / {rhs})"),
                    }
                };
                let next_eqn = eqn.replace(id, &op);
                if next_eqn != eqn {
                    has_changed = true;
                }
                eqn = next_eqn;
            }
            if !has_changed {
                break;
            }
        }
        debug!("post expand: {eqn}");
        let (lhs, rhs) = eqn.split_once(" = ").unwrap();

        let lhs = Self::reduce(lhs);
        let rhs = Self::reduce(rhs);

        debug!("{lhs} = {rhs}");

        let answer = Self::solve("humn", &lhs, &rhs);
        Ok(answer)
    }
}

impl Solution {
    fn solve(target: &str, i_lhs: &str, i_rhs: &str) -> ResultType {
        let (lhs, rhs) = if i_rhs.contains(target) {
            if i_lhs.contains(target) {
                panic!();
            }
            (i_rhs, i_lhs)
        } else {
            (i_lhs, i_rhs)
        };

        let mut lhs = lhs.to_string();
        let mut rhs = rhs.parse::<ResultType>().unwrap();

        let left =
            regex::Regex::new(r"^\((?P<lhs>-?\d+(\.\d+)?) (?P<op>.) (?P<rhs>.*)\)$").unwrap();
        let right =
            regex::Regex::new(r"^\((?P<lhs>.*) (?P<op>.) (?P<rhs>-?\d+(\.\d+)?)\)$").unwrap();
        loop {
            debug!("{lhs} = {rhs}");
            if let Some(m) = left.find(&lhs) {
                let captures = left.captures(m.as_str()).unwrap();
                let l = captures
                    .name("lhs")
                    .unwrap()
                    .as_str()
                    .parse::<ResultType>()
                    .unwrap();
                let r = captures.name("rhs").unwrap().as_str();
                let op = captures.name("op").unwrap().as_str();
                debug!("L  l: {l}, op: {op}, r: {r}");
                rhs = match op {
                    "+" => rhs - l,
                    "-" => l - rhs,
                    "/" => l / rhs,
                    "*" => rhs / l,
                    _ => unreachable!(),
                };
                lhs = r.to_string();
            } else if let Some(m) = right.find(&lhs) {
                let captures = right.captures(m.as_str()).unwrap();
                let l = captures.name("lhs").unwrap().as_str();
                let r = captures
                    .name("rhs")
                    .unwrap()
                    .as_str()
                    .parse::<ResultType>()
                    .unwrap();
                let op = captures.name("op").unwrap().as_str();
                debug!("R  l: {l}, op: {op}, r: {r}");
                rhs = match op {
                    "+" => rhs - r,
                    "-" => rhs + r,
                    "/" => rhs * r,
                    "*" => rhs / r,
                    _ => unreachable!(),
                };
                lhs = l.to_string();
            } else {
                break;
            }
        }
        debug!("{} = {} => {} = {}", i_lhs, i_rhs, lhs, rhs);
        rhs
    }

    fn reduce(eqn: &str) -> String {
        let mut eqn = eqn.to_string();
        let r = regex::Regex::new(r"\((?P<lhs>-?\d+(\.\d+)?) (?P<op>.) (?P<rhs>-?\d+(\.\d+)?)\)")
            .unwrap();
        while let Some(m) = r.find(&eqn) {
            let mut next_rhs = String::new();
            next_rhs.push_str(&eqn[..m.start()]);
            debug!("calc {}", m.as_str());
            let ans = {
                let captures = r.captures(m.as_str()).unwrap();
                let lhs = captures
                    .name("lhs")
                    .unwrap()
                    .as_str()
                    .parse::<ResultType>()
                    .unwrap();
                let rhs = captures
                    .name("rhs")
                    .unwrap()
                    .as_str()
                    .parse::<ResultType>()
                    .unwrap();
                let op = captures.name("op").unwrap().as_str();
                match op {
                    "+" => lhs + rhs,
                    "-" => lhs - rhs,
                    "/" => lhs / rhs,
                    "*" => lhs * rhs,
                    _ => unreachable!(),
                }
            };
            debug!("{} = {}", m.as_str(), ans);
            next_rhs.push_str(&ans.to_string());
            next_rhs.push_str(&eqn[m.end()..]);
            eqn = next_rhs;
        }
        eqn
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            let (monkey, op) = line.split_once(": ").unwrap();
            let op = Operation::from_str(op).unwrap();
            assert!(solution.monkeys.insert(monkey.to_string(), op).is_none());
        }
        Ok(solution)
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Value(ResultType),
    Sum(String, String),
    Diff(String, String),
    Product(String, String),
    Div(String, String),
}

impl FromStr for Operation {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split_whitespace().collect::<Vec<_>>();
        let op = match v.len() {
            1 => Self::Value(v[0].parse().unwrap()),
            3 if v[1] == "+" => Self::Sum(v[0].to_string(), v[2].to_string()),
            3 if v[1] == "-" => Self::Diff(v[0].to_string(), v[2].to_string()),
            3 if v[1] == "*" => Self::Product(v[0].to_string(), v[2].to_string()),
            3 if v[1] == "/" => Self::Div(v[0].to_string(), v[2].to_string()),
            _ => unreachable!(),
        };
        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_lhs_add() {
        assert_eq!(5.0, Solution::solve("huumn", "(huumn + 5)", "10"));
    }

    #[test]
    fn test_lhs_sub() {
        assert_eq!(15.0, Solution::solve("huumn", "(huumn - 5)", "10"));
    }
    #[test]
    fn test_lhs_mult() {
        assert_eq!(2.0, Solution::solve("huumn", "(huumn * 5)", "10"));
    }
    #[test]
    fn test_lhs_div() {
        assert_eq!(50.0, Solution::solve("huumn", "(huumn / 5)", "10"));
    }

    #[test]
    fn test_rhs_add() {
        assert_eq!(5.0, Solution::solve("huumn", "(5 + huumn)", "10"));
    }

    #[test]
    fn test_rhs_sub() {
        assert_eq!(-5.0, Solution::solve("huumn", "(5 - huumn)", "10"));
    }
    #[test]
    fn test_rhs_mult() {
        assert_eq!(2.0, Solution::solve("huumn", "(5 * huumn)", "10"));
    }
    #[test]
    fn test_rhs_div() {
        assert_eq!(0.5, Solution::solve("huumn", "(5 / huumn)", "10"));
    }
}

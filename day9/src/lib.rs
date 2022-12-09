use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;
use log::debug;

#[derive(Debug)]
pub struct Solution {
    motions: Vec<Motion>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<usize>;
    fn analyse(&mut self) {}

    fn answer_part1(&self) -> Self::Result {
        let mut head = Position::new();
        let mut tail = Position::new();

        let mut visited = HashMap::new();
        visited.insert(tail.clone(), 1);
        debug!("{:?} {:?}", head, tail);
        for motion in &self.motions {
            for _ in 1..=motion.count() {
                motion.step(&mut head);
                Self::move_tail(&head, &mut tail);
                visited.insert(tail.clone(), 1);
                debug!("{:?} {:?}", head, tail);
            }
        }
        let answer = visited.len();
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let mut head = Position::new();
        let mut rope = vec![Position::new(); 9];

        let mut visited = HashMap::new();
        visited.insert(rope.last().unwrap().clone(), 1);
        debug!("H{:?} T{:?}", head, rope);
        for motion in &self.motions {
            for _ in 1..=motion.count() {
                motion.step(&mut head);
                Self::move_tail(&head, &mut rope[0]);
                let mut prev_tail = rope[0].clone();
                for tail in rope.iter_mut().skip(1) {
                    Self::move_tail(&prev_tail, tail);
                    prev_tail = tail.clone();
                }
                visited.insert(rope.last().unwrap().clone(), 1);
                debug!("H{:?} T{:?}", head, rope);
            }
        }
        let answer = visited.len();
        Ok(answer)
    }
}

impl Solution {
    fn move_tail(head: &Position, tail: &mut Position) {
        if head.y == tail.y {
            if head.x - tail.x > 1 {
                debug!("R");
                tail.x += 1;
            } else if tail.x - head.x > 1 {
                debug!("L");
                tail.x -= 1;
            }
        } else if head.x == tail.x {
            if head.y - tail.y > 1 {
                debug!("U");
                tail.y += 1;
            } else if tail.y - head.y > 1 {
                debug!("D");
                tail.y -= 1;
            }
        } else if (head.x - tail.x).abs() > 1 || (head.y - tail.y).abs() > 1 {
            if head.x > tail.x && head.y > tail.y {
                tail.x += 1;
                tail.y += 1;
                debug!("NE");
            } else if head.x > tail.x && head.y < tail.y {
                tail.x += 1;
                tail.y -= 1;
                debug!("SE");
            } else if head.x < tail.x && head.y > tail.y {
                tail.x -= 1;
                tail.y += 1;
                debug!("NW");
            } else if head.x < tail.x && head.y < tail.y {
                tail.x -= 1;
                tail.y -= 1;
                debug!("SW");
            }
        }
    }
}
impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            motions: Vec::new(),
        };
        for line in reader.lines() {
            let line = line?;
            solution.motions.push(Motion::from_str(&line).unwrap());
        }
        Ok(solution)
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
enum Motion {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
}

impl Motion {
    fn count(&self) -> usize {
        match self {
            Motion::R(c) => *c,
            Motion::L(c) => *c,
            Motion::U(c) => *c,
            Motion::D(c) => *c,
        }
    }

    fn step(&self, point: &mut Position) {
        match self {
            Motion::R(_) => point.x += 1,
            Motion::L(_) => point.x -= 1,
            Motion::U(_) => point.y += 1,
            Motion::D(_) => point.y -= 1,
        }
    }
}
impl FromStr for Motion {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_whitespace().collect_tuple().unwrap();
        let b = b.parse().unwrap();
        let r = match a {
            "R" => Self::R(b),
            "L" => Self::L(b),
            "U" => Self::U(b),
            "D" => Self::D(b),
            _ => unreachable!(),
        };
        Ok(r)
    }
}

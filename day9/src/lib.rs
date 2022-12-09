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
        let mut head = (0, 0);
        let mut tail = (0, 0);

        let mut visited = HashMap::new();
        visited.insert(tail, 1);
        debug!("{:?} {:?}", head, tail);
        for motion in &self.motions {
            match motion {
                Motion::R(count) => {
                    for _ in 1..=*count {
                        head.0 += 1;
                        Self::move_tail(&head, &mut tail);
                        visited.insert(tail, 1);
                        debug!("{:?} {:?}", head, tail);
                    }
                }
                Motion::L(count) => {
                    for _ in 1..=*count {
                        head.0 -= 1;
                        Self::move_tail(&head, &mut tail);
                        visited.insert(tail, 1);
                        debug!("{:?} {:?}", head, tail);
                    }
                }
                Motion::U(count) => {
                    for _ in 1..=*count {
                        head.1 += 1;
                        Self::move_tail(&head, &mut tail);
                        visited.insert(tail, 1);
                        debug!("{:?} {:?}", head, tail);
                    }
                }
                Motion::D(count) => {
                    for _ in 1..=*count {
                        head.1 -= 1;
                        Self::move_tail(&head, &mut tail);
                        visited.insert(tail, 1);
                        debug!("{:?} {:?}", head, tail);
                    }
                }
            }
        }
        let answer = visited.len();
        // Implement for problem
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let mut head = (0, 0);
        let mut tail1 = (0, 0);
        let mut tail2 = (0, 0);
        let mut tail3 = (0, 0);
        let mut tail4 = (0, 0);
        let mut tail5 = (0, 0);
        let mut tail6 = (0, 0);
        let mut tail7 = (0, 0);
        let mut tail8 = (0, 0);
        let mut tail9 = (0, 0);

        let mut visited = HashMap::new();
        visited.insert(tail9, 1);
        debug!(
            "H{:?} 1{:?} 2{:?} 3{:?} 4{:?} 5{:?} 6{:?} 7{:?} 8{:?} 9{:?}",
            head, tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9
        );
        for motion in &self.motions {
            match motion {
                Motion::R(count) => {
                    for _ in 1..=*count {
                        head.0 += 1;
                        Self::move_tail(&head, &mut tail1);
                        Self::move_tail(&tail1, &mut tail2);
                        Self::move_tail(&tail2, &mut tail3);
                        Self::move_tail(&tail3, &mut tail4);
                        Self::move_tail(&tail4, &mut tail5);
                        Self::move_tail(&tail5, &mut tail6);
                        Self::move_tail(&tail6, &mut tail7);
                        Self::move_tail(&tail7, &mut tail8);
                        Self::move_tail(&tail8, &mut tail9);
                        visited.insert(tail9, 1);
                        debug!(
                            "H{:?} 1{:?} 2{:?} 3{:?} 4{:?} 5{:?} 6{:?} 7{:?} 8{:?} 9{:?}",
                            head, tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9
                        );
                    }
                }
                Motion::L(count) => {
                    for _ in 1..=*count {
                        head.0 -= 1;
                        Self::move_tail(&head, &mut tail1);
                        Self::move_tail(&tail1, &mut tail2);
                        Self::move_tail(&tail2, &mut tail3);
                        Self::move_tail(&tail3, &mut tail4);
                        Self::move_tail(&tail4, &mut tail5);
                        Self::move_tail(&tail5, &mut tail6);
                        Self::move_tail(&tail6, &mut tail7);
                        Self::move_tail(&tail7, &mut tail8);
                        Self::move_tail(&tail8, &mut tail9);
                        visited.insert(tail9, 1);
                        debug!(
                            "H{:?} 1{:?} 2{:?} 3{:?} 4{:?} 5{:?} 6{:?} 7{:?} 8{:?} 9{:?}",
                            head, tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9
                        );
                    }
                }
                Motion::U(count) => {
                    for _ in 1..=*count {
                        head.1 += 1;
                        Self::move_tail(&head, &mut tail1);
                        Self::move_tail(&tail1, &mut tail2);
                        Self::move_tail(&tail2, &mut tail3);
                        Self::move_tail(&tail3, &mut tail4);
                        Self::move_tail(&tail4, &mut tail5);
                        Self::move_tail(&tail5, &mut tail6);
                        Self::move_tail(&tail6, &mut tail7);
                        Self::move_tail(&tail7, &mut tail8);
                        Self::move_tail(&tail8, &mut tail9);
                        visited.insert(tail9, 1);
                        debug!(
                            "H{:?} 1{:?} 2{:?} 3{:?} 4{:?} 5{:?} 6{:?} 7{:?} 8{:?} 9{:?}",
                            head, tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9
                        );
                    }
                }
                Motion::D(count) => {
                    for _ in 1..=*count {
                        head.1 -= 1;
                        Self::move_tail(&head, &mut tail1);
                        Self::move_tail(&tail1, &mut tail2);
                        Self::move_tail(&tail2, &mut tail3);
                        Self::move_tail(&tail3, &mut tail4);
                        Self::move_tail(&tail4, &mut tail5);
                        Self::move_tail(&tail5, &mut tail6);
                        Self::move_tail(&tail6, &mut tail7);
                        Self::move_tail(&tail7, &mut tail8);
                        Self::move_tail(&tail8, &mut tail9);
                        visited.insert(tail9, 1);
                        debug!(
                            "H{:?} 1{:?} 2{:?} 3{:?} 4{:?} 5{:?} 6{:?} 7{:?} 8{:?} 9{:?}",
                            head, tail1, tail2, tail3, tail4, tail5, tail6, tail7, tail8, tail9
                        );
                    }
                }
            }
        }
        // Implement for problem
        let answer = visited.len();
        Ok(answer)
    }
}

impl Solution {
    fn move_tail(head: &(isize, isize), tail: &mut (isize, isize)) {
        if head.1 == tail.1 {
            if head.0 - tail.0 > 1 {
                debug!("R");
                tail.0 += 1;
            } else if tail.0 - head.0 > 1 {
                debug!("L");
                tail.0 -= 1;
            }
        } else if head.0 == tail.0 {
            if head.1 - tail.1 > 1 {
                debug!("U");
                tail.1 += 1;
            } else if tail.1 - head.1 > 1 {
                debug!("D");
                tail.1 -= 1;
            }
        } else if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
            if head.0 > tail.0 && head.1 > tail.1 {
                tail.0 += 1;
                tail.1 += 1;
                debug!("NE");
            } else if head.0 > tail.0 && head.1 < tail.1 {
                tail.0 += 1;
                tail.1 -= 1;
                debug!("SE");
            } else if head.0 < tail.0 && head.1 > tail.1 {
                tail.0 -= 1;
                tail.1 += 1;
                debug!("NW");
            } else if head.0 < tail.0 && head.1 < tail.1 {
                tail.0 -= 1;
                tail.1 -= 1;
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

#[derive(Debug)]
enum Motion {
    R(usize),
    L(usize),
    U(usize),
    D(usize),
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

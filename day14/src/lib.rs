use std::io::{BufRead, BufReader};

use itertools::Itertools;
use log::debug;
use utils::Matrix;

pub type ResultType = usize;

#[derive(Debug, Default)]
pub struct Solution {
    map: Matrix,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let answer = self.drop_sand();
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let answer = self.drop_sand_to_floor();
        Ok(answer)
    }
}

impl Solution {
    fn drop_sand(&self) -> usize {
        let mut sand = Matrix::new();
        let (_maxx, maxy) = self.map.dimensions();
        loop {
            let mut sandx = 500;
            let mut sandy = 0;

            loop {
                if sandy > maxy {
                    return sand.len();
                }
                if let Some((x, y)) = self.sand_step(&sand, sandx, sandy) {
                    sandx = x;
                    sandy = y;
                    continue;
                }
                debug!("sand grain stops at ({}, {})", sandx, sandy);
                sand.set(sandx, sandy, 1);
                break;
            }
        }
    }

    fn drop_sand_to_floor(&self) -> usize {
        let (_maxx, maxy) = self.map.dimensions();
        let mut sand = Matrix::new();
        loop {
            let mut sandx = 500;
            let mut sandy = 0;

            loop {
                if sandy < maxy + 1 {
                    if let Some((x, y)) = self.sand_step(&sand, sandx, sandy) {
                        sandx = x;
                        sandy = y;
                        continue;
                    }
                }
                debug!("sand grain stops at ({}, {})", sandx, sandy);
                sand.set(sandx, sandy, 1);
                if sandx == 500 && sandy == 0 {
                    return sand.len();
                }
                break;
            }
        }
    }

    fn sand_step(
        &self,
        sand: &Matrix,
        mut sandx: isize,
        mut sandy: isize,
    ) -> Option<(isize, isize)> {
        if self
            .map
            .get(sandx, sandy + 1)
            .or_else(|| sand.get(sandx, sandy + 1))
            .is_none()
        {
            // Can go down
            sandy += 1;
            return Some((sandx, sandy));
        }
        if self
            .map
            .get(sandx - 1, sandy + 1)
            .or_else(|| sand.get(sandx - 1, sandy + 1))
            .is_none()
        {
            // Can go down, left
            sandx -= 1;
            sandy += 1;
            return Some((sandx, sandy));
        }
        if self
            .map
            .get(sandx + 1, sandy + 1)
            .or_else(|| sand.get(sandx + 1, sandy + 1))
            .is_none()
        {
            // Can go down, right
            sandx += 1;
            sandy += 1;
            return Some((sandx, sandy));
        }
        None
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;

            let mut lastx = 0;
            let mut lasty = 0;
            for (idx, end) in line.split(" -> ").enumerate() {
                let (x, y) = end
                    .split(',')
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect_tuple::<(_, _)>()
                    .unwrap();
                if idx == 0 {
                    lastx = x;
                    lasty = y;
                    solution.map.set(x as isize, y as isize, 1);
                    continue;
                }
                if lastx == x {
                    loop {
                        solution.map.set(lastx as isize, lasty as isize, 1);
                        if y == lasty {
                            break;
                        }
                        lasty += (y - lasty).signum();
                    }
                }
                if lasty == y {
                    loop {
                        solution.map.set(lastx as isize, lasty as isize, 1);
                        if x == lastx {
                            break;
                        }
                        lastx += (x - lastx).signum();
                    }
                }
            }
        }
        solution.map.display();
        Ok(solution)
    }
}

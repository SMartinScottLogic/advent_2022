use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use log::{debug, info};
use pathfinding::prelude::astar;

pub type ResultType = u64;

#[derive(Debug, Default)]
pub struct Solution {
    map: HashMap<usize, HashMap<usize, char>>,
    max_x: usize,
    max_y: usize,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {
        self.max_y = self.map.len() - 1;
        self.max_x = self.map[&0].len() - 1;
    }

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let result = astar(
            &self.start(),
            |p| self.successors(p),
            |p| self.heuristic(p),
            |p| self.success_end(p),
        );
        let result = result.unwrap();
        info!("{result:?}");
        // Implement for problem
        Ok(result.1 as ResultType)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let start = self.start();
        let end = self.end();
        debug!("{start:?} -> {end:?}");
        let result = astar(
            &start,
            |p| self.successors(p),
            |p| self.heuristic(p),
            |p| self.success_end(p),
        );
        let result = result.unwrap();
        debug!("{result:?}");
        let phase1_time = result.1;

        let mut start = self.end();
        let end = self.start();
        start.2 = phase1_time;
        debug!("{start:?} -> {end:?}");
        let result = astar(
            &start,
            |p| self.successors(p),
            |p| self.heuristic(p),
            |p| self.success_start(p),
        );
        let result = result.unwrap();
        debug!("{result:?}");
        let phase2_time = result.1;

        let mut start = self.start();
        let end = self.end();
        start.2 = phase1_time + phase2_time;
        debug!("{start:?} -> {end:?}");
        let result = astar(
            &start,
            |p| self.successors(p),
            |p| self.heuristic(p),
            |p| self.success_end(p),
        );
        let result = result.unwrap();
        debug!("{result:?}");
        let phase3_time = result.1;
        // Implement for problem
        let answer = (phase1_time + phase2_time + phase3_time) as ResultType;
        Ok(answer)
    }
}

impl Solution {
    fn start(&self) -> (i64, i64, i64) {
        let start_x = self.map[&0]
            .iter()
            .find(|(_, c)| *c == &'.')
            .map(|(x, _)| *x)
            .unwrap();
        (start_x as i64, 0, 0)
    }

    fn end(&self) -> (i64, i64, i64) {
        let end_x = self.map[&self.max_y]
            .iter()
            .find(|(_, c)| *c == &'.')
            .map(|(x, _)| *x)
            .unwrap();
        (end_x as i64, self.max_y as i64, 0)
    }

    fn successors(&self, (x, y, time): &(i64, i64, i64)) -> Vec<((i64, i64, i64), i64)> {
        let mut blizzards = HashMap::new();
        for (y, row) in &self.map {
            let y = *y as i64;
            for (x, c) in row {
                let x = *x as i64;
                let c = *c;
                if c == '#' || c == '.' {
                    continue;
                }
                blizzards.insert((x, y), c);
            }
        }
        let mut next = Vec::new();
        let x = *x;
        let y = *y;
        for dy in -1..=1 {
            for dx in -1..=1 {
                // Can only move in cardinal directions
                if dx != 0 && dy != 0 {
                    continue;
                }
                // Cannot move out of map
                let tx = x + dx;
                let ty = y + dy;
                if tx < 0
                    || ty < 0
                    || tx > self.max_x.try_into().unwrap()
                    || ty > self.max_y.try_into().unwrap()
                {
                    continue;
                }
                // Can't step into walls
                if self.map[&(ty as usize)][&(tx as usize)] == '#' {
                    continue;
                }
                debug!("({tx}, {ty})");
                // Can't go where a blizzard WILL be
                if blizzards.iter().any(|((sx, sy), c)| {
                    let (b_dx, b_dy) = match c {
                        '>' => (1, 0),
                        '<' => (-1, 0),
                        '^' => (0, -1),
                        'v' => (0, 1),
                        _ => unreachable!(),
                    };
                    let dx = sx + b_dx * (time + 1);
                    let dy = sy + b_dy * (time + 1);
                    debug!("({sx}, {sy}) + ({b_dx}, {b_dy}) * {time} = ({dx},{dy})");
                    // Constrain within map
                    let max_x = (self.max_x as i64) - 1;
                    let max_y = (self.max_y as i64) - 1;
                    let dx = 1 + (dx - 1 + (max_x * (time + 1))) % max_x;
                    let dy = 1 + (dy - 1 + (max_y * (time + 1))) % max_y;
                    if dx <= 0 || dy <= 0 || dx > max_x || dy > max_y {
                        panic!("({sx}, {sy}) + ({b_dx}, {b_dy}) * ({time} + 1) = ({dx},{dy})");
                    }
                    debug!("({tx},{tx}) vs ({dx},{dy})");
                    dx == tx && dy == ty
                }) {
                    continue;
                }
                next.push(((tx, ty, time + 1), 1));
            }
        }
        debug!("next ({x},{y}): {next:?}");
        next
    }

    fn heuristic(&self, (x, y, _): &(i64, i64, i64)) -> i64 {
        let x = *x;
        let y = *y;

        let end_x = self.map[&self.max_y]
            .iter()
            .find(|(_, c)| *c == &'.')
            .map(|(x, _)| *x)
            .unwrap();

        (x - end_x as i64).abs() + (y - self.max_y as i64).abs()
    }

    fn success_start(&self, (x, y, _): &(i64, i64, i64)) -> bool {
        let x = *x;
        let y = *y;
        debug!("test ({x},{y})");
        y == 0
    }

    fn success_end(&self, (x, y, _): &(i64, i64, i64)) -> bool {
        let x = *x;
        let y = *y;
        debug!("test ({x},{y})");
        y == self.max_y as i64
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            let entry = solution.map.entry(y).or_insert_with(HashMap::new);
            for (x, c) in line.chars().enumerate() {
                entry.insert(x, c);
            }
        }
        Ok(solution)
    }
}

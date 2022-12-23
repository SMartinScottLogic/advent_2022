use std::{io::{BufRead, BufReader}, collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;
use log::{info, debug};

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    grove: HashMap<(i64, i64), char>
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut elves = self.elf_position();

        for step in 0..10 {
            let (new_elves, _) = Self::perform_step(elves, step);
            elves = new_elves;
        }

        let (xmin, xmax) = elves.iter().map(|p| p.0).minmax().into_option().unwrap();
        let (ymin, ymax) = elves.iter().map(|p| p.1).minmax().into_option().unwrap();
    
        debug!("{elves:?}");
        println!("{} in ({},{})-({},{})", elves.len(), xmin, ymin, xmax, ymax);
        let non_elves = (xmax - xmin + 1) * (ymax - ymin + 1) - elves.len() as i64;
        Ok(non_elves)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut elves = self.elf_position();

        let mut step = 0;
        let answer = loop {
            let (new_elves, has_moved) = Self::perform_step(elves, step);
            step += 1;
            if !has_moved {
                break step;
            }
            elves = new_elves;
        };
        Ok(answer as ResultType)
    }
}

impl Solution {
    fn elf_position(&self) -> HashSet<(i64, i64)> {
        let maxy = self.grove.iter().map(|((_, y), _)| y).max().unwrap();

        // extract elf positions
        let mut elves = HashSet::new();
        for ((x, y), c) in &self.grove {
            if *c != '#' {
                continue;
            }
            let y = maxy - y;
            elves.insert((*x, y));
        }
        elves
    }

    fn perform_step(mut elves: HashSet<(i64, i64)>, step: usize) -> (HashSet<(i64, i64)>, bool) {
        // First half
        let mut proposals = HashMap::new();
        for (x, y) in &elves {
            let mut has_neighbour = false;
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if dx==0 && dy == 0 {
                        continue;
                    }
                    if elves.contains(&(x+dx, y+dy)) {
                        has_neighbour = true;
                        if *x == 51 && step == 4 {
                            println!("elf @ ({},{} has neighbour @ ({},{})", x, y, x+dx, y+dy);
                        }
                    }
                }
            }
            if !has_neighbour {
                continue;
            }
            if *x == 51 && step == 4 {
                println!("elf @ ({x},{y} can propose");
            }
            debug!("elf @ ({x},{y}) can propose");
            let mut dir = vec!["North", "South", "West", "East"];
            dir.rotate_left(step % 4);
            for d in dir {
                let (minx, maxx) = match d {
                    "North" | "South" => (-1, 1),
                    "East" => (1, 1),
                    "West" => (-1, -1),
                    _ => unreachable!()
                };
                let (miny, maxy) = match d {
                    "North" => (1, 1),
                    "South" => (-1, -1),
                    "East" | "West" => (-1, 1),
                    _ => unreachable!()
                };
                if *x == 51 && step == 4 {
                    println!("elf @ ({x},{y} : {d}: ({minx},{miny}) - ({maxx},{maxy})");
                }
                has_neighbour = false;
                for dy in miny..=maxy {
                    for dx in minx..=maxx {
                        if elves.contains(&(x+dx,y+dy)) {
                            has_neighbour = true;
                            if *x == 51 && step == 4 {
                                println!("elf @ ({},{}) : {}: has neighbour @ ({}, {})", x, y, d, x+dx, y+dy);
                            }
                        }
                    }
                }
                if has_neighbour {
                    continue;
                }
                let (dx, dy) = match d {
                    "North" => (0, 1),
                    "South" => (0, -1),
                    "East" => (1, 0),
                    "West" => (-1, 0),
                    _ => unreachable!()
                };
                debug!("elf @ ({x},{y}) proposes {d}");
                if *x == 51 && step == 4 {
                    println!("elf @ ({},{}) proposes {} to ({}, {})", x, y, d, x+dx, y+dy);
                }
                let e = proposals.entry((x+dx, y+dy)).or_insert_with(Vec::new);
                e.push((*x, *y));
                break;
            }
        }
        // Second half
        let mut changed = false;
        debug!("{proposals:?}");
        for ((tx, ty), sources) in proposals {
            if sources.len() > 1 {
                continue;
            }
            let (x, y) = sources[0];
            debug!("elf move from ({x},{y}) to ({tx},{ty})");
            if step == 4 { println!("Move {:?} to {:?}", (x, y), (tx, ty)); }
            assert!(elves.remove(&(x, y)));
            assert!(elves.insert((tx, ty)));
            changed = true;
        }
        debug!("P{step}: {elves:?}");
        (elves, changed)
}
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            for (x, c) in line.chars().enumerate() {
                solution.grove.insert((x as i64, y as i64), c);
            }
        }
        Ok(solution)
    }
}

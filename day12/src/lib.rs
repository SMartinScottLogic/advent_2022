use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use log::debug;

type ResultType = i64;
#[derive(Debug, Default)]
pub struct Solution {
    grid: HashMap<(usize, usize), char>,
    maxx: usize,
    maxy: usize,
    startx: usize,
    starty: usize,
    endx: usize,
    endy: usize,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self) {}

    fn answer_part1(&self) -> Self::Result {
        let initial_scorer = |node: &(usize, usize)| {
            self.grid
                .get(node)
                .and_then(|v| if *v == 'S' { Some(0_i64) } else { None })
        };
        let answer = self.shortest_path(initial_scorer).unwrap();
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let initial_scorer = |node: &(usize, usize)| {
            self.grid
                .get(node)
                .and_then(|v| if *v == 'S' || *v == 'a' { Some(0_i64) } else { None })
        };
        let answer = self.shortest_path(initial_scorer).unwrap();
        Ok(answer)
    }
}

impl Solution {
    fn shortest_path<F>(&self, initial_scorer: F) -> Option<ResultType> 
    where F: Fn(&(usize, usize)) -> Option<ResultType>,
    {
        let heightvals: HashMap<_, _> = "abcdefghijklmnopqrstuvwxyzES"
            .chars()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();
        let nodes = self.grid.keys().copied().collect();
        let get_neighbours = |node: &(usize, usize)| {
            let (x, y) = node;
            let mut neighbours = Vec::new();
            let curheight = self.grid.get(&(*x, *y)).unwrap();
            let curheight = heightvals.get(curheight).unwrap();
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if ((dx + dy) as i32).abs() != 1 {
                        continue;
                    }
                    let tposx = *x as isize + dx;
                    let tposy = *y as isize + dy;
                    if tposx < 0 || tposy < 0 {
                        continue;
                    }
                    if tposx > self.maxx as isize || tposy > self.maxy as isize {
                        continue;
                    }
                    let tposx = tposx as usize;
                    let tposy = tposy as usize;
                    let t_height = self.grid.get(&(tposx, tposy as usize)).unwrap();
                    let t_height = heightvals.get(t_height).unwrap();
                    if t_height > curheight && t_height - curheight > 1 {
                        debug!(
                            "can't reach ({},{})={} from ({},{})={}",
                            tposx, tposy, t_height, x, y, curheight
                        );
                        continue;
                    }
                    debug!(
                        "can reach ({},{})={} from ({},{})={}",
                        tposx, tposy, t_height, x, y, curheight
                    );
                    neighbours.push((tposx, tposy));
                }
            }
            debug!("get_neighbours of ({}, {}): {:?}", x, y, neighbours);
            neighbours.into_iter()
        };
        let is_end = |node: &(usize, usize)| {
            self.grid
                .get(node)
                .map(|v| *v == 'E')
                .unwrap_or(false)
        };
        utils::dijkstra(&nodes, initial_scorer, get_neighbours, is_end)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (y, line) in reader.lines().enumerate() {
            for (x, c) in line?.chars().enumerate() {
                solution.grid.insert((x, y), c);
                if c == 'S' {
                    solution.startx = x;
                    solution.starty = y;
                }
                if c == 'E' {
                    solution.endx = x;
                    solution.endy = y;
                }
                if x > solution.maxx {
                    solution.maxx = x;
                }
            }
            if y > solution.maxy {
                solution.maxy = y;
            }
        }
        Ok(solution)
    }
}

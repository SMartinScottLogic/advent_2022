use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use log::debug;
use utils::Matrix;

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
        let mut cost = Matrix::new();
        let mut startx = 0;
        let mut starty = 0;
        for ((x, y), v) in &self.grid {
            if *v == 'S' {
                cost.set(*x as isize, *y as isize, 0);
                startx = *x;
                starty = *y;
            }
        }
        let answer = self.shortest_path(startx, starty, cost).unwrap();
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let mut cost = Matrix::new();
        let mut startx = 0;
        let mut starty = 0;
        for ((x, y), v) in &self.grid {
            if *v == 'S' || *v == 'a' {
                cost.set(*x as isize, *y as isize, 0);
                startx = *x;
                starty = *y;
            }
        }

        let answer = self.shortest_path(startx, starty, cost).unwrap();
        /*
        let answer = self
            .grid
            .iter()
            .filter(|(_, v)| *v == &'a')
            .flat_map(|((x, y), _)| self.shortest_path(*x, *y))
            .min()
            .unwrap();
        */
        Ok(answer)
    }
}

impl Solution {
    fn shortest_path(&self, startx: usize, starty: usize, mut cost: Matrix) -> Option<ResultType> {
        //let mut cost = Matrix::new();
        let mut visited = Matrix::new();
        cost.set(startx as isize, starty as isize, 0);

        let mut posx = startx;
        let mut posy = starty;
        let mut curstep = 0;
        let heightvals: HashMap<_, _> = "abcdefghijklmnopqrstuvwxyzES"
            .chars()
            .enumerate()
            .map(|(a, b)| (b, a))
            .collect();
        let answer = loop {
            if posx == self.endx && posy == self.endy {
                break curstep;
            }
            visited.set(posx as isize, posy as isize, 1);
            let curheight = self.grid.get(&(posx, posy)).unwrap();
            let curheight = heightvals.get(curheight).unwrap();
            for dy in -1..=1 {
                for dx in -1..=1 {
                    if ((dx + dy) as i32).abs() != 1 {
                        continue;
                    }
                    let tposx = posx as isize + dx;
                    let tposy = posy as isize + dy;
                    if tposx < 0 || tposy < 0 {
                        continue;
                    }
                    if tposx > self.maxx as isize || tposy > self.maxy as isize {
                        continue;
                    }
                    let tposx = tposx as usize;
                    let tposy = tposy as usize;
                    let t_height = self.grid.get(&(tposx, tposy)).unwrap();
                    let t_height = heightvals.get(t_height).unwrap();
                    if t_height > curheight && t_height - curheight > 1 {
                        debug!(
                            "can't reach ({},{})={} from ({},{})={}",
                            tposx, tposy, t_height, posx, posy, curheight
                        );
                        continue;
                    }
                    debug!(
                        "can reach ({},{})={} from ({},{})={}",
                        tposx, tposy, t_height, posx, posy, curheight
                    );
                    let tcost = cost.get(tposx as isize, tposy as isize);
                    match tcost {
                        Some(c) if c < &(1 + curstep) => {}
                        Some(c) => {
                            debug!("set ({},{}) to {} from {}", tposx, tposy, curstep + 1, c);
                            cost.set(tposx as isize, tposy as isize, curstep + 1);
                        }
                        _ => {
                            debug!("set ({},{}) to {}", tposx, tposy, curstep + 1);
                            cost.set(tposx as isize, tposy as isize, curstep + 1);
                        }
                    }
                }
            }
            let mut bestcost = i64::MAX;
            let mut bestx = 0;
            let mut besty = 0;
            for (x, y) in self.grid.keys() {
                if visited.get(*x as isize, *y as isize).is_none() {
                    match cost.get(*x as isize, *y as isize) {
                        Some(c) if c < &bestcost => {
                            bestx = *x;
                            besty = *y;
                            bestcost = *c;
                        }
                        _ => {}
                    }
                };
            }
            if bestcost == i64::MAX {
                return None;
            }
            posx = bestx;
            posy = besty;
            debug!("next: ({posx}, {posy}) with {bestcost}");
            curstep = bestcost;
        };

        for y in 0..=self.maxy {
            let mut line = String::new();
            for x in 0..=self.maxx {
                let s = match cost.get(x as isize, y as isize) {
                    None => "..".to_string(),
                    Some(c) => format!("{:02}", c),
                };
                line.push_str(&s);
                line.push(' ');
            }
            debug!("{}: {}", y, line);
        }
        Some(answer)
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

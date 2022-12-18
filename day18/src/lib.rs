use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

use itertools::Itertools;
use log::{debug, info};

pub type ResultType = usize;

#[derive(Debug, Default)]
pub struct Solution {
    cubes: Vec<Position>,
    minx: f64,
    miny: f64,
    minz: f64,
    maxx: f64,
    maxy: f64,
    maxz: f64,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let answer = Self::uncovered(&self.cubes);
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut material = HashMap::new();
        for cube in &self.cubes {
            material.insert((cube.x as i64, cube.y as i64, cube.z as i64), Colour::Cube);
        }

        loop {
            let position = self.find_uncoloured(&material);
            if position.is_none() {
                break;
            }
            let mut probes = Vec::new();

            let position = position.unwrap();
            debug!("uncovered: {position:?}");
            probes.push(position);

            let mut reached_outside = false;
            while let Some(p) = probes.pop() {
                debug!("p: {p:?}");
                material.insert((p.x as i64, p.y as i64, p.z as i64), Colour::Probe);

                for dz in -1..=1 {
                    for dy in if dz == 0 { -1..=1 } else { 0..=0 } {
                        for dx in if dz == 0 && dy == 0 { -1..=1 } else { 0..=0 } {
                            let x = p.x + dx as f64;
                            let y = p.y + dy as f64;
                            let z = p.z + dz as f64;
                            if x < self.minx || y < self.miny || z < self.minz {
                                reached_outside = true;
                                continue;
                            }
                            if x > self.maxx || y > self.maxy || z > self.maxz {
                                reached_outside = true;
                                continue;
                            }
                            material
                                .entry((x as i64, y as i64, z as i64))
                                .or_insert_with(|| {
                                    probes.push(Position { id: 0, x, y, z });
                                    Colour::Probe
                                });
                        }
                    }
                }
            }
            let recolour = if reached_outside {
                Colour::Water
            } else {
                Colour::Air
            };
            for (p, c) in material.iter_mut() {
                if *c == Colour::Probe {
                    *c = recolour;
                    debug!("recoloured {p:?} to {recolour:?}");
                }
            }
        }

        debug!("{material:?}");

        let cubes: Vec<Position> = material
            .into_iter()
            .filter(|(_, colour)| *colour == Colour::Cube || *colour == Colour::Air)
            .enumerate()
            .map(|(idx, (p, _))| Position {
                id: idx,
                x: p.0 as f64,
                y: p.1 as f64,
                z: p.2 as f64,
            })
            .collect();

        let answer = Self::uncovered(&cubes);

        Ok(answer)
    }
}

impl Solution {
    fn new() -> Self {
        Self {
            cubes: Vec::new(),
            minx: f64::NAN,
            miny: f64::NAN,
            minz: f64::NAN,
            maxx: f64::NAN,
            maxy: f64::NAN,
            maxz: f64::NAN,
        }
    }

    fn add_cube(&mut self, position: Position) {
        self.minx = self.minx.min(position.x);
        self.miny = self.miny.min(position.y);
        self.minz = self.minz.min(position.z);
        self.maxx = self.maxx.max(position.x);
        self.maxy = self.maxy.max(position.y);
        self.maxz = self.maxz.max(position.z);
        self.cubes.push(position);
    }

    fn find_uncoloured(&self, material: &HashMap<(i64, i64, i64), Colour>) -> Option<Position> {
        for cube in &self.cubes {
            for dz in -1..=1 {
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let x = cube.x + dx as f64;
                        let y = cube.y + dy as f64;
                        let z = cube.z + dz as f64;
                        if x < self.minx || y < self.miny || y < self.minz {
                            continue;
                        }
                        if x > self.maxx || y > self.maxy || y > self.maxz {
                            continue;
                        }
                        if !material.contains_key(&(x as i64, y as i64, z as i64)) {
                            return Some(Position { id: 0, x, y, z });
                        }
                    }
                }
            }
        }
        None
    }

    fn uncovered(cubes: &Vec<Position>) -> ResultType {
        let mut uncovered = cubes.iter().map(|_| 6).collect::<Vec<_>>();
        for cube1 in cubes {
            for cube2 in cubes {
                if cube1.id >= cube2.id {
                    continue;
                }
                let overlaps = cube1.num_overlaps(cube2);
                debug!("{:?} {:?} overlaps {}", cube1, cube2, overlaps);

                uncovered[cube1.id] -= overlaps;
                uncovered[cube2.id] -= overlaps;
            }
        }
        uncovered.iter().sum()
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::new();
        for (id, line) in reader.lines().enumerate() {
            let line = line?;
            let mut position = Position::from_str(&line).unwrap();
            position.id = id;
            solution.add_cube(position);
        }
        Ok(solution)
    }
}

#[derive(Debug, Clone, PartialOrd)]
struct Position {
    id: usize,
    x: f64,
    y: f64,
    z: f64,
}

impl Position {
    fn num_overlaps(&self, other: &Self) -> usize {
        let mut count = 0;
        for face1 in self.faces() {
            for face2 in other.faces() {
                if face1 == face2 {
                    count += 1;
                }
            }
        }
        count
    }

    fn faces(&self) -> Vec<Position> {
        vec![
            Position {
                id: 0,
                x: self.x - 0.5,
                y: self.y,
                z: self.z,
            },
            Position {
                id: 0,
                x: self.x + 0.5,
                y: self.y,
                z: self.z,
            },
            Position {
                id: 0,
                x: self.x,
                y: self.y - 0.5,
                z: self.z,
            },
            Position {
                id: 0,
                x: self.x,
                y: self.y + 0.5,
                z: self.z,
            },
            Position {
                id: 0,
                x: self.x,
                y: self.y,
                z: self.z - 0.5,
            },
            Position {
                id: 0,
                x: self.x,
                y: self.y,
                z: self.z + 0.5,
            },
        ]
    }
}

impl FromStr for Position {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y, z) = s
            .split(',')
            .map(|v| v.parse::<f64>().unwrap())
            .collect_tuple()
            .unwrap();
        Ok(Self { id: 0, x, y, z })
    }
}

impl PartialEq<Self> for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Colour {
    Cube,
    Probe,
    Air,
    Water,
}

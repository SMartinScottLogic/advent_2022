use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use log::{debug, info, trace};
use utils::Matrix;

pub type ResultType = isize;

#[derive(Debug, Default)]
pub struct Solution {
    input: String,
    jet: Vec<char>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let height = self.simulate(2022);
        Ok(height)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let height = self.simulate(1000000000000);
        Ok(height)
    }
}

impl Solution {
    fn simulate(&self, final_rock_idx: i64) -> isize {
        let mut memo = HashMap::new();
        let mut height_store = Vec::new();
        let mut grid = Matrix::new();
        let mut jet_idx = 0;
        let mut shape_idx = 0;
        loop {
            // bottom-most
            let grid_maxy = grid.max_y();
            let mut sy = grid_maxy + 3 + 1;
            let mut sx = 2; // left-most
            let shape = Shape::from(shape_idx);
            if shape_idx > 2022 && shape == Shape::Bar {
                let hash = {
                    let mut s = String::new();
                    for y in grid_maxy - 17..=grid_maxy {
                        for x in 0..7 {
                            let c = match grid.get(x, y).unwrap_or(&0) {
                                0 => '0',
                                1 => '1',
                                _ => unreachable!(),
                            };
                            s.push(c);
                        }
                    }
                    s
                };
                if let Some((old_idx, old_height)) = memo.get(&hash.clone()) {
                    debug!(
                        "cur_shape_idx: {} cur_height: {}: old_idx: {} old_height: {}",
                        shape_idx, grid_maxy, old_idx, old_height
                    );
                    let grid_maxy = grid_maxy as i64;
                    let height_step: i64 = grid_maxy - (*old_height) as i64;
                    let idx_step = shape_idx - old_idx;

                    let multiplier = (final_rock_idx - old_idx) / idx_step;
                    debug!("multiplier: {multiplier}");
                    let input_ff = idx_step * multiplier + old_idx;
                    debug!("input_ff: {input_ff}");
                    let height_ff = height_step * multiplier + *old_height as i64;
                    debug!("height_ff: {height_ff}");
                    let input_remaining = final_rock_idx - input_ff;
                    debug!("input_remaining: {input_remaining}");
                    let height_remaining = height_store[(*old_idx + input_remaining) as usize]
                        - height_store[*old_idx as usize];
                    debug!("height_remaining: {height_remaining}");
                    let res = height_remaining as i64 + height_ff;
                    debug!("res: {res}");
                    return res.try_into().unwrap();
                }
                memo.insert(hash, (shape_idx, grid_maxy));
            }
            if memo.len() % 1000000 == 0 {
                debug!("memo len: {}", memo.len());
            }

            shape_idx += 1;

            loop {
                // Jet
                let jet = self.jet.get(jet_idx % self.jet.len()).unwrap();
                jet_idx += 1;
                let tx = match jet {
                    '>' => sx + 1,
                    '<' => sx - 1,
                    _ => unreachable!(),
                };
                if tx >= 0 && tx + shape.width() <= 7 && !self.collision(&shape, tx, sy, &grid) {
                    sx = tx;
                }
                // Drop
                let ty = sy - 1;
                if ty <= 0 || self.collision(&shape, sx, ty, &grid) {
                    // Stop rock
                    // Write shape to grid
                    for (cx, cy) in shape.cells(sx, sy) {
                        grid.set(cx, cy, 1);
                    }
                    let grid_maxy = grid.max_y();
                    height_store.push(grid_maxy);
                    break;
                }
                sy = ty;
            }
            debug!("{shape:?} {sx} {sy}");
            if shape_idx >= final_rock_idx {
                break;
            }
        }
        let grid_maxy = grid.max_y();
        info!("{:?}", grid.dimensions());
        grid_maxy
    }

    fn collision(&self, shape: &Shape, x: isize, y: isize, grid: &Matrix) -> bool {
        let cells = shape.cells(x, y);
        trace!("{shape:?} @ ({x},{y}): {cells:?}");
        cells.iter().any(|(cx, cy)| {
            let v = grid.get(*cx, *cy).unwrap_or(&0);
            if v != &0 {
                debug!("{shape:?} collision @ ({cx},{cy})");
            }
            v != &0
        })
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for (idx, line) in reader.lines().enumerate() {
            assert_eq!(0, idx);
            let line = line?;
            solution.input = line.clone();
            solution.jet = line.chars().collect();
        }
        Ok(solution)
    }
}

#[derive(Debug, PartialEq)]
enum Shape {
    Bar,
    Cross,
    Angle,
    Pipe,
    Box,
}

impl From<i64> for Shape {
    fn from(idx: i64) -> Shape {
        match idx % 5 {
            0 => Self::Bar,
            1 => Self::Cross,
            2 => Self::Angle,
            3 => Self::Pipe,
            4 => Self::Box,
            _ => unreachable!(),
        }
    }
}

impl Shape {
    fn cells(&self, x: isize, y: isize) -> Vec<(isize, isize)> {
        match self {
            Shape::Bar => vec![(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
            Shape::Cross => vec![
                (x, y + 1),
                (x + 1, y + 2),
                (x + 1, y + 1),
                (x + 1, y),
                (x + 2, y + 1),
            ],
            Shape::Angle => vec![
                (x, y),
                (x + 1, y),
                (x + 2, y),
                (x + 2, y + 1),
                (x + 2, y + 2),
            ],
            Shape::Pipe => vec![(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            Shape::Box => vec![(x, y), (x + 1, y), (x + 1, y + 1), (x, y + 1)],
        }
    }

    fn width(&self) -> isize {
        match self {
            Shape::Bar => 4,
            Shape::Cross => 3,
            Shape::Angle => 3,
            Shape::Pipe => 1,
            Shape::Box => 2,
        }
    }
}

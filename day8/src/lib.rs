use log::debug;
use std::{
    cmp::max,
    io::{BufRead, BufReader},
};
use utils::Matrix;

#[derive(Debug)]
pub struct Solution {
    heights: Matrix,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<i64>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let visible = self.get_visible();

        visible.display();
        let (maxx, maxy) = self.heights.dimensions();
        let mut answer = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                answer += visible.get(x, y).unwrap_or(&0);
            }
        }
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let answer = self.best_scenic();
        Ok(answer)
    }
}

impl Solution {
    fn get_visible(&self) -> Matrix {
        let mut visible = Matrix::new();
        let (maxx, maxy) = self.heights.dimensions();
        for y in 0..=maxy {
            for x in 0..=maxx {
                let v = self.is_visible(x, y);
                visible.set(x, y, i64::from(v));
            }
        }
        visible
    }

    fn best_scenic(&self) -> i64 {
        let (maxx, maxy) = self.heights.dimensions();
        let mut best = 0;
        for y in 0..=maxy {
            for x in 0..=maxx {
                best = max(self.get_scenic(x, y), best);
            }
        }
        best
    }

    fn get_scenic(&self, x: isize, y: isize) -> i64 {
        let (maxx, maxy) = self.heights.dimensions();
        if x == 0 || y == 0 || x == maxx || y == maxy {
            return 0;
        }
        let height = self.heights.get(x, y).unwrap();
        let mut seen = (0, 0, 0, 0);

        // Upwards
        let mut py = y - 1;
        loop {
            seen.0 += 1;
            let pheight = self.heights.get(x, py).unwrap();
            if pheight >= height {
                break;
            }
            if py == 0 {
                break;
            }
            py -= 1;
        }
        // Downwards
        let mut py = y + 1;
        loop {
            seen.1 += 1;
            let pheight = self.heights.get(x, py).unwrap();
            if pheight >= height {
                break;
            }
            if py == maxy {
                break;
            }
            py += 1;
        }
        // Left
        let mut px = x - 1;
        loop {
            seen.2 += 1;
            let pheight = self.heights.get(px, y).unwrap();
            if pheight >= height {
                break;
            }
            if px == 0 {
                break;
            }
            px -= 1;
        }
        // Right
        let mut px = x + 1;
        loop {
            seen.3 += 1;
            let pheight = self.heights.get(px, y).unwrap();
            if pheight >= height {
                break;
            }
            if px == maxx {
                break;
            }
            px += 1;
        }

        debug!("{:?}", seen);
        seen.0 * seen.1 * seen.2 * seen.3
    }

    fn is_visible(&self, x: isize, y: isize) -> bool {
        let (maxx, maxy) = self.heights.dimensions();
        if x == 0 || y == 0 || x == maxx || y == maxy {
            return true;
        }
        let height = self.heights.get(x, y).unwrap();
        let mut max_height = &0;
        for py in 0..y {
            max_height = max(self.heights.get(x, py).unwrap(), max_height)
        }
        if max_height < height {
            return true;
        }

        max_height = &0;
        for py in (y + 1)..=maxy {
            max_height = max(self.heights.get(x, py).unwrap(), max_height)
        }
        if max_height < height {
            return true;
        }

        let mut max_height = &0;
        for px in 0..x {
            max_height = max(self.heights.get(px, y).unwrap(), max_height)
        }
        if max_height < height {
            return true;
        }

        max_height = &0;
        for px in (x + 1)..=maxx {
            max_height = max(self.heights.get(px, y).unwrap(), max_height)
        }
        if max_height < height {
            return true;
        }

        false
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            heights: Matrix::new(),
        };
        for (y, line) in reader.lines().enumerate() {
            let line = line?;
            let line = line.trim();
            for (x, c) in line.chars().enumerate() {
                let value = c.to_digit(10).unwrap();
                solution
                    .heights
                    .set(x as isize, y as isize, value.try_into().unwrap());
            }
        }
        Ok(solution)
    }
}

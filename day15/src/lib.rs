use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
    str::FromStr,
};

use anyhow::Context;
use log::{debug, info};
use utils::Matrix;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    sensors: Vec<Sensor>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, is_full: bool) -> Self::Result {
        let probe_y = if is_full { 2000000 } else { 10 };

        let mut not = Matrix::new();
        let mut minx = i64::MAX;
        let mut maxx = i64::MIN;
        let mut miny = i64::MAX;
        let mut maxy = i64::MIN;
        for sensor in &self.sensors {
            let distance_from_beacon = sensor.distance_from_beacon();
            debug!("{:?}: {}", sensor, distance_from_beacon);
            not.set(
                sensor.px.try_into().unwrap(),
                sensor.py.try_into().unwrap(),
                1,
            );
            not.set(
                sensor.bx.try_into().unwrap(),
                sensor.by.try_into().unwrap(),
                2,
            );
            for y in probe_y..=probe_y {
                for x in sensor.px - distance_from_beacon..=sensor.px + distance_from_beacon {
                    if sensor.distance_from_location(x, y) <= distance_from_beacon {
                        if not
                            .get(x.try_into().unwrap(), y.try_into().unwrap())
                            .is_none()
                        {
                            not.set(x.try_into().unwrap(), y.try_into().unwrap(), 3);
                        }
                        if x > maxx {
                            maxx = x;
                        }
                        if x < minx {
                            minx = x;
                        }
                        if y > maxy {
                            maxy = y;
                        }
                        if y < miny {
                            miny = y;
                        }
                    }
                }
            }
        }
        let mut answer = 0;
        for x in minx..=maxx {
            let v = not
                .get(x.try_into().unwrap(), probe_y.try_into().unwrap())
                .unwrap_or(&0);
            if *v == 3 {
                answer += 1;
            }
        }
        Ok(answer)
    }

    fn answer_part2(&self, is_full: bool) -> Self::Result {
        let (minx, maxx, miny, maxy) = if is_full {
            (0, 4000000, 0, 4000000)
        } else {
            (0, 20, 0, 20)
        };
        let mut uncovered = HashSet::new();
        for sensor in &self.sensors {
            let probe_distance = 1 + sensor.distance_from_beacon();
            debug!("{:?}: {}", sensor, probe_distance);

            // Top right
            let mut y = sensor.py - probe_distance;
            let mut x = sensor.px;
            loop {
                if x >= minx && x <= maxx && y >= miny && y <= maxy {
                    //info!("{sensor:?} probe ({x},{y})");
                    if self.is_visible(x, y) {
                        uncovered.insert((x, y));
                        info!("uncovered: {x}{y}");
                    }
                }
                if y == sensor.py {
                    break;
                }
                y += 1;
                x += 1;
            }
            // Top left
            let mut y = sensor.py - probe_distance;
            let mut x = sensor.px;
            loop {
                if x >= minx && x <= maxx && y >= miny && y <= maxy {
                    //info!("{sensor:?} probe ({x},{y})");
                    if self.is_visible(x, y) {
                        uncovered.insert((x, y));
                        info!("uncovered: {x}{y}");
                    }
                }
                if y == sensor.py {
                    break;
                }
                y += 1;
                x -= 1;
            }
            // Bottom left
            let mut y = sensor.py + probe_distance;
            let mut x = sensor.px;
            loop {
                if x >= minx && x <= maxx && y >= miny && y <= maxy {
                    //info!("{sensor:?} probe ({x},{y})");
                    if self.is_visible(x, y) {
                        uncovered.insert((x, y));
                        info!("uncovered: {x}{y}");
                    }
                }
                if y == sensor.py {
                    break;
                }
                y -= 1;
                x -= 1;
            }
            // Bottom right
            let mut y = sensor.py + probe_distance;
            let mut x = sensor.px;
            loop {
                if x >= minx && x <= maxx && y >= miny && y <= maxy {
                    //info!("{sensor:?} probe ({x},{y})");
                    if self.is_visible(x, y) {
                        uncovered.insert((x, y));
                        info!("uncovered: {x}{y}");
                    }
                }
                if y == sensor.py {
                    break;
                }
                y -= 1;
                x += 1;
            }
        }

        debug!("{uncovered:?}");

        let answer = uncovered.iter().map(|(x, y)| x * 4000000 + y).sum();
        Ok(answer)
    }
}

impl Solution {
    fn is_visible(&self, x: i64, y: i64) -> bool {
        for sensor in &self.sensors {
            if sensor.distance_from_location(x, y) <= sensor.distance_from_beacon() {
                return false;
            }
        }
        true
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            let sensor = Sensor::from_str(&line).unwrap();
            solution.sensors.push(sensor);
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Sensor {
    px: i64,
    py: i64,
    bx: i64,
    by: i64,
}

impl Sensor {
    #[inline]
    fn distance_from_location(&self, x: i64, y: i64) -> i64 {
        (self.px - x).abs() + (self.py - y).abs()
    }

    #[inline]
    fn distance_from_beacon(&self) -> i64 {
        self.distance_from_location(self.bx, self.by)
    }
}

impl FromStr for Sensor {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = regex::Regex::new(r"^Sensor at x=(?P<px>-?\d+), y=(?P<py>-?\d+): closest beacon is at x=(?P<bx>-?\d+), y=(?P<by>-?\d+)$").context("constructing parser")?;
        let captures = r.captures(s).context(format!("Parsing: '{s}'"))?;

        let px = captures
            .name("px")
            .unwrap()
            .as_str()
            .parse()
            .context("px")?;
        let py = captures
            .name("py")
            .unwrap()
            .as_str()
            .parse()
            .context("py")?;
        let bx = captures
            .name("bx")
            .unwrap()
            .as_str()
            .parse()
            .context("bx")?;
        let by = captures
            .name("by")
            .unwrap()
            .as_str()
            .parse()
            .context("by")?;

        Ok(Self { px, py, bx, by })
    }
}

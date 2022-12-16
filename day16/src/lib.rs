use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    str::FromStr,
};

use log::{debug, info};
use utils::Matrix;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    valves: Vec<Valve>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let shortest_path = self.calculate_shortest_paths();
        debug!("{shortest_path:?}");

        let answer = self.attempt(self.get_valve("AA"), &shortest_path, 30, &HashMap::new());
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let shortest_paths = self.calculate_shortest_paths();
        debug!("{shortest_paths:?}");

        let training_time = 4;
        let remaining_time = 30 - training_time;

        let (answer, best) = self.attempt5(
            self.get_valve("AA"),
            &shortest_paths,
            remaining_time,
            &HashMap::new(),
            &HashMap::new(),
        );
        info!("attempt4: {answer} {best:?}");
        let (answer, best) = self.attempt5(
            self.get_valve("AA"),
            &shortest_paths,
            remaining_time,
            &best,
            &HashMap::new(),
        );
        info!("attempt4: {answer} {best:?}");
        Ok(answer)
    }
}

impl Solution {
    fn calculate_shortest_paths(&self) -> HashMap<(&str, &str), i64> {
        let mut distances = Matrix::new();
        let mut ids = HashMap::new();
        for (idx, valve) in self.valves.iter().enumerate() {
            ids.insert(valve.valve_id.as_str(), idx);
        }
        loop {
            let mut changed = false;
            for valve in &self.valves {
                let current_valve_idx = *ids.get(valve.valve_id.as_str()).unwrap() as isize;
                distances.set(current_valve_idx, current_valve_idx, 0);
                for neigh_valve_id in &valve.valves {
                    let neigh_valve_idx = *ids.get(&neigh_valve_id.as_str()).unwrap() as isize;
                    if 1 < *distances
                        .get(current_valve_idx, neigh_valve_idx)
                        .unwrap_or(&i64::MAX)
                    {
                        distances.set(current_valve_idx, neigh_valve_idx, 1);
                        changed = true;
                    }
                    for other in ids.values() {
                        if let Some(d) = distances.get(neigh_valve_idx, *other as isize) {
                            if d + 1
                                < *distances
                                    .get(current_valve_idx, *other as isize)
                                    .unwrap_or(&i64::MAX)
                            {
                                distances.set(current_valve_idx, *other as isize, d + 1);
                                changed = true;
                            }
                        }
                    }
                }
            }
            if !changed {
                break;
            }
        }
        debug!("{ids:?}");
        //distances.display();

        let mut id_distance = HashMap::new();
        // Calculate id -> id distances
        for (s_id, s_idx) in ids.iter() {
            for (t_id, t_idx) in ids.iter() {
                id_distance.insert(
                    (*s_id, *t_id),
                    *distances.get(*s_idx as isize, *t_idx as isize).unwrap(),
                );
            }
        }
        id_distance
    }

    fn attempt(
        &self,
        cur_valve: &Valve,
        shortest_paths: &HashMap<(&str, &str), i64>,
        remaining_time: i64,
        closed: &HashMap<&str, i64>,
    ) -> ResultType {
        let mut closed = closed.clone();
        let remaining_time = if cur_valve.flow > 0 {
            closed.insert(cur_valve.valve_id.as_str(), remaining_time - 1);
            remaining_time - 1
        } else {
            remaining_time
        };
        let reachable = self
            .valves
            .iter()
            .filter(|v| v.flow > 0)
            .filter(|v| !closed.contains_key(v.valve_id.as_str()))
            .filter(|v| {
                shortest_paths
                    .get(&(cur_valve.valve_id.as_str(), v.valve_id.as_str()))
                    .unwrap()
                    < &(remaining_time + 1)
            })
            .collect::<Vec<_>>();
        let mut best_cost: i64 = closed
            .iter()
            .map(|(v, remaining)| self.get_valve(v).flow * remaining)
            .sum();
        debug!("{cur_valve:?}: {reachable:?} {closed:?} {best_cost}");
        for next in reachable {
            let remaining_time = remaining_time
                - shortest_paths
                    .get(&(cur_valve.valve_id.as_str(), next.valve_id.as_str()))
                    .unwrap();
            let cost = self.attempt(next, shortest_paths, remaining_time, &closed);
            best_cost = std::cmp::max(best_cost, cost);
        }
        best_cost
    }

    fn attempt5(
        &self,
        cur_valve: &Valve,
        shortest_paths: &HashMap<(&str, &str), i64>,
        remaining_time: i64,
        old_open: &HashMap<String, i64>,
        open: &HashMap<String, i64>,
    ) -> (ResultType, HashMap<String, i64>) {
        let mut open = open.clone();
        let remaining_time = if cur_valve.flow > 0 {
            open.insert(cur_valve.valve_id.clone(), remaining_time - 1);
            remaining_time - 1
        } else {
            remaining_time
        };
        let reachable = self
            .valves
            .iter()
            .filter(|v| v.flow > 0)
            .filter(|v| !open.contains_key(v.valve_id.as_str()))
            .filter(|v| {
                shortest_paths
                    .get(&(cur_valve.valve_id.as_str(), v.valve_id.as_str()))
                    .unwrap()
                    < &(remaining_time + 1)
            })
            .collect::<Vec<_>>();
        let mut best_cost: i64 = self
            .valves
            .iter()
            .map(|v| {
                let id = v.valve_id.clone();
                let flow = v.flow;
                flow * std::cmp::max(open.get(&id).unwrap_or(&0), old_open.get(&id).unwrap_or(&0))
            })
            .sum();
        debug!("{cur_valve:?}: {reachable:?} {open:?} {best_cost}");
        let mut best_opening = open.clone();
        for next in reachable {
            let remaining_time = remaining_time
                - shortest_paths
                    .get(&(cur_valve.valve_id.as_str(), next.valve_id.as_str()))
                    .unwrap();
            let (cost, opening) =
                self.attempt5(next, shortest_paths, remaining_time, old_open, &open);
            if best_cost < cost {
                best_cost = cost;
                best_opening = opening;
            }
        }
        (best_cost, best_opening)
    }

    fn get_valve(&self, id: &str) -> &Valve {
        self.valves
            .iter()
            .find(|valve| valve.valve_id == id)
            .unwrap()
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            let valve = Valve::from_str(&line).unwrap();
            solution.valves.push(valve);
        }
        Ok(solution)
    }
}

#[derive(Debug)]
struct Valve {
    valve_id: String,
    flow: i64,
    valves: Vec<String>,
}

impl FromStr for Valve {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        debug!("{s}");
        let r = regex::Regex::new(r"Valve (?P<id>[A-Z]+) has flow rate=(?P<flow>-?\d+); tunnels? leads? to valves? (?P<valves>.*)$").unwrap();
        let captures = r.captures(s).unwrap();
        let valve_id = captures.name("id").unwrap().as_str().to_string();
        let flow = captures.name("flow").unwrap().as_str().parse().unwrap();
        let valves = captures
            .name("valves")
            .unwrap()
            .as_str()
            .split(", ")
            .map(|s| s.to_string())
            .collect();
        Ok(Self {
            valve_id,
            flow,
            valves,
        })
    }
}

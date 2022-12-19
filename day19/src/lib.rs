use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{BufRead, BufReader},
    str::FromStr,
};

use log::info;
use regex::Regex;

pub type ResultType = i64;

#[derive(Debug, Default)]
pub struct Solution {
    blueprints: Vec<Blueprint>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self, _is_full: bool) {}

    fn answer_part1(&self, _is_full: bool) -> Self::Result {
        let mut answer = 0;
        for blueprint in &self.blueprints {
            let robots = [1, 0, 0, 0];
            let cur_ore = [0, 0, 0, 0];
            let geodes = Self::run_part1(blueprint, 24, robots, cur_ore);
            info!("{} = {}", blueprint.id, geodes);
            answer += geodes * blueprint.id;
        }
        Ok(answer)
    }

    fn answer_part2(&self, _is_full: bool) -> Self::Result {
        let mut answer = 1;
        for blueprint in self.blueprints.iter().take(3) {
            let robots = [1, 0, 0, 0];
            let cur_ore = [0, 0, 0, 0];
            let geodes = Self::run_part1(blueprint, 32, robots, cur_ore);
            info!("{} = {}", blueprint.id, geodes);
            answer *= geodes;
        }
        Ok(answer)
    }
}

impl Solution {
    fn mine(
        robots: [ResultType; 4],
        inventory: [ResultType; 4],
    ) -> ([ResultType; 4], [ResultType; 4]) {
        let mut inventory = inventory;
        inventory[0] += robots[0];
        inventory[1] += robots[1];
        inventory[2] += robots[2];
        inventory[3] += robots[3];
        (robots, inventory)
    }

    fn run_part1(
        blueprint: &Blueprint,
        minutes: ResultType,
        robots: [ResultType; 4],
        inventory: [ResultType; 4],
    ) -> ResultType {
        let ore_index = Material::Ore.index();
        let clay_index = Material::Clay.index();
        let obsidian_index = Material::Obsidian.index();
        let geode_index = Material::Geode.index();

        let ore_ore_cost = blueprint.cost.get(&Material::Ore).unwrap()[ore_index];
        let clay_ore_cost = blueprint.cost.get(&Material::Clay).unwrap()[ore_index];
        let obsidian_ore_cost = blueprint.cost.get(&Material::Obsidian).unwrap()[ore_index];
        let obsidian_clay_cost = blueprint.cost.get(&Material::Obsidian).unwrap()[clay_index];
        let geode_ore_cost = blueprint.cost.get(&Material::Geode).unwrap()[ore_index];
        let geode_obsidian_cost = blueprint.cost.get(&Material::Geode).unwrap()[obsidian_index];

        let max_ore_cost = *blueprint
            .cost
            .iter()
            .map(|(_, [ore, _, _, _])| ore)
            .max()
            .unwrap();
        let mut seen = HashSet::new();
        let mut queue = VecDeque::new();

        queue.push_back((0, robots, inventory));
        let mut answer = 0;
        while let Some((cur_time, robots, inventory)) = queue.pop_front() {
            if cur_time >= minutes {
                answer = std::cmp::max(answer, inventory[geode_index]);
                continue;
            }
            if seen.contains(&(robots, inventory)) {
                continue;
            }
            seen.insert((robots, inventory));
            if robots[ore_index] < max_ore_cost && inventory[ore_index] >= ore_ore_cost {
                let (mut robots, mut inventory) = Self::mine(robots, inventory);
                inventory[ore_index] -= ore_ore_cost;
                robots[ore_index] += 1;
                queue.push_back((cur_time + 1, robots, inventory));
            }
            if robots[clay_index] < obsidian_clay_cost && inventory[ore_index] >= clay_ore_cost {
                let (mut robots, mut inventory) = Self::mine(robots, inventory);
                inventory[ore_index] -= clay_ore_cost;
                robots[clay_index] += 1;
                queue.push_back((cur_time + 1, robots, inventory));
            }
            if robots[obsidian_index] < geode_obsidian_cost
                && inventory[ore_index] >= obsidian_ore_cost
                && inventory[clay_index] >= obsidian_clay_cost
            {
                let (mut robots, mut inventory) = Self::mine(robots, inventory);
                inventory[ore_index] -= obsidian_ore_cost;
                inventory[clay_index] -= obsidian_clay_cost;
                robots[obsidian_index] += 1;
                queue.push_back((cur_time + 1, robots, inventory));
            }
            if inventory[ore_index] >= geode_ore_cost
                && inventory[obsidian_index] >= geode_obsidian_cost
            {
                let (mut robots, mut inventory) = Self::mine(robots, inventory);
                inventory[ore_index] -= geode_ore_cost;
                inventory[obsidian_index] -= geode_obsidian_cost;
                robots[geode_index] += 1;
                queue.push_back((cur_time + 1, robots, inventory));
            } else {
                let (robots, inventory) = Self::mine(robots, inventory);
                queue.push_back((cur_time + 1, robots, inventory));
            }
        }
        answer
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        for line in reader.lines() {
            let line = line?;
            let blueprint = Blueprint::from_str(&line).unwrap();
            solution.blueprints.push(blueprint);
        }
        Ok(solution)
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    id: i64,
    cost: HashMap<Material, [ResultType; 4]>,
}

impl FromStr for Blueprint {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r_bp = Regex::new(r"^Blueprint (?P<id>\d+)$").unwrap();
        let r_cost = Regex::new(r"^Each (?P<name>[^ ]+) robot costs (?P<costs>.*)$").unwrap();

        let (lhs, rhs) = s.split_once(": ").unwrap();
        let c = r_bp.captures(lhs).unwrap();
        let id = c.name("id").unwrap().as_str().parse().unwrap();

        let cost = rhs.split(". ").fold(HashMap::new(), |mut a, v| {
            let captures = r_cost.captures(v).unwrap();
            let name = Material::from_str(captures.name("name").unwrap().as_str()).unwrap();
            let costs = captures
                .name("costs")
                .unwrap()
                .as_str()
                .split(" and ")
                .fold([0; 4], |mut acc, cs| {
                    let (l, r) = cs.split_once(' ').unwrap();
                    let nom = Material::from_str(r.trim_end_matches('.')).unwrap();
                    let value = l.parse().unwrap();
                    acc[nom.index()] = value;
                    acc
                });
            a.insert(name, costs);
            a
        });

        Ok(Self { id, cost })
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Material {
    Ore,
    Clay,
    Obsidian,
    Geode,
}

impl Material {
    fn index(&self) -> usize {
        match self {
            Material::Ore => 0,
            Material::Clay => 1,
            Material::Obsidian => 2,
            Material::Geode => 3,
        }
    }
}

impl FromStr for Material {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let r = match s {
            "ore" => Self::Ore,
            "clay" => Self::Clay,
            "obsidian" => Self::Obsidian,
            "geode" => Self::Geode,
            _ => unreachable!(),
        };
        Ok(r)
    }
}

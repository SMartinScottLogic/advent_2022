use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

use log::{debug, info, trace};

type ResultType = i64;
#[derive(Debug, Default)]
pub struct Solution {
    monkeys: Vec<Monkey>,

    answer_part1: ResultType,
    answer_part2: ResultType,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<ResultType>;
    fn analyse(&mut self) {
        let monkeys = self.monkeys.clone();
        self.answer_part1 = self.analyse_part(3, 20);
        self.monkeys = monkeys;
        self.answer_part2 = self.analyse_part(1, 10000);
    }

    fn answer_part1(&self) -> Self::Result {
        Ok(self.answer_part1)
    }

    fn answer_part2(&self) -> Self::Result {
        Ok(self.answer_part2)
    }
}
impl Solution {
    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    fn analyse_part(&mut self, worry_scale: ResultType, num_rounds: usize) -> ResultType {
        let mut inspections = HashMap::new();
        let mut monkey_ids: Vec<_> = Vec::new();
        let mut total_div = 1;
        for monkey in &self.monkeys {
            total_div *= monkey.divisibility_test;
            monkey_ids.push(monkey.id);
        }
        for _ in 1..=num_rounds {
            for monkey_id in &monkey_ids {
                let monkey = self.monkeys.get(*monkey_id).unwrap().clone();
                *inspections.entry(monkey_id).or_insert(0) += monkey.levels.len();
                debug!("Monkey {}: {:?}", monkey.id, monkey.levels);
                debug!("   {:?}", monkey);
                for level in &monkey.levels {
                    debug!("Monkey inspects an item with a worry level of {}", level);
                    let mut new_level = monkey.operation.calculate(*level);
                    debug!("Worry level from {} to {}", level, new_level);
                    new_level /= worry_scale;
                    debug!(
                        "Monkey gets bored with item. Worry level is divided by 3 to {}",
                        new_level
                    );
                    new_level %= total_div;
                    let r = (new_level % monkey.divisibility_test) == 0;
                    debug!(
                        "Current worry level {} divisible by {}",
                        if r { "is" } else { "is not" },
                        monkey.divisibility_test
                    );
                    let next_monkey = if r {
                        debug!(
                            "Item with worry level {} is thrown to monkey {}",
                            new_level, monkey.next_monkey_true
                        );
                        monkey.next_monkey_true
                    } else {
                        debug!(
                            "Item with worry level {} is thrown to monkey {}",
                            new_level, monkey.next_monkey_false
                        );
                        monkey.next_monkey_false
                    };
                    self.monkeys[next_monkey].levels.push(new_level);
                }
                self.monkeys[*monkey_id].levels.clear();
            }
        }
        info!("inspections: {:?}", inspections);
        let mut largest1 = 0;
        let mut largest2 = 0;
        for (_, i) in inspections {
            if i > largest1 {
                if largest1 > largest2 {
                    largest2 = largest1;
                }
                largest1 = i;
            } else if i > largest2 {
                largest2 = i;
            }
        }
        info!("{largest1} {largest2}");
        largest1 as ResultType * largest2 as ResultType
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self::default();
        let mut current_monkey = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim().to_string();
            if line.is_empty() {
                let monkey = Monkey::from(current_monkey);
                solution.add_monkey(monkey);
                current_monkey = Vec::new();
            } else {
                current_monkey.push(line);
            }
        }
        let monkey = Monkey::from(current_monkey);
        solution.add_monkey(monkey);
        Ok(solution)
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    levels: Vec<ResultType>,
    operation: Operation,
    divisibility_test: ResultType,
    next_monkey_true: usize,
    next_monkey_false: usize,
}

impl From<Vec<String>> for Monkey {
    fn from(lines: Vec<String>) -> Self {
        trace!("{:?}", lines);

        let mut id = 0;
        let mut levels = Vec::new();
        let mut operation = Operation::None;
        let mut divisibility_test = 0;
        let mut next_monkey_true = 0;
        let mut next_monkey_false = 0;
        for (i, line) in lines.iter().enumerate() {
            match i {
                0 => {
                    id = lines[0]
                        .chars()
                        .filter(|c| c.is_numeric())
                        .collect::<String>()
                        .parse()
                        .unwrap()
                }
                1 => {
                    levels = line
                        .split(':')
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|s| s.trim().parse::<ResultType>().unwrap())
                        .collect()
                }
                2 => {
                    operation = Operation::from(line.split(':').map(|s| s.trim()).nth(1).unwrap());
                }
                3 => {
                    let skip = "Test: divisible by ";
                    assert!(line.starts_with(skip));
                    divisibility_test = line
                        .chars()
                        .skip(skip.len())
                        .collect::<String>()
                        .parse()
                        .unwrap();
                }
                _ => {
                    let true_skip = "If true: throw to monkey ";
                    let false_skip = "If false: throw to monkey ";
                    if line.starts_with(true_skip) {
                        next_monkey_true = line
                            .chars()
                            .skip(true_skip.len())
                            .collect::<String>()
                            .parse()
                            .unwrap();
                    } else if line.starts_with(false_skip) {
                        next_monkey_false = line
                            .chars()
                            .skip(false_skip.len())
                            .collect::<String>()
                            .parse()
                            .unwrap();
                    } else {
                        unreachable!()
                    }
                }
            }
        }
        Self {
            id,
            levels,
            operation,
            divisibility_test,
            next_monkey_true,
            next_monkey_false,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    None,
    Add(ResultType),
    Multiply(ResultType),
    Square,
}

impl Operation {
    fn calculate(&self, value: ResultType) -> ResultType {
        match self {
            Operation::None => unreachable!(),
            Operation::Add(v) => value + v,
            Operation::Multiply(v) => value * v,
            Operation::Square => value * value,
        }
    }
}

impl From<&str> for Operation {
    fn from(value: &str) -> Self {
        let parts: Vec<_> = value.split_whitespace().collect();
        assert_eq!(parts[0], "new");
        assert_eq!(parts[1], "=");
        assert_eq!(parts[2], "old");
        if parts[4] == "old" {
            assert_eq!(parts[3], "*");
            Self::Square
        } else if parts[3] == "*" {
            Self::Multiply(parts[4].parse().unwrap())
        } else if parts[3] == "+" {
            Self::Add(parts[4].parse().unwrap())
        } else {
            Self::None
        }
    }
}

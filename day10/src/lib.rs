use std::{
    io::{BufRead, BufReader},
    str::FromStr,
};

use log::info;
use utils::Matrix;

#[derive(Debug)]
pub struct Solution {
    instructions: Vec<Instruction>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<i64>;
    fn analyse(&mut self) {}

    fn answer_part1(&self) -> Self::Result {
        let mut answer = 0;
        let mut cycle = 0;
        let mut reg_x = 1;
        for instruction in &self.instructions {
            let prev_cycle = cycle;
            let prev_x = reg_x;
            match instruction {
                Instruction::Noop => cycle += 1,
                Instruction::Addx(v) => {
                    reg_x += v;
                    cycle += 2;
                }
            }
            for i in 0..=5 {
                let p = 20 + 40 * i;
                if prev_cycle < p && cycle >= p {
                    answer += prev_x * p;
                }
            }
        }
        // Implement for proble
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let mut crt = Matrix::new();
        let mut reg_x = 1;
        let mut x = 0;
        let mut y = 0;

        for instruction in &self.instructions {
            let steps = match instruction {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            };
            for _ in 0..steps {
                if (x as i64) >= (reg_x - 1) && (x as i64) < (reg_x + 2) {
                    crt.set(x, y, 1);
                    info!("@({},{}): {} {} = 1", x, y, x, reg_x);
                } else {
                    crt.set(x, y, 0);
                    info!("@({},{}): {} {} = 0", x, y, x, reg_x);
                }
                x += 1;
                if x % 40 == 0 {
                    x = 0;
                    y += 1;
                }
            }
            if let Instruction::Addx(v) = instruction {
                reg_x += v;
            }
        }
        crt.display_with_mapping(|v| {
            if v == 1 {
                "#".to_string()
            } else {
                ".".to_string()
            }
        });
        Ok(0)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            instructions: Vec::new(),
        };
        for line in reader.lines() {
            let line = line?;
            solution
                .instructions
                .push(Instruction::from_str(&line).unwrap());
        }
        Ok(solution)
    }
}

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl FromStr for Instruction {
    type Err = std::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split_whitespace();
        let instr = i.next().unwrap();
        let val = i.next();
        let r = match instr {
            "noop" => Self::Noop,
            "addx" => Self::Addx(val.unwrap().parse().unwrap()),
            _ => unreachable!(),
        };
        Ok(r)
    }
}

use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use log::{debug, trace};

#[derive(Debug)]
pub struct Solution {
    commands: Vec<(Command, Vec<String>)>,
    treesize: HashMap<String, u64>,
}

impl Solution {
    fn treesize(&self) -> HashMap<String, u64> {
        let mut size: HashMap<String, u64> = HashMap::new();
        let mut cwd: Vec<String> = Vec::new();
        for (command, output) in &self.commands {
            match command.command.as_str() {
                "ls" => {
                    if !command.parameters.is_empty() {
                        panic!("Unexpected parameters to ls");
                    }
                    let dirsize: u64 = output
                        .iter()
                        .map(|s| s.split_whitespace().next().unwrap())
                        .filter(|s| !((*s).eq("dir")))
                        .map(|size| size.parse::<u64>().unwrap())
                        .sum();
                    trace!("filesize {} from {:?}", dirsize, output);
                    let mut fullpath = PathBuf::new();
                    for dir in &cwd {
                        fullpath.push(dir);
                        let e = size
                            .entry(fullpath.to_str().unwrap().to_string())
                            .or_default();
                        *e += dirsize;
                        trace!("size {} => {}", dir, *e);
                    }
                }
                "cd" => {
                    if command.parameters.len() != 1 {
                        panic!("cd MUST have 1 parameter (only)");
                    }
                    match command.parameters.get(0).unwrap().as_str() {
                        "/" => cwd = vec!["/".to_string()],
                        ".." => {
                            cwd.pop();
                        }
                        s => cwd.push(s.to_string()),
                    }
                    trace!("{:?} => {:?}", command, cwd);
                }
                _ => unreachable!(),
            }
        }
        size
    }
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<u64>;
    fn analyse(&mut self) {
        self.treesize = self.treesize();
        debug!("sizes {:?}", self.treesize());
    }

    fn answer_part1(&self) -> Self::Result {
        let answer = self
            .treesize
            .iter()
            .map(|(_, s)| *s)
            .filter(|s| *s <= 100000)
            .sum();
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let unuseddisk = 70000000 - self.treesize.get("/").unwrap();
        let required = 30000000 - unuseddisk;
        debug!("unused={} required={}", unuseddisk, required);

        let answer = self
            .treesize
            .iter()
            .filter(|(_e, s)| *s >= &required)
            .map(|e| e.1)
            .min()
            .unwrap()
            .to_owned();
        Ok(answer)
    }
}

impl<T: std::io::Read> TryFrom<BufReader<T>> for Solution {
    type Error = std::io::Error;

    fn try_from(reader: BufReader<T>) -> Result<Self, Self::Error> {
        let mut solution = Self {
            commands: Vec::new(),
            treesize: HashMap::new(),
        };
        let mut current_command = Command::default();
        let mut current_output = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.starts_with('$') {
                if !current_command.command.is_empty() {
                    solution.commands.push((current_command, current_output));
                }
                current_command = Command::from(line.to_string());
                current_output = Vec::new();
            } else {
                current_output.push(line.to_string());
            }
            // Implement for problem
        }
        if !current_command.command.is_empty() {
            solution.commands.push((current_command, current_output));
        }
        Ok(solution)
    }
}

#[derive(Debug, Default)]
struct Command {
    command: String,
    parameters: Vec<String>,
}

impl From<String> for Command {
    fn from(input: String) -> Self {
        let mut line = input.split_whitespace().skip(1).map(|s| s.to_string());
        let command = line.next().unwrap();
        let parameters = line.collect::<Vec<_>>();
        Self {
            command,
            parameters,
        }
    }
}

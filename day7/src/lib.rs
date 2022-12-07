use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    path::PathBuf,
};

use log::{debug, trace};

#[derive(Debug)]
pub struct Solution {
    commands: Vec<((String, Vec<String>), Vec<String>)>,
}

impl utils::Solution for Solution {
    type Result = anyhow::Result<u64>;
    fn analyse(&mut self) {}

    fn answer_part1(&self) -> Self::Result {
        let mut cwd: Vec<String> = Vec::new();
        let mut size: HashMap<String, u64> = HashMap::new();
        for ((command, parameters), output) in &self.commands {
            match command.as_str() {
                "ls" => {
                    if !parameters.is_empty() {
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
                    if parameters.len() != 1 {
                        panic!("cd MUST have 1 parameter (only)");
                    }
                    match parameters.get(0).unwrap().as_str() {
                        "/" => cwd = vec!["/".to_string()],
                        ".." => {
                            cwd.pop();
                        }
                        s => cwd.push(s.to_string()),
                    }
                    trace!("cd {:?} => {:?}", parameters, cwd);
                }
                _ => unreachable!(),
            }
        }
        debug!("sizes {:?}", size);
        let answer = size.iter().map(|(_, s)| *s).filter(|s| *s <= 100000).sum();
        Ok(answer)
    }

    fn answer_part2(&self) -> Self::Result {
        let mut cwd: Vec<String> = Vec::new();
        let mut size: HashMap<String, u64> = HashMap::new();
        for ((command, parameters), output) in &self.commands {
            match command.as_str() {
                "ls" => {
                    if !parameters.is_empty() {
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
                    if parameters.len() != 1 {
                        panic!("cd MUST have 1 parameter (only)");
                    }
                    match parameters.get(0).unwrap().as_str() {
                        "/" => cwd = vec!["/".to_string()],
                        ".." => {
                            cwd.pop();
                        }
                        s => cwd.push(s.to_string()),
                    }
                    trace!("cd {:?} => {:?}", parameters, cwd);
                }
                _ => unreachable!(),
            }
        }
        let unuseddisk = 70000000 - size.get("/").unwrap();
        let required = 30000000 - unuseddisk;
        debug!("unused={} required={}", unuseddisk, required);

        let answer = size
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
        };
        let mut current_command = (String::new(), Vec::new());
        let mut current_output = Vec::new();
        for line in reader.lines() {
            let line = line?;
            let line = line.trim();
            if line.starts_with('$') {
                if !current_command.0.is_empty() {
                    solution.commands.push((current_command, current_output));
                }
                let mut line = line.split_whitespace().skip(1).map(|s| s.to_string());
                let command = line.next().unwrap();
                let parameters = line.collect::<Vec<_>>();
                current_command = (command, parameters);
                current_output = Vec::new();
            } else {
                current_output.push(line.to_string());
            }
            // Implement for problem
        }
        if !current_command.0.is_empty() {
            solution.commands.push((current_command, current_output));
        }
        Ok(solution)
    }
}

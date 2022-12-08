use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day07 {
    input: Vec<String>,
    cumulative_sizes: HashMap<String, usize>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            input: Vec::new(),
            cumulative_sizes: HashMap::new(),
        };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut cwd = VecDeque::<&str>::new();
        cwd.push_back("/");
        self.cumulative_sizes.insert("/".to_string(), 0);

        for line in &self.input {
            let parts: Vec<&str> = line.split(' ').collect();
            if parts[1] == "cd" {
                match parts[2] {
                    ".." => {
                        cwd.pop_back();
                    }
                    "/" => {
                        cwd.clear();
                        cwd.push_back("/");
                    }
                    d => {
                        cwd.push_back(d);
                    }
                }
            } else {
                match parts[0].parse::<usize>() {
                    Ok(sz) => {
                        let mut k = String::new();
                        for i in 0..cwd.len() {
                            k += cwd[i];
                            self.cumulative_sizes
                                .entry(k.to_string())
                                .and_modify(|x| *x += sz)
                                .or_insert(sz);
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut total = 0;
        for v in self.cumulative_sizes.values() {
            if v <= &100000 {
                total += v;
            }
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let required_space = 30000000 - (70000000 - self.cumulative_sizes["/"]);
        let mut min = usize::MAX;
        for sz in self.cumulative_sizes.values().filter(|x| *x >= &required_space) {
            min = min.min(*sz);
        }
        return Box::new(min);
    }
}

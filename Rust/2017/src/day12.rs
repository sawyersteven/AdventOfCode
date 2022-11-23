use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day12 {
    pub progs: HashMap<String, Vec<String>>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            progs: HashMap::<String, Vec<String>>::new(),
        };
    }

    fn prog_group(&self, seed: &str) -> HashSet<String> {
        let mut good_progs = HashSet::<String>::new();
        good_progs.insert(seed.to_string());
        loop {
            let c = good_progs.len();
            for (k, v) in &self.progs {
                if good_progs.contains(k) {
                    good_progs.extend(v.iter().map(|x| x.to_string()));
                }
            }
            if good_progs.len() == c {
                break;
            }
        }
        return good_progs;
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.progs.insert(
                line.split(' ').nth(0).unwrap().to_string(),
                line.split("> ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|x| x.to_string())
                    .collect(),
            );
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        return Box::new(self.prog_group("0").len());
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut group_count = 0;
        let mut consumed = HashSet::<String>::new();

        for k in self.progs.keys() {
            if consumed.contains(k) {
                continue;
            }

            group_count += 1;
            consumed.extend(self.prog_group(k));
        }

        return Box::new(group_count);
    }
}

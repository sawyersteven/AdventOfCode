use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day06 {
    groups: Vec<String>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { groups: Vec::new() };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.groups = raw_input.split("\n\n").map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        let mut hs: HashSet<u8> = HashSet::new();
        for group in &self.groups {
            for line in group.lines() {
                hs.extend(line.as_bytes())
            }
            total += hs.len();
            hs.clear()
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;

        for group in &self.groups {
            let mut hs = HashSet::new();
            let mut lines = group.lines();
            hs.extend(lines.next().unwrap().as_bytes());
            while let Some(l) = lines.next() {
                hs = hs
                    .intersection(&l.as_bytes().iter().cloned().collect())
                    .cloned()
                    .collect();
            }
            total += hs.len();
        }

        return Box::new(total);
    }
}

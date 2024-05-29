use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day01 {
    pub input: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input.push(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for n in &self.input {
            sum += n;
        }
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut h = HashSet::new();
        let mut sum = 0;
        let mut ind = 0;
        loop {
            if ind == self.input.len() {
                ind = 0;
            }
            sum += self.input[ind];
            if h.contains(&sum) {
                break;
            }
            h.insert(sum);
            ind += 1;
        }
        return Box::new(sum);
    }
}

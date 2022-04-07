use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    pub input: Vec<char>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut floor = 0;
        for c in &self.input {
            floor += if c == &'(' { 1 } else { -1 };
        }

        return Box::new(floor);
    }
    fn part2(&mut self) -> Box<dyn Display> {
        let mut floor = 0;
        for i in 0..self.input.len() {
            floor += if self.input[i] == '(' { 1 } else { -1 };
            if floor < 0 {
                return Box::new(i + 1);
            }
        }
        return Box::new(-1);
    }
}

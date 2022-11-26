use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    pub input: String,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: String::from("") };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

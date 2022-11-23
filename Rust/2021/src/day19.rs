use crate::Base;
use std::fmt::Display;

pub struct Day19 {
    pub input: String,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            input: String::from(""),
        };
    }
}

impl Base for Day19 {
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

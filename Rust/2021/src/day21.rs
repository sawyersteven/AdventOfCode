use crate::Base;
use std::fmt::Display;

pub struct Day21 {
    pub input: String,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            input: String::from(""),
        };
    }
}

impl Base for Day21 {
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

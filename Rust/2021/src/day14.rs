use crate::Base;
use std::fmt::Display;

pub struct Day14 {
    pub input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 {
            input: String::from(""),
        };
    }
}

impl Base for Day14 {
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

use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    pub input: String,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 {
            input: String::from(""),
        };
    }
}

impl Base for Day02 {
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

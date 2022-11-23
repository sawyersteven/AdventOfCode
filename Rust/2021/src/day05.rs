use crate::Base;
use std::fmt::Display;

pub struct Day05 {
    pub input: String,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            input: String::from(""),
        };
    }
}

impl Base for Day05 {
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

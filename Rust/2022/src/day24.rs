use crate::Base;
use std::fmt::Display;

pub struct Day24 {
    pub input: String,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            input: String::from(""),
        };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

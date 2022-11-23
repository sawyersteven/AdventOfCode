use crate::Base;
use std::fmt::Display;

pub struct Day12 {
    pub input: String,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            input: String::from(""),
        };
    }
}

impl Base for Day12 {
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

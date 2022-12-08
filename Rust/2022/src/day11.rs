use crate::Base;
use std::fmt::Display;

pub struct Day11 {
    pub input: String,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            input: String::from(""),
        };
    }
}

impl Base for Day11 {
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

use crate::Base;
use std::fmt::Display;

pub struct Day13 {
    pub input: String,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 { input: String::from("") };
    }
}

impl Base for Day13 {
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

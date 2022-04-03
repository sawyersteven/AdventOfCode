use crate::Base;
use std::fmt::Display;

pub struct Day22 {
    pub input: String,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 { input: String::from("") };
    }
}

impl Base for Day22 {
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

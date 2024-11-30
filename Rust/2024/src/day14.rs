use crate::Base;
use std::fmt::Display;

pub struct Day14 {
    input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { input: String::new() };
    }
}

impl Base for Day14 {
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

use crate::Base;
use std::fmt::Display;

pub struct Day17 {
    input: String,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 { input: String::new() };
    }
}

impl Base for Day17 {
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

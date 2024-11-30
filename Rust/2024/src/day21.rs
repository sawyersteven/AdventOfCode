use crate::Base;
use std::fmt::Display;

pub struct Day21 {
    input: String,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 { input: String::new() };
    }
}

impl Base for Day21 {
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

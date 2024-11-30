use crate::Base;
use std::fmt::Display;

pub struct Day20 {
    input: String,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { input: String::new() };
    }
}

impl Base for Day20 {
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

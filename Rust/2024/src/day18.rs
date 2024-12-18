use crate::Base;
use std::fmt::Display;

pub struct Day18 {
    input: String,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { input: String::new() };
    }
}

impl Base for Day18 {
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

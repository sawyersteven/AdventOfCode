use crate::Base;
use std::fmt::Display;

pub struct Day23 {
    input: String,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: String::new() };
    }
}

impl Base for Day23 {
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

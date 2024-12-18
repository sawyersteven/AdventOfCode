use crate::Base;
use std::fmt::Display;

pub struct Day16 {
    input: String,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 { input: String::new() };
    }
}

impl Base for Day16 {
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

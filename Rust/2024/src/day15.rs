use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { input: String::new() };
    }
}

impl Base for Day15 {
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

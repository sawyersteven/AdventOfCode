use crate::Base;
use std::fmt::Display;

pub struct Day09 {
    pub input: String,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: String::from("") };
    }
}

impl Base for Day09 {
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

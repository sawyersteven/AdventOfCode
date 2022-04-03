use crate::Base;
use std::fmt::Display;

pub struct Day25 {
    pub input: String,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { input: String::from("") };
    }
}

impl Base for Day25 {
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

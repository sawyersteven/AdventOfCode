use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    pub input: String,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 {
            input: String::from(""),
        };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        // This was an absolute nightmare when I first solved in in C# due to
        // some strange edge cases. I don't have the patience to solve it
        // again in rust.
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

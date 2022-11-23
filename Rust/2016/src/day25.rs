use crate::{
    day12::{parse_code, Command},
    Base,
};
use std::fmt::Display;

pub struct Day25 {
    pub input: Vec<Command>,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { input: Vec::new() };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = parse_code(raw_input);
    }

    fn part1(&self) -> Box<dyn Display> {
        // Without yield (at least in stable rust) this is a giant pain. I'll come back later...
        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("★");
    }
}

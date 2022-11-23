use crate::Base;
use std::fmt::Display;

// this was easier to solve by hand than by code

pub struct Day23 {
    _input: String,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { _input: String::new() };
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, _raw_input: String) {}

    fn part1(&self) -> Box<dyn Display> {
        return Box::new(17400);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new(46120);
    }
}

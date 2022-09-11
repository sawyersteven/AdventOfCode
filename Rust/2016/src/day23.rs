use crate::{
    day12::{parse_code, run_code, Command},
    Base,
};
use std::fmt::Display;

pub struct Day23 {
    pub input: Vec<Command>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: Vec::new() };
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = parse_code(raw_input);
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut registers: [isize; 4] = [7, 0, 0, 0];
        run_code(self.input.clone(), &mut registers);
        return Box::new(registers[0]);
    }

    // Didn't feel like optimizing. This runs in 7 seconds in release mode
    fn part2(&self) -> Box<dyn Display> {
        let mut registers: [isize; 4] = [12, 0, 0, 0];
        run_code(self.input.clone(), &mut registers);
        return Box::new(registers[0]);
    }
}

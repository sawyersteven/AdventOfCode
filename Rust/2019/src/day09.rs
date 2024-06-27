use crate::{
    intcode::{self, Emulator},
    Base,
};
use std::fmt::Display;

pub struct Day09 {
    input: Vec<isize>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: Vec::new() };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emu = Emulator::new(self.input.clone());
        emu.queue_input(&[1]);
        return Box::new(emu.run().1);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut emu = Emulator::new(self.input.clone());
        emu.queue_input(&[2]);
        return Box::new(emu.run().1);
    }
}

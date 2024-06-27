use crate::{
    intcode::{self, Emulator, StatusCode},
    Base,
};
use std::fmt::Display;

pub struct Day05 {
    input: Vec<isize>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 { input: Vec::new() };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());
        let mut prev_resp = (StatusCode::Null, 0);
        loop {
            emulator.queue_input(&[1]);
            let resp = emulator.run();
            if resp.0 == StatusCode::Complete {
                return Box::new(prev_resp.1);
            }
            prev_resp = resp;
        }
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());
        let mut prev_resp = (StatusCode::Null, 0);

        loop {
            emulator.queue_input(&[5]);
            let resp = emulator.run();
            if resp.0 == StatusCode::Complete {
                return Box::new(prev_resp.1);
            }
            prev_resp = resp;
        }
    }
}

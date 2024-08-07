use crate::{
    intcode::{self, Emulator},
    Base,
};
use std::fmt::Display;

const GRID_SIZE: isize = 50;
const SHIP_SIZE: isize = 100;

pub struct Day19 {
    input: Vec<isize>,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 { input: Vec::new() };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());

        let mut counter = 0;
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                emulator.queue_input(&[x, y]);
                counter += emulator.run().1;
                emulator.reboot();
            }
        }
        return Box::new(counter);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());

        let mut y = SHIP_SIZE;
        let mut x = 0;

        let mut changes = true;
        while changes {
            changes = false;
            // move down until top-right in field
            loop {
                emulator.reboot();
                emulator.queue_input(&[x + SHIP_SIZE - 1, y]);
                if emulator.run().1 == 1 {
                    break;
                }
                y += 1;
                changes = true;
            }

            // move right until bottom-left in field
            loop {
                emulator.reboot();
                emulator.queue_input(&[x, y + SHIP_SIZE - 1]);
                if emulator.run().1 == 1 {
                    break;
                }
                x += 1;
                changes = true;
            }
        }

        return Box::new((x * 10000) + y);
    }
}

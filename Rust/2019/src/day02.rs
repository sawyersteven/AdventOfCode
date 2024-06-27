use crate::{
    intcode::{self, Emulator},
    Base,
};
use std::fmt::Display;

pub struct Day02 {
    intcode: Vec<isize>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { intcode: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.intcode = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut code = self.intcode.clone();
        code[1] = 12;
        code[2] = 2;

        let mut emulator = Emulator::new(code);
        let _ = emulator.run();

        return Box::new(emulator.read(0));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const REQ_OUT: isize = 19690720;

        let code = self.intcode.clone();
        let mut emulator = Emulator::new(code);

        for verb in (0..100).rev() {
            for noun in 0..100 {
                emulator.write(noun, 1);
                emulator.write(verb, 2);
                emulator.run();
                if emulator.read(0) == REQ_OUT {
                    return Box::new((100 * noun) + verb);
                }
                emulator.reboot();
            }
        }
        return Box::new("-");
    }
}

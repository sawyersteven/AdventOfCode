use crate::{
    intcode::{self, Emulator, StatusCode},
    Base,
};
use std::fmt::Display;

const SPRING_SCRIPT_1: [&str; 7] = ["NOT B J", "NOT C T", "OR T J", "AND D J", "NOT A T", "OR T J", "WALK"];
const SPRING_SCRIPT_2: [&str; 8] = [
    "NOT B J", "NOT C T", "OR T J", "AND D J", "AND H J", "NOT A T", "OR T J", "RUN",
];
const LINEFEED: isize = 10;

pub struct Day21 {
    code: Vec<isize>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 { code: Vec::new() };
    }

    fn send_string(&self, emu: &mut intcode::Emulator, instructions: &str) -> isize {
        let mut inst: Vec<isize> = instructions.bytes().map(|x| x as isize).collect();
        inst.push(LINEFEED);
        emu.queue_input(&inst);
        return emu.run().1;
    }

    fn run_spring_script(&self, script: &[&str]) -> Vec<isize> {
        let mut emulator = Emulator::new(self.code.clone());

        let mut out_buffer = Vec::new();

        loop {
            let resp = emulator.run();
            if resp.0 == StatusCode::OutputDelivery {
                out_buffer.push(resp.1);
            } else {
                break;
            }
        }

        for line in script {
            self.send_string(&mut emulator, line);
        }

        out_buffer.clear();
        loop {
            let resp = emulator.run();
            if resp.0 == StatusCode::OutputDelivery {
                out_buffer.push(resp.1);
            } else {
                break;
            }
        }

        return out_buffer;
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        self.code = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let output = self.run_spring_script(&SPRING_SCRIPT_1);

        let last = output[output.len() - 1];
        if last > 128 {
            return Box::new(last);
        }
        return Box::new("?");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let output = self.run_spring_script(&SPRING_SCRIPT_2);

        let last = output[output.len() - 1];
        if last > 128 {
            return Box::new(last);
        }
        return Box::new("?");
    }
}

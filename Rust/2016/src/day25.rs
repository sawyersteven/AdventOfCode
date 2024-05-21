use crate::{
    day12::{parse_code, CodeRunner, Command},
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
        const SEARCH_DEPTH: isize = 1000;
        const MIN_PATTERN: isize = 1000;

        // Without yield (at least in stable rust) this is a giant pain. I'll come back later...
        for a in 0..SEARCH_DEPTH {
            let mut i = 0;
            let mut prev_sig = true;
            let mut code_runner = CodeRunner::new(self.input.clone());
            code_runner.registers[0] = a;
            loop {
                match code_runner.next() {
                    Some(out_val) => {
                        let new_sig = if out_val == 0 { false } else { true };
                        if new_sig == prev_sig {
                            break;
                        }
                        prev_sig = !prev_sig;
                        if i == MIN_PATTERN {
                            return Box::new(a);
                        }
                        i += 1;
                    }
                    None => {
                        break;
                    }
                }
            }
        }

        return Box::new("?");
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("★");
    }
}

use crate::Base;
use std::fmt::Display;

pub struct Day05 {
    pub input: Vec<isize>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 { input: Vec::new() };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut instructions = self.input.clone();

        let mut loc: isize = 0;
        let mut steps = 0;

        let limit: isize = (instructions.len() as isize) - 1;

        loop {
            if loc < 0 || loc > limit {
                break;
            }
            let old_loc = loc as usize;
            loc += instructions[loc as usize];
            instructions[old_loc] += 1;
            steps += 1;
        }

        return Box::new(steps);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut instructions = self.input.clone();

        let mut loc: isize = 0;
        let mut steps = 0;

        let limit: isize = (instructions.len() as isize) - 1;

        loop {
            if loc < 0 || loc > limit {
                break;
            }
            let old_loc = loc as usize;
            loc += instructions[loc as usize];

            if instructions[old_loc] >= 3 {
                instructions[old_loc] -= 1;
            } else {
                instructions[old_loc] += 1;
            }
            steps += 1;
        }

        return Box::new(steps);
    }
}

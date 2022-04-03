use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    pub input: Vec<Vec<usize>>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split("\n") {
            let mut b: Vec<usize> = line
                .split("x")
                .map(|x| x.trim().parse().unwrap())
                .collect::<Vec<usize>>();

            b.sort();
            let _ = &self.input.push(b);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sqft = 0;
        for b in &self.input {
            sqft += (b[0] * b[1] * 3) + (b[1] * b[2] * 2) + (b[0] * b[2] * 2);
        }
        return Box::new(sqft);
    }
    fn part2(&self) -> Box<dyn Display> {
        let mut ribbon = 0;
        for b in &self.input {
            ribbon += (b[0] * 2) + (b[1] * 2) + (b[0] * b[1] * b[2]);
        }
        return Box::new(ribbon);
    }
}

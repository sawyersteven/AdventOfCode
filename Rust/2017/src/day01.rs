use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    pub input: Vec<usize>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().map(|f| f.to_digit(10).unwrap() as usize).collect();
        println!("{}", self.input[2]);
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for i in 0..(self.input.len() - 1) {
            if self.input[i] == self.input[i + 1] {
                sum += self.input[i];
            }
        }
        if self.input[0] == self.input[self.input.len() - 1] {
            sum += self.input[0];
        }

        return Box::new(sum);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut sum = 0;
        let half = self.input.len() / 2;

        for i in 0..self.input.len() {
            if self.input[i] == self.input[(i + half) % self.input.len()] {
                sum += self.input[i];
            }
        }

        return Box::new(sum);
    }
}

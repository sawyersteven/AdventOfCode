use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    lines: Vec<String>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { lines: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.lines = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let sum = self
            .lines
            .iter()
            .map(|x| x.parse::<isize>().unwrap() / 3 - 2)
            .sum::<isize>();
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for line in &self.lines {
            let mut mass: isize = line.parse().unwrap();
            loop {
                mass = mass / 3 - 2;
                if mass <= 0 {
                    break;
                }
                total += mass;
            }
        }

        return Box::new(total);
    }
}

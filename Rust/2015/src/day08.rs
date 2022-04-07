use crate::Base;
use std::fmt::Display;

pub struct Day08 {
    pub input: Vec<Vec<char>>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: Vec::new() };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split("\n") {
            self.input.push(line.chars().collect());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let mut i = 0;
            while i < line.len() {
                if line[i] == '\\' {
                    let inc = match line[i + 1] {
                        '\\' => 1,
                        '"' => 1,
                        'x' => 3,
                        _ => 0,
                    };
                    count += inc;
                    i += inc;
                }
                i += 1;
            }
            count += 2;
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;

        for line in &self.input {
            count += 2;
            count += line.iter().filter(|x| **x == '\\' || **x == '"').count();
        }
        return Box::new(count);
    }
}

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day06 {
    pub input: Vec<Vec<char>>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { input: Vec::new() };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.chars().collect()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut corrected: Vec<char> = Vec::with_capacity(self.input[0].len());
        for _ in 0..self.input[0].len() {
            corrected.push('_');
        }

        for i in 0..corrected.len() {
            let mut counts: HashMap<char, usize> = HashMap::new();
            for line in &self.input {
                counts.entry(line[i]).and_modify(|v| *v += 1).or_insert(0);
            }

            let mut best = 0;
            let mut best_char = ' ';
            for (k, v) in counts {
                if v > best {
                    best = v;
                    best_char = k;
                }
            }
            corrected[i] = best_char;
        }

        return Box::new(String::from_iter(corrected));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut corrected: Vec<char> = Vec::with_capacity(self.input[0].len());
        for _ in 0..self.input[0].len() {
            corrected.push('_');
        }

        for i in 0..corrected.len() {
            let mut counts: HashMap<char, usize> = HashMap::new();
            for line in &self.input {
                counts.entry(line[i]).and_modify(|v| *v += 1).or_insert(0);
            }

            let mut worst = usize::MAX;
            let mut worst_char = ' ';
            for (k, v) in counts {
                if v < worst {
                    worst = v;
                    worst_char = k;
                }
            }
            corrected[i] = worst_char;
        }

        return Box::new(String::from_iter(corrected));
    }
}

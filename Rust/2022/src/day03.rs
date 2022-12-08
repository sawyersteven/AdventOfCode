use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day03 {
    pub input: Vec<Vec<u8>>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: Vec::new() };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.trim().bytes().collect()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total: usize = 0;
        for line in &self.input {
            let mid = line.len() / 2;
            let back_half: HashSet<&u8> = HashSet::from_iter(line[mid..].iter());
            for c in &line[0..mid] {
                if back_half.contains(c) {
                    total += priority_val(c) as usize;
                    break;
                }
            }
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total: usize = 0;
        for i in (0..self.input.len()).step_by(3) {
            let mut set: HashSet<&u8> = HashSet::from_iter(self.input[i].iter());
            set.retain(|x| self.input[i + 1].contains(x) && self.input[i + 2].contains(x));
            if set.len() != 1 {
                println!("{:?}", set);
                panic!();
            }
            total += priority_val(set.iter().nth(0).unwrap()) as usize;
        }

        return Box::new(total);
    }
}

fn priority_val(c: &u8) -> u8 {
    if c >= &b'a' {
        return *c - b'a' + 1;
    } else {
        return *c - b'A' + 27;
    }
}

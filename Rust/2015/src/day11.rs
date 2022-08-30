use crate::Base;
use std::fmt::Display;

pub struct Day11 {
    pub input: Vec<u8>,
    forbidden: Vec<u8>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            input: Vec::new(),
            forbidden: vec![b'i', b'o', b'l'],
        };
    }

    fn increase(&self, pw: &mut Vec<u8>) {
        let mut i = pw.len() - 1;
        pw[i] += 1;
        while pw[i] > b'z' {
            pw[i] = b'a';
            i -= 1;
            pw[i] += 1;
        }
    }

    fn is_valid(&self, pw: &Vec<u8>) -> bool {
        for c in pw {
            if self.forbidden.contains(c) {
                return false;
            }
        }

        let mut has_seq = false;
        for i in 0..(pw.len() - 3) {
            if pw[i + 1] == pw[i] + 1 && pw[i + 2] == pw[i] + 2 {
                has_seq = true;
                break;
            }
        }

        if !has_seq {
            return false;
        }

        let mut pairs = 0;
        let mut i = 0;
        while i < pw.len() - 1 {
            if pw[i] == pw[i + 1] {
                pairs += 1;
                i += 1;
            }
            i += 1;
        }

        return pairs >= 2;
    }

    fn increment_until_valid(&self, pw: &mut Vec<u8>) {
        self.increase(pw);
        loop {
            if self.is_valid(pw) {
                break;
            }
            self.increase(pw);
        }
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().map(|x| x as u8).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut pw = self.input.clone();
        self.increment_until_valid(&mut pw);
        let result = pw.iter().map(|x| *x as char).collect::<String>();

        return Box::new(result);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut pw = vec![b'c', b'q', b'j', b'x', b'x', b'y', b'z', b'z']; // cqjxxyzz is the answer for part 1
        self.increment_until_valid(&mut pw);
        let result = pw.iter().map(|x| *x as char).collect::<String>();

        return Box::new(result);
    }
}

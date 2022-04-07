use crate::Base;
use std::fmt::Display;

pub struct Day10 {
    pub input: Vec<char>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { input: Vec::new() };
    }

    fn see_say(seq: Vec<char>) -> Vec<char> {
        let mut next = Vec::new();

        let mut i = 0;
        while i < seq.len() {
            let mut char_count = 1;
            while i < seq.len() - 1 && seq[i + 1] == seq[i] {
                char_count += 1;
                i += 1;
            }
            next.push(char::from_digit(char_count, 10).unwrap());
            next.push(seq[i]);
            i += 1;
        }

        return next;
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut seq = self.input.clone();
        for _ in 0..40 {
            seq = Day10::see_say(seq);
        }
        return Box::new(seq.len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut seq = self.input.clone();
        for _ in 0..50 {
            seq = Day10::see_say(seq);
        }
        return Box::new(seq.len());
    }
}

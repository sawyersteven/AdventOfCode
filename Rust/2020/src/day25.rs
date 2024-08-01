use crate::Base;
use std::fmt::Display;

pub struct Day25 {
    card_key: usize,
    door_key: usize,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 {
            card_key: 0,
            door_key: 0,
        };
    }

    fn loop_size(&self, pubkey: usize) -> usize {
        let mut val = 1;
        for loopnum in 1..usize::MAX {
            val = (val * 7) % 20201227;
            if val == pubkey {
                return loopnum;
            }
        }
        unreachable!();
    }

    fn transform(&self, subject: usize, loopsize: usize) -> usize {
        let mut val = 1;
        for _ in 0..loopsize {
            val = (val * subject) % 20201227;
        }
        return val;
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        let mut lines = raw_input.lines();
        self.card_key = lines.next().unwrap().parse().unwrap();
        self.door_key = lines.next().unwrap().parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let ls = self.loop_size(self.card_key);

        return Box::new(self.transform(self.door_key, ls));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("🌟");
    }
}

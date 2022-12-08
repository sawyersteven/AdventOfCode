use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day06 {
    pub input: Vec<u8>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { input: Vec::new() };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.trim().bytes().collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(find_tag(&self.input, 4));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(find_tag(&self.input, 14));
    }
}

fn find_tag(message: &Vec<u8>, tag_len: usize) -> usize {
    for i in 0..message.len() {
        if HashSet::<&u8>::from_iter(message[i..(i + tag_len)].iter()).len() == tag_len {
            return i + tag_len;
        }
    }
    return usize::MAX;
}

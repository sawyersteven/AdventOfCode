use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day05 {
    pub input: Vec<String>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 { input: Vec::new() };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let poly = reduce(&self.input[0]);
        return Box::new(poly.len());
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut shortest = usize::MAX;

        for i in 65..91 {
            let stripped = strip_letter(&self.input[0], i as u8 as char);
            let len = reduce(&stripped).len();
            shortest = shortest.min(len);
        }

        return Box::new(shortest);
    }
}

fn reduce(poly: &String) -> String {
    let mut keep = VecDeque::new();
    for c in poly.bytes() {
        if keep.len() == 0 {
            keep.push_back(c);
        } else if c - 32 == *keep.back().unwrap() || c + 32 == *keep.back().unwrap() {
            keep.pop_back();
        } else {
            keep.push_back(c);
        }
    }
    let mut output = String::new();
    for c in keep {
        output.push(c as char);
    }
    return output;
}

fn strip_letter(s: &String, c: char) -> String {
    return s.chars().filter(|x| *x != c && *x != c.to_ascii_lowercase()).collect();
}

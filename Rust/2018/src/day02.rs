use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day02 {
    pub input: Vec<String>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input.push(line.to_string());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut twos = 0;
        let mut threes = 0;
        for line in &self.input {
            let mut d = HashMap::new();
            for c in line.chars() {
                if !d.contains_key(&c) {
                    d.insert(c, 0);
                }
                d.insert(c, d[&c] + 1);
            }
            if d.values().any(|x| *x == 2) {
                twos += 1;
            }
            if d.values().any(|x| *x == 3) {
                threes += 1;
            }
        }
        return Box::new(twos * threes);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for a in 0..(self.input.len() - 1) {
            for b in (a + 1)..self.input.len() {
                if compare(&self.input[a], &self.input[b]) {
                    return Box::new(filter(&self.input[a], &self.input[b]));
                }
            }
        }
        return Box::new("-");
    }
}

// returns true if a == b or a b begins with a
fn compare(a: &String, b: &String) -> bool {
    let mut diff = false;
    for i in 0..a.len() {
        if a.as_bytes()[i] != b.as_bytes()[i] {
            if diff {
                return false;
            }
            diff = true;
        }
    }
    return diff;
}

fn filter(a: &String, b: &String) -> String {
    let mut s = String::new();
    for i in 0..a.len() {
        if a.as_bytes()[i] == b.as_bytes()[i] {
            s.push(a.as_bytes()[i] as char);
        }
    }
    return s;
}

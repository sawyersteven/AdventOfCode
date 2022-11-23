use itertools::Itertools;

use crate::Base;
use std::fmt::Display;

pub struct Day04 {
    pub input: Vec<String>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { input: Vec::new() };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let words: Vec<&str> = line.split(' ').collect();
            let mut is_valid = true;
            for i in 0..(words.len() - 1) {
                for j in (i + 1)..words.len() {
                    if words[i] == words[j] {
                        is_valid = false;
                        break;
                    }
                }
                if !is_valid {
                    break;
                }
            }
            if is_valid {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let words: Vec<&str> = line.split(' ').collect();
            let mut is_valid = true;
            for i in 0..(words.len() - 1) {
                for j in (i + 1)..words.len() {
                    if is_anagram(words[i], words[j]) {
                        is_valid = false;
                        break;
                    }
                }
                if !is_valid {
                    break;
                }
            }
            if is_valid {
                count += 1;
            }
        }
        return Box::new(count);
    }
}

fn is_anagram(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let ca: Vec<char> = a.chars().sorted().collect();
    let cb: Vec<char> = b.chars().sorted().collect();

    for i in 0..ca.len() {
        if ca[i] != cb[i] {
            return false;
        }
    }
    return true;
}

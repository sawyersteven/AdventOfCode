use crate::Base;
use std::fmt::Display;

pub struct Day04 {
    parts: Vec<Vec<u8>>,
    p1_matches: Vec<Vec<u8>>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 {
            parts: Vec::new(),
            p1_matches: Vec::new(),
        };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        let bounds: Vec<usize> = raw_input.split('-').map(|x| x.parse().unwrap()).collect();
        for part in bounds[0]..=bounds[1] {
            let s = format!("{}", part);
            self.parts.push(s.into_bytes());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;

        for part in &self.parts {
            if test_doubles(&part) && test_ascending(&part) {
                self.p1_matches.push(part.clone());
                count += 1;
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for part in &self.p1_matches {
            if p2_test_doubles(&part) {
                count += 1;
            }
        }

        return Box::new(count);
    }
}

// Returns true if two adjacent u8s are the same
fn test_doubles(word: &Vec<u8>) -> bool {
    for i in 0..(word.len() - 1) {
        if word[i] == word[i + 1] {
            return true;
        }
    }
    return false;
}

fn p2_test_doubles(word: &Vec<u8>) -> bool {
    let mut group_start = 0;
    let mut group_len = 1;
    while (group_start + group_len) < word.len() {
        let a = word[group_start];
        let b = word[group_start + group_len];
        if a == b {
            group_len += 1;
            continue;
        }
        if group_len == 2 {
            return true;
        }
        group_start += group_len;
        group_len = 1;
    }

    return group_len == 2;
}

// Returns true if all u8s are in ascending numerical order
fn test_ascending(word: &Vec<u8>) -> bool {
    for i in 0..(word.len() - 1) {
        if word[i] > word[i + 1] {
            return false;
        }
    }
    return true;
}

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day19 {
    input: String,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 { input: String::new() };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut lines = self.input.lines();
        let towels: Vec<&[u8]> = lines.next().unwrap().split(", ").map(|x| x.as_bytes()).collect();
        lines.next();

        let mut cache = HashMap::new();

        let mut count = 0;
        for pattern in lines {
            let c = solve_p1(&pattern.as_bytes(), &towels, &mut cache);
            if c {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut lines = self.input.lines();
        let towels: Vec<&[u8]> = lines.next().unwrap().split(", ").map(|x| x.as_bytes()).collect();
        lines.next();

        let mut cache = HashMap::new();

        let mut count = 0;
        for pattern in lines {
            count += solve_p2(&pattern.as_bytes(), &towels, &mut cache);
        }

        return Box::new(count); // 919219286602165
    }
}

// Almost identical to p1, just count matches instead of flagging them in the cache
fn solve_p2<'a>(pattern: &'a [u8], towels: &Vec<&[u8]>, cache: &mut HashMap<&'a [u8], usize>) -> usize {
    if let Some(c) = cache.get(pattern) {
        return *c;
    }

    let mut count = 0;
    for towel in towels {
        if pattern.starts_with(towel) {
            if pattern.len() == towel.len() {
                count += 1;
            } else {
                count += solve_p2(&pattern[towel.len()..], &towels, cache);
            }
        }
    }

    *cache.entry(pattern).or_insert(0) += count;
    return count;
}

fn solve_p1<'a>(pattern: &'a [u8], towels: &Vec<&[u8]>, cache: &mut HashMap<&'a [u8], bool>) -> bool {
    if let Some(c) = cache.get(pattern) {
        return *c;
    }

    for towel in towels {
        if pattern.starts_with(towel) {
            if pattern.len() == towel.len() || solve_p1(&pattern[towel.len()..], &towels, cache) {
                cache.insert(pattern, true);
                return true;
            }
        }
    }

    cache.insert(pattern, false);
    return false;
}

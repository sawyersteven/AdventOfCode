use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day22 {
    input: String,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 { input: String::new() };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0isize;
        for ln in self.input.lines() {
            let mut n: isize = ln.parse().unwrap();
            for _ in 0..2000 {
                n = get_next(n);
            }
            sum += n;
        }
        return Box::new(sum);
    }

    // This is more or less just brute forcing
    fn part2(&mut self) -> Box<dyn Display> {
        let mut totals = HashMap::new();

        for ln in self.input.lines() {
            let mut n: isize = ln.parse().unwrap();
            let mut sequences = HashMap::new();
            let mut seq = [0isize; 4];
            for _ in 0..3 {
                let prev = n % 10;
                n = get_next(n);
                seq = [seq[1], seq[2], seq[3], (n % 10) - prev];
            }
            for _ in 3..2000 {
                let prev = n % 10;
                n = get_next(n);
                seq = [seq[1], seq[2], seq[3], (n % 10) - prev];
                sequences.entry(seq).or_insert(n % 10);
            }
            for (sequence, num) in sequences {
                *totals.entry(sequence).or_insert(0) += num;
            }
        }

        let bananas = *totals.values().max().unwrap();
        return Box::new(bananas);
    }
}

#[inline(always)]
fn get_next(mut num: isize) -> isize {
    num ^= num << 6; // n*64
    num %= 16777216;
    num ^= num >> 5; // n/32

    // num &= 0xFFFFFF; this ultimately has no effect

    num ^= num << 11; // n*2048
    num %= 16777216;
    return num;
}

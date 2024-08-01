use crate::Base;
use std::{fmt::Display, ops::RangeInclusive};

struct PW {
    range: RangeInclusive<usize>,
    req_char: u8,
    string: String,
}

pub struct Day02 {
    passwords: Vec<PW>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { passwords: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut parts = line.split(|x| [' ', '-', ':'].contains(&x));
            let rs = parts.next().unwrap().parse().unwrap();
            let re = parts.next().unwrap().parse().unwrap();
            let req_char = parts.next().unwrap().as_bytes()[0];
            _ = parts.next();

            self.passwords.push(PW {
                range: rs..=re,
                req_char: req_char,
                string: parts.next().unwrap().to_string(),
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut good_pws = 0;
        for pw in &self.passwords {
            let mut count = 0;
            for b in pw.string.as_bytes() {
                if *b == pw.req_char {
                    count += 1;
                }
            }
            if pw.range.contains(&count) {
                good_pws += 1;
            }
        }
        return Box::new(good_pws);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut good_pws = 0;
        for pw in &self.passwords {
            let a = pw.string.as_bytes()[*pw.range.start() - 1] == pw.req_char;
            let b = pw.string.as_bytes()[*pw.range.end() - 1] == pw.req_char;
            if a ^ b {
                good_pws += 1;
            }
        }
        return Box::new(good_pws);
    }
}

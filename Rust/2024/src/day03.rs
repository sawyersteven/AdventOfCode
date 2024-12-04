use crate::Base;
use std::{fmt::Display, num::ParseIntError};

pub struct Day03 {
    input: String,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: String::new() };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut t = 0;

        let mut substr = &self.input[0..];
        while substr.len() > 8 {
            // min chars required for mul(x,y)
            if substr.starts_with("mul(") {
                substr = &substr[4..];
                t += match substr
                    .split_once(')')
                    .and_then(|(a, _)| a.split_once(',').and_then(|(l, r)| try_mult(l, r).ok()))
                {
                    Some(n) => n,
                    None => 0,
                };
                continue;
            }
            substr = &substr[1..];
        }
        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        let mut can_do = true;
        let mut substr = &self.input[0..];
        while substr.len() > 8 {
            if can_do && substr.starts_with("mul(") {
                substr = &substr[4..];
                t += match substr
                    .split_once(')')
                    .and_then(|(a, _)| a.split_once(',').and_then(|(l, r)| try_mult(l, r).ok()))
                {
                    Some(n) => n,
                    None => 0,
                };
                continue;
            } else if substr.starts_with("don't(") {
                substr = &substr[6..];
                can_do = false;
                continue;
            } else if substr.starts_with("do(") {
                substr = &substr[3..];
                can_do = true;
                continue;
            }
            substr = &substr[1..];
        }
        return Box::new(t);
    }
}

fn try_mult(a: &str, b: &str) -> Result<isize, ParseIntError> {
    a.parse()
        .and_then(|ai: isize| b.parse().and_then(|bi: isize| Ok(bi * ai)))
}

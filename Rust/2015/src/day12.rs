use crate::Base;
use std::{fmt::Display, ops::Add};

pub struct Day12 {
    pub input: String,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            input: String::from(""),
        };
    }

    fn count_nums(input: &String) -> isize {
        let mut total: isize = 0;
        let parts = input
            .split([':', ',', '}', '[', ']'])
            .collect::<Vec<&str>>();
        for part in parts {
            match part.parse::<isize>() {
                Ok(d) => total += d,
                Err(_) => {}
            }
        }
        return total;
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(Day12::count_nums(&self.input));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut remove_ranges: Vec<(usize, usize)> = Vec::new();

        let chars: Vec<char> = self.input.chars().collect();
        let mut i = self.input.len();
        while i > 1 {
            i -= 1;
            if chars[i] != ':' {
                continue;
            }

            if chars.get(i..i + 6).unwrap() != [':', '"', 'r', 'e', 'd', '"'] {
                continue;
            }

            let mut start = i;
            let mut end = i;

            let mut depth = 1;
            while start > 0 {
                start -= 1;
                depth += match chars[start] {
                    '}' => 1,
                    '{' => -1,
                    _ => 0,
                };
                if depth == 0 {
                    break;
                }
            }

            depth = 1;
            while end < chars.len() {
                end += 1;
                depth += match chars[end] {
                    '}' => -1,
                    '{' => 1,
                    _ => 0,
                };
                if depth == 0 {
                    break;
                }
            }
            remove_ranges.push((start, end));
            i = start;
        }

        remove_ranges.reverse();

        let mut i = 0;
        while i < remove_ranges.len() {
            let mut j = i;
            while j < remove_ranges.len() {
                if remove_ranges[j].1 < remove_ranges[i].1 {
                    remove_ranges.remove(j);
                    j -= 1;
                }
                j += 1;
            }
            i += 1;
        }

        let mut combined = String::new();
        let mut ind = 0;
        for (start, end) in remove_ranges {
            combined = combined.add(self.input.get(ind..start).unwrap());
            ind = end + 1;
        }
        combined = combined.add(self.input.get(ind..).unwrap());

        return Box::new(Day12::count_nums(&combined));
    }
}

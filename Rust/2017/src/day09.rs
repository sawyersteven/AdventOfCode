use crate::Base;
use std::fmt::Display;

pub struct Day09 {
    pub input: String,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 {
            input: String::from(""),
        };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut score = 0;
        let mut depth = 0;
        let mut in_garbage = false;

        let chars: Vec<char> = self.input.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];

            if c == '!' {
                i += 2;
                continue;
            }

            if in_garbage {
                in_garbage = c != '>';
                i += 1;
                continue;
            }

            if c == '<' {
                in_garbage = true;
                i += 1;
                continue;
            }

            if c == '{' {
                depth += 1;
                i += 1;
                continue;
            }

            if c == '}' {
                score += depth;
                depth -= 1;
                i += 1;
                continue;
            }

            i += 1;
        }

        return Box::new(score);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut garbage_count = 0;
        let mut in_garbage = false;

        let chars: Vec<char> = self.input.chars().collect();

        let mut i = 0;
        while i < chars.len() {
            let c = chars[i];
            if c == '!' {
                i += 2;
                continue;
            }

            if in_garbage {
                if c == '>' {
                    in_garbage = false;
                } else {
                    garbage_count += 1;
                }
                i += 1;
                continue;
            }
            in_garbage = c == '<';
            i += 1;
        }

        return Box::new(garbage_count);
    }
}

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day01 {
    pub lines: Vec<String>,
    number_words: HashMap<String, u32>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 {
            lines: Vec::new(),
            number_words: HashMap::from([
                ("one".to_string(), 1),
                ("two".to_string(), 2),
                ("three".to_string(), 3),
                ("four".to_string(), 4),
                ("five".to_string(), 5),
                ("six".to_string(), 6),
                ("seven".to_string(), 7),
                ("eight".to_string(), 8),
                ("nine".to_string(), 9),
            ]),
        };
    }

    fn process_line(&self, line: &String) -> u32 {
        let mut num = 0;
        for c in line.chars() {
            if c.is_digit(10) {
                num += c.to_digit(10).unwrap() * 10;
                break;
            }
        }
        for c in line.chars().rev() {
            if c.is_digit(10) {
                num += c.to_digit(10).unwrap();
                break;
            }
        }
        return num;
    }

    /*
    My input has cases where the line can end with something like "eightwo"
    This means I can't just replace words starting at the beginning of the string
    I have to go both forward and backward so "eightwo" becomes "eigh2" instead of "8wo"
    */
    fn process_line_2(&self, line: &String) -> u32 {
        let chars: Vec<char> = line.chars().collect();
        let mut sum = 0;

        // find first digit
        let mut i = 0;
        while i < line.len() {
            let mut found = false;
            if chars[i].is_digit(10) {
                sum += chars[i].to_digit(10).unwrap() * 10;
                break;
            } else {
                for (word, number) in &self.number_words {
                    if line[i..].starts_with(word) {
                        sum += number * 10;
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            i += 1;
        }

        // find last digit by doing the same thing but backward
        // Elegant? No. Effective? Sure.
        i = line.len() - 1;
        loop {
            let mut found = false;
            if chars[i].is_digit(10) {
                sum += chars[i].to_digit(10).unwrap();
                break;
            } else {
                for (word, number) in &self.number_words {
                    if line[i..].starts_with(word) {
                        sum += number;

                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            i -= 1;
        }
        return sum;
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.lines = raw_input.split("\n").map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sum = 0;

        for line in &self.lines {
            sum += self.process_line(line);
        }

        return Box::new(sum);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut sum = 0;

        for line in &self.lines {
            sum += self.process_line_2(&line);
        }

        return Box::new(sum);
    }
}

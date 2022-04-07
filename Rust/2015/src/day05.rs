use crate::Base;
use std::fmt::Display;

pub struct Day05 {
    pub input: Vec<String>,
    pub vowels: [char; 5], //HashSet<char>,
    pub forbidden: [String; 4],
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            input: Vec::new(),
            vowels: ['a', 'e', 'i', 'o', 'u'],
            forbidden: ["ab", "cd", "pq", "xy"].map(|x| String::from(x)),
        };
    }

    fn forbidden_test(&self, line: &String) -> bool {
        for pair in &self.forbidden {
            if line.contains(pair) {
                return false;
            }
        }
        return true;
    }

    fn vowel_and_double_test(&self, line: &String) -> bool {
        let c: Vec<char> = line.chars().collect();
        let mut vowel_count = 0;
        let mut has_double: bool = false;
        if self.vowels.contains(c.last().unwrap()) {
            vowel_count += 1;
        }
        for i in 0..(c.len() - 1) {
            if self.vowels.contains(&c[i]) {
                vowel_count += 1;
            }
            if c[i] == c[i + 1] {
                has_double = true;
            }
        }

        return vowel_count >= 3 && has_double;
    }

    fn has_pair(&self, chars: &Vec<char>) -> bool {
        for i in 0..(chars.len() - 3) {
            for j in (i + 2)..(chars.len() - 1) {
                if chars[i] == chars[j] && chars[i + 1] == chars[j + 1] {
                    return true;
                }
            }
        }
        return false;
    }

    fn has_aba(&self, chars: &Vec<char>) -> bool {
        for i in 0..(chars.len() - 2) {
            if chars[i] == chars[i + 2] {
                return true;
            }
        }
        return false;
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split("\n").map(|x| String::from(x)).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            if self.forbidden_test(line) && self.vowel_and_double_test(line) {
                count += 1;
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let chars = line.chars().collect();
            if self.has_pair(&chars) && self.has_aba(&chars) {
                count += 1;
            }
        }

        return Box::new(count);
    }
}

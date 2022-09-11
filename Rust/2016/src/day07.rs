use crate::Base;
use std::{collections::HashSet, fmt::Display};

const SEP: [char; 2] = ['[', ']'];

pub struct Day07 {
    pub input: Vec<String>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { input: Vec::new() };
    }

    fn has_abba(chars: &Vec<char>) -> bool {
        for i in 0..chars.len() - 3 {
            if chars[i] == chars[i + 3] && chars[i + 1] == chars[i + 2] && chars[i] != chars[i + 1] {
                return true;
            }
        }
        return false;
    }

    fn find_aba(chars: &Vec<char>, invert: bool) -> HashSet<String> {
        let mut abas = HashSet::new();

        for i in 0..chars.len() - 2 {
            if chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
                let itr = chars[i..i + 2].iter();
                if invert {
                    abas.insert(String::from_iter(itr.rev()));
                } else {
                    abas.insert(String::from_iter(itr));
                }
            }
        }

        return abas;
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut count = 0;

        for line in &self.input {
            let parts: Vec<&str> = line.split(SEP).collect();
            let mut inside = false;
            let mut outside = false;
            for i in 0..parts.len() {
                if i % 2 == 0 {
                    outside |= Self::has_abba(&parts[i].chars().collect());
                } else {
                    inside |= Self::has_abba(&parts[i].chars().collect());
                }
            }
            if outside & !inside {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let parts: Vec<&str> = line.split(SEP).collect();
            let mut abas: Vec<String> = Vec::new();

            for i in (0..parts.len()).step_by(2) {
                for n in Self::find_aba(&parts[i].chars().collect(), false) {
                    abas.push(n);
                }
            }

            let mut found = false;
            for i in (1..parts.len()).step_by(2) {
                for bab in Self::find_aba(&parts[i].chars().collect(), true) {
                    if abas.contains(&bab) {
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }

            if found {
                count += 1;
            }
        }
        return Box::new(count);
    }
}

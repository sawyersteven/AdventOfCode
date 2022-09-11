use crypto::{digest::Digest, md5::Md5};

use crate::Base;
use std::fmt::Display;

const PREFIX: [u8; 5] = [b'0', b'0', b'0', b'0', b'0'];

pub struct Day05 {
    pub input: String,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            input: String::from(""),
        };
    }
}

impl Base for Day05 {
    #[allow(unused)]
    fn parse_input(&mut self, raw_input: String) {
        self.input = String::from("ugkcyxxp");
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut passcode: Vec<char> = Vec::new();
        let mut hasher = Md5::new();

        for i in 0..usize::MAX {
            if passcode.len() == 8 {
                break;
            }

            hasher.input_str(&self.input);
            hasher.input_str(&i.to_string());
            let h = hasher.result_str();
            let hash = h.as_bytes();
            hasher.reset();

            if hash.starts_with(&PREFIX) {
                passcode.push(hash[5] as char);
            }
        }

        return Box::new(format!("{:?}", passcode));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut passcode = ['-', '-', '-', '-', '-', '-', '-', '-'];
        let mut found = 0;
        let mut hasher = Md5::new();

        for idx in 0..usize::MAX {
            hasher.reset();
            hasher.input_str(&self.input);
            hasher.input_str(&idx.to_string());

            let h = hasher.result_str();
            let hash = h.as_bytes();

            if !hash.starts_with(&PREFIX) {
                continue;
            }

            let i = (hash[5] - 48) as usize;

            if i > 7 || passcode[i] != '-' {
                continue;
            }
            passcode[i] = hash[6] as char; // hash.chars().nth(6).unwrap();
            found += 1;
            if found == passcode.len() {
                break;
            }
        }
        return Box::new(format!("{:?}", passcode));
    }
}

extern crate crypto;
use crate::Base;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::fmt::Display;

pub struct Day04 {
    pub input: String,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 {
            input: String::from(""),
        };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let key = self.input.as_bytes();

        let mut hasher = Md5::new();
        let mut res = [0; 16];
        for i in 0..std::usize::MAX {
            hasher.input(key);
            hasher.input(i.to_string().as_bytes());
            hasher.result(&mut res);

            // First 5 chars are first 2.5 bytes
            if res[0] as usize + res[1] as usize + (res[2] >> 4) as usize == 0 {
                return Box::new(i);
            }

            hasher.reset();
        }

        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        let key = self.input.as_bytes();

        let mut hasher = Md5::new();
        let mut res = [0; 16];
        for i in 0..std::usize::MAX {
            hasher.input(key);
            hasher.input(i.to_string().as_bytes());
            hasher.result(&mut res);

            // First 6 chars are first 3 bytes
            if res[0] as usize + res[1] as usize + res[2] as usize == 0 {
                return Box::new(i);
            }

            hasher.reset();
        }

        return Box::new("-");
    }
}

use crypto::{digest::Digest, md5::Md5};

use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day14 {
    pub input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { input: String::new() };
    }

    fn hash(hasher: &mut Md5, input: String, iters: usize) -> String {
        let mut input = input;
        for _ in 0..iters {
            hasher.input_str(&input);
            input = hasher.result_str();
            hasher.reset();
        }
        return input;
    }

    pub fn solve(&self, iters: usize) -> usize {
        let mut hash_q = VecDeque::<String>::with_capacity(1000);

        let mut hasher = Md5::new();
        for i in 0..1000 {
            hash_q.push_back(Self::hash(&mut hasher, format!("{}{}", self.input, i), iters));
        }

        let mut valid_count = 0;
        for i in 1000..usize::MAX {
            let current = hash_q.pop_front().unwrap();

            hash_q.push_back(Self::hash(&mut hasher, format!("{}{}", self.input, i), iters));

            // let quintuple = match find_triple(&current) {
            //     Some(c) => (0..5).map(|_| c).collect::<String>(),
            //     None => continue,
            // };

            let quintuple = match find_triple(&current) {
                b' ' => continue,
                c => (0..5).map(|_| c as char).collect::<String>(),
            };

            for hash in &hash_q {
                match hash.find(&quintuple) {
                    Some(_) => {
                        valid_count += 1;
                        break;
                    }
                    None => {}
                }
            }
            if valid_count == 64 {
                return i - 1000;
            }
        }
        return 0;
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        return Box::new(self.solve(1));
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new(self.solve(2017));
    }
}

fn find_triple(hash: &String) -> u8 {
    //let chars: Vec<char> = hash.chars().collect();
    let chars = hash.as_bytes();
    for i in 0..(chars.len() - 2) {
        if chars[i] == chars[i + 1] && chars[i] == chars[i + 2] {
            return chars[i];
        }
    }
    return b' ';
}

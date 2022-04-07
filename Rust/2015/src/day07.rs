use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day07 {
    pub wire_map: HashMap<String, String>,
    pub found: HashMap<String, isize>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            wire_map: HashMap::new(),
            found: HashMap::new(),
        };
    }

    fn find(&mut self, out_channel: &String) -> isize {
        if self.found.contains_key(out_channel) {
            return self.found[out_channel];
        }

        let cmd = &self.wire_map[out_channel];

        match cmd.parse::<isize>() {
            Ok(n) => {
                self.found.insert(out_channel.to_owned(), n);
                return n;
            }
            Err(_) => {}
        }

        let val: isize;
        let parts: Vec<String> = cmd.split(" ").map(|x| String::from(x)).collect();
        if parts.len() == 1 {
            val = self.find(&parts[0]);
        } else if parts.len() == 2 {
            val = !self.find(&parts[1]);
        } else {
            let a: isize;
            let b: isize;

            match parts[0].parse::<isize>() {
                Ok(n) => {
                    a = n;
                }
                Err(_) => {
                    a = self.find(&parts[0]);
                }
            }

            match parts[2].parse::<isize>() {
                Ok(n) => {
                    b = n;
                }
                Err(_) => {
                    b = self.find(&parts[2]);
                }
            }

            val = match parts[1].chars().next().unwrap() {
                'O' => a | b,
                'A' => a & b,
                'R' => a >> b,
                'L' => a << b,
                _ => panic!(),
            };
        }
        self.found.insert(out_channel.to_owned(), val);
        return val;
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split("\n") {
            let parts: Vec<&str> = line.split(" -> ").collect();
            self.wire_map
                .insert(String::from(parts[1]), String::from(parts[0]));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.find(&String::from("a")));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        self.found.clear();
        self.found.insert("b".to_string(), 3176); // 3176 is answer from part 1
        return Box::new(self.find(&String::from("a")));
    }
}

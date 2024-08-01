use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day19 {
    rules: Vec<String>,
    messages: Vec<String>,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            rules: Vec::new(),
            messages: Vec::new(),
        };
    }

    fn fragment_matches_any(&self, fragment: &str, messages: &Vec<String>) -> bool {
        let l = fragment.len();
        return messages.iter().any(|m| m.len() >= l && m[0..l] == *fragment);
    }

    /*
    This could be quicker if I used Vec<usize> in place of strings, but its
    kind of a hassle and this finishes both parts in ~4 seconds and that's good
    enough for now.
     */
    fn find_matches(&self, rule_num: usize) -> usize {
        let mut messages = self.messages.clone();
        let rules = self.rules.clone();
        let max_message_length = self.messages.iter().map(|x| x.len()).max().unwrap();

        let mut possibilities = VecDeque::new();
        possibilities.push_back(rules[rule_num].clone());

        let mut matches = 0;
        while let Some(mut code) = possibilities.pop_front() {
            let mut check_messages = true;

            let code_parts: Vec<&str> = code.split(' ').collect();

            for i in 0..(code_parts.len().min(max_message_length)) {
                if code_parts[i] == "a" || code_parts[i] == "b" {
                    continue;
                }

                if i > 0 && !self.fragment_matches_any(&code.replace(" ", "")[0..i], &messages) {
                    check_messages = false;
                    break;
                };

                check_messages = false;
                let mut split_code: Vec<&str> = code.split(" ").collect();

                for opt in rules[code_parts[i].parse::<usize>().unwrap()].split(" | ") {
                    split_code[i] = opt;

                    possibilities.push_back(split_code.join(" "));
                }
                break;
            }

            if check_messages {
                code = code.replace(" ", "");
                let mut i = 0;
                while i < messages.len() {
                    if messages[i] == code {
                        matches += 1;
                        messages.remove(i);
                        continue;
                    }
                    i += 1;
                }
            }
        }

        return matches;
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        const MT_STR: String = String::new();
        self.rules = vec![MT_STR; 129];
        for line in raw_input.split("\n\n").nth(0).unwrap().lines() {
            let mut id_sbrls = line.split(": ");
            let id = id_sbrls.next().unwrap().parse::<usize>().unwrap();
            let subrules = id_sbrls.next().unwrap().replace("\"", "");

            self.rules[id] = subrules;
        }

        for r in raw_input.split("\n\n").nth(1).unwrap().lines() {
            self.messages.push(r.to_string());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let matches = self.find_matches(0);
        return Box::new(matches);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        self.rules[8] = "42 | 42 8".to_string();
        self.rules[11] = "42 31 | 42 11 31".to_string();
        let matches = self.find_matches(0);
        return Box::new(matches);
    }
}

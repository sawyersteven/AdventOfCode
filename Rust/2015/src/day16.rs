use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day16 {
    pub sues: Vec<HashMap<String, usize>>,
    pub report: HashMap<String, usize>,
}

impl Day16 {
    pub fn new() -> Day16 {
        let mut day = Day16 {
            sues: Vec::new(),
            report: HashMap::new(),
        };
        day.report.insert("children".to_string(), 3);
        day.report.insert("cats".to_string(), 7);
        day.report.insert("samoyeds".to_string(), 2);
        day.report.insert("pomeranians".to_string(), 3);
        day.report.insert("akitas".to_string(), 0);
        day.report.insert("vizslas".to_string(), 0);
        day.report.insert("goldfish".to_string(), 5);
        day.report.insert("trees".to_string(), 3);
        day.report.insert("cars".to_string(), 2);
        day.report.insert("perfumes".to_string(), 1);

        return day;
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let mut sue = HashMap::<String, usize>::new();
            let parts: Vec<&str> = line
                .split(|c| c == ',' || c == ':' || c == ' ')
                .filter(|p| !p.is_empty())
                .collect();

            for i in (2..parts.len()).step_by(2) {
                sue.insert(parts[i].to_string(), parts[i + 1].parse().unwrap());
            }
            self.sues.push(sue);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        for (i, sue) in self.sues.iter().enumerate() {
            let mut ok = true;
            for (k, v) in sue {
                if &self.report[k] != v {
                    ok = false;
                    break;
                }
            }
            if ok {
                return Box::new(i + 1);
            }
        }

        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for (i, sue) in self.sues.iter().enumerate() {
            let mut ok = true;
            for (k, v) in sue {
                if k == "trees" || k == "cats" {
                    if self.report[k] > *v {
                        ok = false;
                    }
                } else if k == "pomeranians" || k == "goldfish" {
                    if self.report[k] < *v {
                        ok = false;
                    }
                } else {
                    if self.report[k] != *v {
                        ok = false;
                    }
                }
                if !ok {
                    break;
                }
            }
            if ok {
                return Box::new(i + 1);
            }
        }
        return Box::new("-");
    }
}

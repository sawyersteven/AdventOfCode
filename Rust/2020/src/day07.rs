use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day07 {
    bag_types: HashMap<String, Vec<(String, usize)>>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            bag_types: HashMap::new(),
        };
    }

    fn parent_bags(&self, child_bag: &str) -> HashSet<String> {
        let mut parents = HashSet::new();
        for (key, value) in &self.bag_types {
            for v in value {
                if v.0 == child_bag {
                    parents.extend(self.parent_bags(&key));
                    parents.insert(key.clone());
                    break;
                }
            }
        }
        return parents;
    }

    fn count_children(&self, bag_name: &str) -> usize {
        let mut count = 0;
        for children in &self.bag_types[bag_name] {
            count += children.1;
            count += self.count_children(&children.0) * children.1;
        }
        return count;
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut parts = line.splitn(2, " bags");
            let parent = parts.next().unwrap();
            let p1 = &parts.next().unwrap()[8..];

            if p1 == " no other bags." {
                self.bag_types.insert(parent.to_string(), Vec::new());
                continue;
            }

            let mut children = Vec::new();
            for p in p1.split(',') {
                let a = p[3..].split(" bag").nth(0).unwrap().to_string();
                let b = p[0..3].trim().parse().unwrap();
                children.push((a, b));
            }
            self.bag_types.insert(parent.to_string(), children);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.parent_bags("shiny gold").len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.count_children("shiny gold"));
    }
}

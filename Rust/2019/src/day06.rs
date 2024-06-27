use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day06 {
    orbits: HashMap<String, Vec<String>>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { orbits: HashMap::new() };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let bodies: Vec<&str> = line.splitn(2, ')').collect();
            if !self.orbits.contains_key(&*bodies[0]) {
                self.orbits.insert(bodies[0].to_string(), Vec::new());
            }
            self.orbits
                .get_mut(&bodies[0].to_string())
                .unwrap()
                .push(bodies[1].to_string());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let ans = self.count_orbits(&"COM".to_string(), 0);
        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let branches = self.find_branches(&"COM".to_string());

        let branch_a: Vec<&str> = branches[0].split(',').rev().collect();
        let branch_b: Vec<&str> = branches[1].split(',').rev().collect();

        for i in 0..branch_a.len() {
            match branch_b.iter().position(|x| *x == branch_a[i]) {
                Some(ind_b) => return Box::new(i + ind_b - 2),
                None => {}
            }
        }

        return Box::new("-");
    }
}

impl Day06 {
    fn count_orbits(&self, hub_body: &String, depth: usize) -> usize {
        if self.orbits.contains_key(hub_body) {
            let mut count = depth;
            for child in &self.orbits[hub_body] {
                count += self.count_orbits(child, depth + 1);
            }
            return count;
        } else {
            return depth;
        }
    }

    fn find_branches(&self, hub_body: &String) -> Vec<String> {
        let mut branches = Vec::new();

        if !self.orbits.contains_key(hub_body) {
            if hub_body == "YOU" || hub_body == "SAN" {
                branches.push(hub_body.clone());
            }
            return branches;
        }

        for child in &self.orbits[hub_body] {
            for c in self.find_branches(&child) {
                branches.push(format!("{},{}", hub_body, c));
            }
        }

        return branches;
    }
}

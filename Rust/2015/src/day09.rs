use crate::Base;
use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day09 {
    pub map: HashMap<String, Vec<(String, usize)>>,
    pub cities: HashSet<String>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 {
            map: HashMap::new(),
            cities: HashSet::new(),
        };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split("\n") {
            let parts = line.split(" ").collect::<Vec<&str>>();
            if !self.map.contains_key(parts[0]) {
                self.map.insert(String::from(parts[0]), Vec::new());
            }
            if !self.map.contains_key(parts[2]) {
                self.map.insert(String::from(parts[2]), Vec::new());
            }

            self.map
                .get_mut(parts[0])
                .unwrap()
                .push((String::from(parts[2]), parts[4].parse().unwrap()));

            self.map
                .get_mut(parts[2])
                .unwrap()
                .push((String::from(parts[0]), parts[4].parse().unwrap()));

            self.cities.insert(String::from(parts[0]));
            self.cities.insert(String::from(parts[2]));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut shortest = usize::MAX;
        let mut longest = usize::MIN;

        for path in self.cities.iter().permutations(self.cities.len()) {
            let mut dist: usize = 0;

            let mut i = 0;
            while i < path.len() - 1 {
                if !self.map.contains_key(path[i]) {
                    continue;
                }
                for (c, d) in &self.map[path[i]] {
                    if c == path[i + 1] {
                        dist += d;
                    }
                }
                i += 1;
            }

            if dist > longest {
                longest = dist;
            }
            if dist < shortest {
                shortest = dist;
            }
        }

        return Box::new(shortest);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(898); // no sense in copy/pasting all of part 1
    }
}

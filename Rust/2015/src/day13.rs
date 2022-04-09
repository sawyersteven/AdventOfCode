use itertools::Itertools;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day13 {
    people: HashSet<String>,
    map: HashMap<String, HashMap<String, isize>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 {
            people: HashSet::new(),
            map: HashMap::new(),
        };
    }

    fn find_best(&self) -> isize {
        let mut best: isize = 0;
        for arrangement in self.people.iter().permutations(self.people.len()) {
            let mut total: isize = 0;
            let l = arrangement.len();

            // adds one of indexes 0 and 1, -1 and -2, and two of all others
            for i in 0..arrangement.len() - 2 {
                total += self.map[arrangement[i + 1]][arrangement[i]];
                total += self.map[arrangement[i + 1]][arrangement[i + 2]];
            }

            total += self.map[arrangement[0]][arrangement[1]];
            total += self.map[arrangement[0]][arrangement[l - 1]];

            total += self.map[arrangement[l - 1]][arrangement[0]];
            total += self.map[arrangement[l - 1]][arrangement[l - 2]];

            best = std::cmp::max(best, total);
        }

        return best;
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts = line
                .split(' ')
                .map(|x| x.to_string())
                .collect::<Vec<String>>();

            let val: isize = parts[3].parse::<isize>().unwrap()
                * (if parts[2].starts_with('g') { 1 } else { -1 });

            let key = parts[parts.len() - 1].trim_end_matches('.').to_string();
            // This is apparently the onyl reasonable way to assign a k:v to a nested hashmap...
            self.map
                .entry(parts[0].to_owned())
                .or_default()
                .insert(key, val);

            self.people.insert(parts[0].clone());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.find_best());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for person in &self.people {
            self.map
                .entry("me".to_string())
                .or_default()
                .insert(person.to_owned(), 0);

            self.map
                .entry(person.to_owned())
                .or_default()
                .insert("me".to_string(), 0);
        }

        self.people.insert("me".to_string());
        return Box::new(self.find_best());
    }
}

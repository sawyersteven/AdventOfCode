use itertools::Itertools;

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day04 {
    pub naps: HashMap<usize, [usize; 60]>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { naps: HashMap::new() };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<String> = raw_input.split('\n').sorted().map(|x| x.to_string()).collect();

        let mut i = 0;
        while i < lines.len() {
            let line = &lines[i];

            // parse ID line
            let id = line
                .split('#')
                .nth(1)
                .unwrap()
                .split(' ')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();
            if !self.naps.contains_key(&id) {
                self.naps.insert(id.clone(), [0; 60]);
            }
            i += 1;

            // parse nap lines
            while i < lines.len() && !lines[i].contains('#') {
                let start: usize = lines[i].split(':').nth(1).unwrap()[0..2].parse().unwrap();
                i += 1;
                let end: usize = lines[i].split(':').nth(1).unwrap()[0..2].parse().unwrap();
                i += 1;

                for j in start..end {
                    self.naps.get_mut(&id).unwrap()[j] += 1;
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sleepy_guard: usize = 0;
        let mut mins_asleep = 0;

        for (id, naps) in &self.naps {
            let sum = naps.iter().sum();
            if sum > mins_asleep {
                sleepy_guard = *id;
                mins_asleep = sum;
            }
        }

        let mut best_min = 0;
        let mut min_val = 0;
        for i in 0..self.naps[&sleepy_guard].len() {
            if self.naps[&sleepy_guard][i] > min_val {
                min_val = self.naps[&sleepy_guard][i];
                best_min = i;
            }
        }

        return Box::new(sleepy_guard * best_min);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut gid = 0;
        let mut best_min = 0;
        let mut min_val = 0;

        for (id, naps) in &self.naps {
            for i in 0..naps.len() {
                if naps[i] > min_val {
                    min_val = naps[i];
                    best_min = i;
                    gid = *id;
                }
            }
        }

        return Box::new(gid * best_min);
    }
}

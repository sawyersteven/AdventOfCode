use crate::Base;
use std::{collections::HashMap, fmt::Display};

const CRANGE: usize = 3;

pub struct Day10 {
    nums: Vec<usize>,
    connections: Vec<usize>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 {
            nums: Vec::new(),
            connections: Vec::new(),
        };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.nums = raw_input.lines().map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.nums.sort();

        self.connections = vec![0; self.nums.len() + 2];
        for i in 0..self.nums.len() {
            self.connections[i + 1] = self.nums[i];
        }
        let l = self.connections.len() - 1;
        self.connections[l] = self.nums.last().unwrap() + 3;

        let mut current = self.connections[0];
        let mut ones = 0;
        let mut threes = 0;
        for i in 0..self.connections.len() {
            if self.connections[i] - current == 1 {
                ones += 1;
            }
            if self.connections[i] - current == 3 {
                threes += 1;
            }
            current = self.connections[i];
        }
        return Box::new(threes * ones);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut cache: HashMap<usize, usize> = HashMap::new();
        for c in &self.connections {
            cache.insert(*c, 0);
        }
        cache.insert(0, 1);

        for i in 0..self.connections.len() {
            let mut j = i + 1;
            while j < self.connections.len() && self.connections[j] - self.connections[i] <= CRANGE {
                let v = cache[&self.connections[j]] + cache[&self.connections[i]];
                cache.insert(self.connections[j], v);
                j += 1;
            }
        }

        return Box::new(cache[self.connections.iter().nth_back(1).unwrap()]);
    }
}

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day17 {
    containers: Vec<usize>,
    size_counts: HashMap<usize, usize>,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 {
            containers: Vec::new(),
            size_counts: HashMap::new(),
        };
    }

    fn count_combinations(&mut self, numbers: Vec<usize>, target_sum: usize, partial: Vec<usize>) -> usize {
        let sum = sum(&partial);
        if sum == target_sum {
            let count = partial.len();
            let mapped_val = self.size_counts.get(&count).map_or(1, |x| x + 1);
            self.size_counts.insert(count, mapped_val);
            return 1;
        }

        if sum > target_sum {
            return 0;
        }

        let mut t = 0;
        for i in 0..numbers.len() {
            let remaining = crate::utils::copy_range(&numbers, i + 1, numbers.len() - (i + 1));
            let mut p = partial.clone();
            p.push(numbers[i]);
            t += self.count_combinations(remaining, target_sum, p);
        }

        return t;
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.containers = raw_input.split('\n').map(|l| l.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut c = self.containers.clone();
        c.sort();

        return Box::new(Self::count_combinations(self, c, 150, Vec::new()));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut smallest: usize = usize::MAX;
        for k in self.size_counts.keys() {
            if k < &smallest {
                smallest = *k;
            }
        }
        return Box::new(self.size_counts[&smallest]);
    }
}

fn sum(vec: &Vec<usize>) -> usize {
    let mut s = 0;
    for u in vec {
        s += u;
    }
    return s;
}

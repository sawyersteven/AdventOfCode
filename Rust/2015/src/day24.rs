use itertools::Itertools;

use crate::Base;
use std::fmt::Display;

pub struct Day24 {
    pub input: Vec<usize>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 { input: Vec::new() };
    }

    fn find_best_combo(&self, target_weight: usize) -> usize {
        let mut lowest = usize::MAX;
        let mut found = false;
        for perm_len in 2..self.input.len() {
            for perm in self.input.iter().combinations(perm_len) {
                if sum(&perm) == target_weight {
                    found = true;
                    let prod = perm.iter().map(|x| *x).product();
                    if prod < lowest {
                        lowest = prod;
                    }
                }
            }
            if found {
                break;
            }
        }

        return lowest;
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let target_weight = self.input.iter().sum::<usize>() / 3;

        let answer = Self::find_best_combo(&self, target_weight);

        return Box::new(answer);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let target_weight = self.input.iter().sum::<usize>() / 4;

        let answer = Self::find_best_combo(&self, target_weight);

        return Box::new(answer);
    }
}

fn sum(vec: &Vec<&usize>) -> usize {
    let mut s = 0;
    for u in vec {
        s += *u;
    }
    return s;
}

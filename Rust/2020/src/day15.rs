use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const TURNS: usize = 2020;

pub struct Day15 {
    nums: Vec<usize>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { nums: Vec::new() };
    }

    fn last_turn(&self, nums: &Vec<usize>, member: usize) -> usize {
        for i in (0..=(nums.len() - 2)).rev() {
            if nums[i] == member {
                return i + 1;
            }
        }
        unreachable!();
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.nums = raw_input.split(',').map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut num_hist = self.nums.clone();
        let mut spoken = HashSet::<usize>::from_iter(num_hist.iter().cloned());
        let mut next = 0;

        while num_hist.len() < TURNS {
            if !spoken.contains(&next) {
                num_hist.push(next);
                spoken.insert(next);
                next = 0;
                continue;
            }
            num_hist.push(next);
            spoken.insert(next);
            let ft = self.last_turn(&num_hist, next);

            next = num_hist.len() - ft;
        }
        return Box::new(num_hist[num_hist.len() - 1]);
    }

    /*
    There may be a pattern that emerges at some point, but this takes ~2.5
    seconds to run and that's ok enough for me.
    */
    fn part2(&mut self) -> Box<dyn Display> {
        const TURNS: usize = 30000000;

        let mut hist = HashMap::new();
        for (i, n) in self.nums.iter().enumerate() {
            hist.insert(*n, i + 1);
        }

        let mut prev_num: usize;
        let mut next_num = 0;

        for turn in (self.nums.len() + 1)..TURNS {
            prev_num = next_num;

            next_num = match hist.get(&next_num) {
                Some(n) => turn - n,
                None => 0,
            };

            hist.insert(prev_num, turn);
        }

        return Box::new(next_num);
    }
}

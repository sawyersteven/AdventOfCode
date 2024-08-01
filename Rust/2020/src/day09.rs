use crate::Base;
use std::fmt::Display;

pub struct Day09 {
    nums: Vec<usize>,
}

const PREAMBLE_LEN: usize = 25;

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { nums: Vec::new() };
    }

    fn sum_exists(&self, nums: &[usize], sum: usize) -> bool {
        for a in nums {
            if sum < *a {
                continue;
            }
            let b = sum - a;
            if *a != b && nums.contains(&b) {
                return true;
            }
        }
        return false;
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.nums = raw_input.lines().map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut preamble_start = 0;

        for i in PREAMBLE_LEN..self.nums.len() {
            let pre_slice = &self.nums[preamble_start..(preamble_start + PREAMBLE_LEN)];
            if !self.sum_exists(pre_slice, self.nums[i]) {
                return Box::new(self.nums[i]);
            }
            preamble_start += 1;
        }

        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const T1: usize = 138879426;
        for i in 0..self.nums.len() {
            let mut t = 0;
            let mut j = i;
            while t < T1 {
                t += self.nums[j];
                j += 1;
            }
            if t == T1 {
                let range = &self.nums[i..j];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                return Box::new(min + max);
            }
        }

        return Box::new("-");
    }
}

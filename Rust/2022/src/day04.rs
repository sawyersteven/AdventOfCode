use crate::Base;
use std::fmt::Display;

pub struct Day04 {
    pub input: Vec<Vec<usize>>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { input: Vec::new() };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let nums = line
                .split(|x| ['-', ' ', ','].contains(&x))
                .map(|x| x.trim().parse::<usize>().unwrap())
                .collect();
            self.input.push(nums);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for nums in &self.input {
            if nums[0] <= nums[2] && nums[1] >= nums[3] {
                count += 1;
            } else if nums[2] <= nums[0] && nums[3] >= nums[1] {
                count += 1;
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for nums in &self.input {
            let r2 = nums[2]..=nums[3];
            if (nums[0]..=nums[1]).any(|x| r2.contains(&x)) {
                count += 1;
            };
        }
        return Box::new(count);
    }
}

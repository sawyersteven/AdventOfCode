use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    pub input: Vec<String>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: Vec::new() };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut count = 0;
        for line in &self.input {
            let mut nums: Vec<usize> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            nums.sort();
            if nums[0] + nums[1] > nums[2] {
                count += 1;
            }
        }
        return Box::new(count);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut count = 0;
        let mut tri = [0, 0, 0];

        for i in (0..self.input.len()).step_by(3) {
            let nums_a: Vec<usize> = self.input[i].split_whitespace().map(|x| x.parse().unwrap()).collect();
            let nums_b: Vec<usize> = self.input[i + 1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            let nums_c: Vec<usize> = self.input[i + 2]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();

            for j in 0..3 {
                tri[0] = nums_a[j];
                tri[1] = nums_b[j];
                tri[2] = nums_c[j];
                tri.sort();
                if tri[0] + tri[1] > tri[2] {
                    count += 1;
                }
            }
        }

        return Box::new(count);
    }
}

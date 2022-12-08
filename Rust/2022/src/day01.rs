use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    pub elves: Vec<String>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { elves: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.elves = raw_input.split("\n\n").map(|x| x.trim().to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut best = 0;
        for elf in &self.elves {
            let sum = elf.split('\n').map(|x| x.parse::<usize>().unwrap()).sum::<usize>();
            best = best.max(sum);
        }
        return Box::new(best);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut totals = vec![0; self.elves.len()];
        for (i, elf) in self.elves.iter().enumerate() {
            let sum = elf
                .split('\n')
                .map(|x| x.trim().parse::<usize>().unwrap())
                .sum::<usize>();
            totals[i] = sum;
        }
        totals.sort();
        return Box::new(totals.iter().rev().take(3).sum::<usize>());
    }
}

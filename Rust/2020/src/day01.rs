use crate::Base;
use std::fmt::Display;

pub struct Day01 {
    expenses: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { expenses: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.expenses = raw_input.lines().map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        for i in 0..self.expenses.len() {
            for j in (i + 1)..self.expenses.len() {
                if self.expenses[i] + self.expenses[j] == 2020 {
                    return Box::new(self.expenses[i] * self.expenses[j]);
                }
            }
        }
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for i in 0..self.expenses.len() {
            for j in (i + 1)..self.expenses.len() {
                for k in (j + 1)..self.expenses.len() {
                    if self.expenses[i] + self.expenses[j] + self.expenses[k] == 2020 {
                        return Box::new(self.expenses[i] * self.expenses[j] * self.expenses[k]);
                    }
                }
            }
        }
        return Box::new("-");
    }
}

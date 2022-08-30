use crate::Base;
use std::fmt::Display;

pub struct Day20 {
    pub input: usize,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { input: 0 };
    }

    /// Returns a sum of every divisor of val
    fn sum_divisors(val: usize) -> usize {
        let mut sum = 0;
        let end = (val as f64).sqrt() as usize + 1;

        for i in 1..end {
            if val % i == 0 {
                sum += i;
                if i != val / i {
                    // i is not sqrt of val
                    sum += val / i;
                }
            }
        }
        return sum;
    }

    fn sum_divisors_2(val: usize) -> usize {
        let mut sum = 0;
        let end = (val as f64).sqrt() as usize + 1;

        for i in 1..end {
            if val % i == 0 {
                if val / i <= 50 {
                    sum += i;
                }

                if i <= 50 {
                    sum += val / i;
                }
            }
        }
        return sum * 11;
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.parse::<usize>().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let target = self.input / 10;

        for i in 1..usize::MAX {
            if Self::sum_divisors(i) >= target {
                return Box::new(i);
            }
        }

        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let target = self.input;
        for i in 1..usize::MAX {
            if Self::sum_divisors_2(i) >= target {
                return Box::new(i);
            }
        }
        return Box::new("-");
    }
}

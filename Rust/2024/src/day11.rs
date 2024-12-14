use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day11 {
    input: String,
    cache: Cache,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            input: String::new(),
            cache: Cache::new(),
        };
    }
}

/// rock: blinks: answer
type Cache = HashMap<usize, HashMap<usize, usize>>;

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        // Cache is {current_rock: [(index of split, a, b])}
        let mut t = 0;
        for r in self
            .input
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
        {
            t += self.blink(r, 25);
        }
        self.cache.clear();
        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        for r in self
            .input
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>()
        {
            t += self.blink(r, 75);
        }
        self.cache.clear();
        return Box::new(t);
    }
}

impl Day11 {
    fn blink(&mut self, mut rock: usize, blinks: usize) -> usize {
        if let Some(a) = self.cache.entry(rock).or_insert(HashMap::new()).get(&blinks) {
            return *a;
        }

        let ogrock = rock;

        for blink in (1..=blinks).rev() {
            if rock == 0 {
                rock = 1;
            } else {
                let num_digits = rock.ilog10() + 1;
                if num_digits % 2 == 1 {
                    rock *= 2024;
                } else {
                    let (a, b) = self.split_at(rock, num_digits / 2);
                    let ans = self.blink(a, blink - 1) + self.blink(b, blink - 1);
                    self.cache.get_mut(&ogrock).unwrap().insert(blinks, ans);
                    return ans;
                }
            }
        }
        return 1;
    }

    /// Splits a number into two at given position from right
    /// eg split_at(123456, 1) == (12345,6)
    #[inline(always)]
    fn split_at(&self, num: usize, split: u32) -> (usize, usize) {
        let div = 10usize.pow((split) as u32);
        let a = num / div;
        let b = num % div;
        return (a, b);
    }
}

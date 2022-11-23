use crate::Base;
use std::{collections::VecDeque, fmt::Display};

const GEN_A_FACTOR: usize = 16807;
const GEN_B_FACTOR: usize = 48271;
const MOD_FACTOR: usize = 2147483647;
const MASK: usize = u16::MAX as usize;

pub struct Day15 {
    gen_a: usize,
    gen_b: usize,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { gen_a: 0, gen_b: 0 };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.gen_a = raw_input
            .split('\n')
            .nth(0)
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap(); // oof

        self.gen_b = raw_input
            .split('\n')
            .nth(1)
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse()
            .unwrap();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut a = self.gen_a;
        let mut b = self.gen_b;

        let mut total = 0;
        for _ in 1..40_000_000 {
            a = (a * GEN_A_FACTOR) % MOD_FACTOR;
            b = (b * GEN_B_FACTOR) % MOD_FACTOR;
            if (a & MASK) == (b & MASK) {
                total += 1;
            }
        }

        return Box::new(total);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<usize>::new();

        let mut a = self.gen_a;
        let mut b = self.gen_b;

        while q.len() < 5_000_000 {
            a = (a * GEN_A_FACTOR) % MOD_FACTOR;
            if a % 4 == 0 {
                q.push_front(a & MASK);
            }
        }

        let mut total = 0;
        let mut i = 0;
        while i < 5_000_000 {
            b = (b * GEN_B_FACTOR) % MOD_FACTOR;
            if b % 8 == 0 {
                i += 1;
                let a = q.pop_back().unwrap();
                if a == (b & MASK) {
                    total += 1;
                }
            }
        }

        return Box::new(total);
    }
}

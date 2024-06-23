use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day08 {
    steps: Vec<usize>,
    nodes: HashMap<u32, [u32; 2]>,
    start: u32,
    end: u32,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 {
            steps: Vec::new(),
            nodes: HashMap::new(),
            start: 0,
            end: 0,
        };
    }
}

const A: u32 = b'A' as u32;
const Z: u32 = b'Z' as u32;
const MASK8: u32 = u32::MAX >> 24;

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        self.steps = lines[0]
            .trim()
            .chars()
            .map(|x| if x == 'L' { 0 } else { 1 })
            .into_iter()
            .collect();

        for line in lines.iter().skip(2).map(|x| x.as_bytes()) {
            let mut k32: u32 = 0;
            for c in line[0..3].iter() {
                k32 <<= 8;
                k32 |= *c as u32;
            }

            let mut a32: u32 = 0;
            for c in line[7..10].iter() {
                a32 <<= 8;
                a32 |= *c as u32;
            }

            let mut b32 = 0;
            for c in line[12..15].iter() {
                b32 <<= 8;
                b32 |= *c as u32;
            }
            self.nodes.insert(k32, [a32, b32]);
        }
        for _ in 0..3 {
            self.start <<= 8;
            self.start |= b'A' as u32;
            self.end <<= 8;
            self.end |= b'Z' as u32;
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut step_ind = 0;
        let mut step_count = 0;

        let mut current = self.start;
        while current != self.end {
            let step = self.steps[step_ind];
            current = self.nodes[&current][step];
            step_ind = (step_ind + 1) % self.steps.len();
            step_count += 1;
        }

        return Box::new(step_count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total_step_count = 1;
        let l = self.steps.len();

        for mut ghost in self.nodes.keys().filter(|x| *x & MASK8 == A) {
            let mut step_ind = 0;
            let mut step_count = 0;
            while ghost & MASK8 != Z {
                ghost = &self.nodes[ghost][self.steps[step_ind]];
                step_ind = (step_ind + 1) % l;
                step_count += 1;
            }
            total_step_count = lcm(step_count, total_step_count);
        }

        return Box::new(total_step_count);
    }
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

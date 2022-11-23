use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day06 {
    pub input: Vec<usize>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { input: Vec::new() };
    }
}

static mut PT1MEM: Vec<usize> = Vec::new();

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split_whitespace().map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut mem = self.input.clone();
        let mut history = HashSet::<String>::new();

        for cycle in 1..usize::MAX {
            run_cycle(&mut mem);
            let id = String::from_iter(mem.iter().map(|x| x.to_string()));
            if history.contains(&id) {
                unsafe {
                    PT1MEM = mem;
                }
                return Box::new(cycle);
            }
            history.insert(id);
        }

        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut mem: Vec<usize>;
        unsafe {
            mem = PT1MEM.clone();
        }

        let target = mem.clone();
        for i in 1..usize::MAX {
            run_cycle(&mut mem);
            if mem == target {
                return Box::new(i);
            }
        }
        return Box::new("-");
    }
}

fn run_cycle(mem: &mut Vec<usize>) {
    let mut ind = 0;
    let mut num = 0;
    for i in 0..mem.len() {
        if mem[i] > num {
            num = mem[i];
            ind = i;
        }
    }
    mem[ind] = 0;

    for _ in 0..num {
        ind = (ind + 1) % mem.len();
        mem[ind] += 1;
        num -= 1;
    }
}

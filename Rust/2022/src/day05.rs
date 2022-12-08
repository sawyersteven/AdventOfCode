use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day05 {
    stacks: Vec<VecDeque<u8>>,
    instructions: Vec<(usize, usize, usize)>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            stacks: Vec::new(),
            instructions: Vec::new(),
        };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        let stack_map: Vec<Vec<u8>> = raw_input
            .split("\n\n")
            .nth(0)
            .unwrap()
            .split('\n')
            .rev()
            .map(|x| x.bytes().collect())
            .collect();

        for (i, c) in stack_map[0].iter().enumerate() {
            if c >= &b'1' {
                let mut stk = VecDeque::<u8>::new();
                for row in 1..stack_map.len() {
                    if stack_map[row][i] != b' ' {
                        stk.push_front(stack_map[row][i]);
                    }
                }
                self.stacks.push(stk);
            }
        }

        for line in raw_input.split("\n\n").nth(1).unwrap().split('\n') {
            let parts = line.split(' ').collect::<Vec<&str>>();
            let mut inst = (0, 0, 0);
            inst.0 = parts[1].parse().unwrap();
            inst.1 = parts[3].parse().unwrap();
            inst.2 = parts[5].parse().unwrap();
            self.instructions.push(inst);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut stacks = self.stacks.clone();
        for inst in &self.instructions {
            for _ in 0..inst.0 {
                let t = stacks[inst.1 - 1].pop_front().unwrap();
                stacks[inst.2 - 1].push_front(t);
            }
        }

        let result = String::from_iter(stacks.iter().map(|x| x[0] as char));

        return Box::new(result);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut stacks = self.stacks.clone();
        let mut tmp_stack = VecDeque::<u8>::new();
        for inst in &self.instructions {
            for _ in 0..inst.0 {
                tmp_stack.push_front(stacks[inst.1 - 1].pop_front().unwrap());
            }
            tmp_stack.drain(0..).for_each(|x| stacks[inst.2 - 1].push_front(x));
        }

        let result = String::from_iter(stacks.iter().map(|x| x[0] as char));

        return Box::new(result);
    }
}

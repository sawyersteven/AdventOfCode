use crate::Base;
use std::fmt::Display;

pub struct Day09 {
    pub input: Vec<Vec<isize>>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: Vec::new() };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let n = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
            self.input.push(n);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for line in &self.input {
            let mut stack: Vec<Vec<isize>> = Vec::new();
            stack.push(line.clone());

            find_zeroes(&mut stack);

            let last_i = stack.len() - 1;
            stack[last_i].push(0);

            find_right_values(&mut stack);
            sum += *stack[0].last().unwrap();
        }
        return Box::new(sum);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for line in &self.input {
            let mut stack: Vec<Vec<isize>> = Vec::new();
            stack.push(line.clone());

            find_zeroes(&mut stack);

            let last_i = stack.len() - 1;
            stack[last_i].push(0);

            find_left_values(&mut stack);
            sum += stack[0][0];
        }
        return Box::new(sum);
    }
}

fn find_right_values(stack: &mut Vec<Vec<isize>>) {
    for i in (0..stack.len() - 1).rev() {
        let a = stack[i].last().unwrap();
        let b = stack[i + 1].last().unwrap();
        let n = a + b;
        stack[i].push(n);
    }
}

fn find_left_values(stack: &mut Vec<Vec<isize>>) {
    for i in (0..stack.len() - 1).rev() {
        let a = stack[i][0];
        let b = stack[i + 1][0];
        let n = a - b;
        stack[i].insert(0, n);
    }
}

fn find_zeroes(stack: &mut Vec<Vec<isize>>) {
    // work down to zeros
    for i in 0..usize::MAX {
        let capacity = stack[i].len() - 1;
        let mut next = vec![0; capacity];

        for j in 0..capacity {
            next[j] = stack[i][j + 1] - stack[i][j];
        }
        stack.push(next);

        if stack[i + 1].iter().all(|x| *x == 0) {
            break;
        }
    }
}

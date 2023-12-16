use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day08 {
    steps: Vec<usize>,
    nodes: HashMap<[u8; 3], [[u8; 3]; 2]>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 {
            steps: Vec::new(),
            nodes: HashMap::new(),
        };
    }
}

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
            let mut ka = [0; 3];
            ka.clone_from_slice(&line[0..3]);

            let mut va = [0; 3];
            va.clone_from_slice(&line[7..10]);
            let mut vb = [0; 3];
            vb.clone_from_slice(&line[12..15]);

            self.nodes.insert(ka, [va, vb]);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut step_ind = 0;
        let mut step_count = 0;

        let mut current = [b'A', b'A', b'A'];
        while current != [b'Z', b'Z', b'Z'] {
            let step = self.steps[step_ind];
            current = self.nodes[&current][step];
            step_ind = (step_ind + 1) % self.steps.len();
            step_count += 1;
        }

        return Box::new(step_count);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut total_step_count = 1;
        let l =self.steps.len();

        for mut ghost in self.nodes.keys().filter(|x| x[2] == b'A') {
            let mut step_ind = 0;
            let mut step_count = 0;
            while !(ghost[2] == b'Z') {
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

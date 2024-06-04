use crate::{day16::Computer, Base};
use std::fmt::Display;

pub struct Day19 {
    inst_ptr: usize,
    code: Vec<[usize; 4]>,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            code: Vec::new(),
            inst_ptr: 0,
        };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        self.inst_ptr = raw_input
            .split('\n')
            .nth(0)
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        self.code = Computer::parse_input_code(&raw_input.split('\n').skip(1).collect());
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut mem = vec![0; 6];

        while mem[self.inst_ptr] < self.code.len() {
            let inst = self.code[mem[self.inst_ptr]];
            Computer::OPS[inst[0]](&mut mem, &inst);
            mem[self.inst_ptr] += 1;
        }
        return Box::new(mem[0]);
    }

    /*
    For part 2 the code very inefficiently generates a large number then
    calcs the sum of its prime factors. The shortcut here is to run the
    program as-is until the last register stabilizes, then sum the prime
    factors the easy/fast way
    */
    fn part2(&mut self) -> Box<dyn Display> {
        const SAMPLE_RATE: usize = 10;
        let mut mem = vec![1, 0, 0, 0, 0, 0];

        let mut last_reg = mem[5];
        for i in 1..usize::MAX {
            let inst = self.code[mem[self.inst_ptr]];
            Computer::OPS[inst[0]](&mut mem, &inst);
            mem[self.inst_ptr] += 1;

            if i % SAMPLE_RATE == 0 {
                if last_reg == mem[5] {
                    break;
                }
                last_reg = mem[5];
            }
        }
        return Box::new(prime_factors(last_reg).iter().sum::<usize>());
    }
}

fn prime_factors(n: usize) -> Vec<usize> {
    let mut f = Vec::new();
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            f.push(i);
            if n / i != i {
                f.push(n / i);
            }
        }
        i += 1;
    }
    return f;
}

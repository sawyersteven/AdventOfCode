use crate::{day16::Computer, Base};
use std::{collections::HashSet, fmt::Display};

pub struct Day21 {
    inst_ptr: usize,
    code: Vec<[usize; 4]>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            code: Vec::new(),
            inst_ptr: 0,
        };
    }
}

impl Base for Day21 {
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

        while mem[self.inst_ptr] != 30 {
            let inst = self.code[mem[self.inst_ptr]];
            Computer::OPS[inst[0]](&mut mem, &inst);
            mem[self.inst_ptr] += 1;
        }
        return Box::new(mem[3]);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut results = HashSet::new();

        let mut pp = PortedProgram::new(0);

        let mut prev = 0;
        while let Some(r) = pp.next() {
            if results.contains(&r) {
                break;
            }
            prev = r;
            results.insert(r);
        }
        return Box::new(prev);
    }
}

struct PortedProgram {
    mem: [usize; 6],
    finished: bool,
}

impl PortedProgram {
    pub fn new(reg_1: usize) -> Self {
        let mut s = PortedProgram {
            mem: [reg_1, 0, 0, 0, 0, 0],
            finished: false,
        };

        s.mem[1] = s.mem[3] | 65536;
        s.mem[3] = 9450265;
        return s;
    }

    // Runs until next output is generated
    // If program has completed, result will be None
    pub fn next(&mut self) -> Option<usize> {
        let mut rv = None;
        if self.finished {
            return rv;
        }
        loop {
            self.mem[3] += self.mem[1] & 255;
            self.mem[3] &= 16777215;
            self.mem[3] *= 65899;
            self.mem[3] &= 16777215;

            if 256 <= self.mem[1] {
                self.mem[1] /= 256;
                continue;
            }
            rv = Some(self.mem[3]);

            if self.mem[3] == self.mem[0] {
                self.finished = true;
                break;
            }
            self.mem[1] = self.mem[3] | 65536;
            self.mem[3] = 9450265;
            break;
        }
        return rv;
    }
}

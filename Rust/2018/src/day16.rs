use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day16 {
    pub input: Vec<String>,
    pt2code: Vec<[usize; 4]>,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            input: Vec::new(),
            pt2code: Vec::new(),
        };
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();

        self.pt2code = raw_input
            .split("\n\n\n\n")
            .nth(1)
            .unwrap()
            .split('\n')
            .map(|line| self.parse_instruction_ints(line, ' '))
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for i in (0..self.input.len()).step_by(4) {
            if self.input[i].is_empty() {
                break;
            }

            let before = self.parse_instruction_ints(&self.input[i][9..(self.input[i].len() - 1)], ',');
            let inst = self.parse_instruction_ints(&self.input[i + 1], ' ');
            let after = self.parse_instruction_ints(&self.input[i + 2][9..(self.input[i].len() - 1)], ',');

            let mut matches = 0;
            for op in Computer::OPS {
                let mut b4 = before.to_vec();
                op(&mut b4, &inst);
                if b4 == after {
                    matches += 1;
                }
                if matches == 3 {
                    total += 1;
                    break;
                }
            }
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let ordered_codes = self.sort_op_codes();
        let mut mem = vec![0; 4];
        for inst in &self.pt2code {
            ordered_codes[inst[0]](&mut mem, &inst);
        }

        return Box::new(mem[0]);
    }
}

impl Day16 {
    fn parse_instruction_ints(&self, line: &str, sep: char) -> [usize; 4] {
        let x: Vec<usize> = line.split(sep).map(|x| x.trim().parse().unwrap()).collect();
        return x.try_into().unwrap();
    }

    fn sort_op_codes(&self) -> Vec<fn(&mut Vec<usize>, &[usize; 4])> {
        let mut possibilities: HashMap<usize, HashSet<usize>> = HashMap::new();
        for i in 0..Computer::OPS.len() {
            possibilities.insert(i, HashSet::new());
        }

        for i in (0..self.input.len()).step_by(4) {
            if self.input[i].is_empty() {
                break;
            }

            let before = self.parse_instruction_ints(&self.input[i][9..(self.input[i].len() - 1)], ',');
            let inst = self.parse_instruction_ints(&self.input[i + 1], ' ');
            let after = self.parse_instruction_ints(&self.input[i + 2][9..(self.input[i].len() - 1)], ',');

            for op_num in 0..possibilities.len() {
                let mut b4 = before.to_vec();

                Computer::OPS[op_num](&mut b4, &inst);
                if b4 == after {
                    possibilities.get_mut(&op_num).unwrap().insert(inst[0]);
                }
            }
        }

        let mut val: usize;
        let mut ordered_ops: Vec<fn(&mut Vec<usize>, &[usize; 4])> = vec![Computer::OPS[0]; 16];

        while possibilities.len() > 0 {
            let keys: Vec<usize> = possibilities.keys().map(|x| *x).collect();

            for k in &keys {
                if possibilities[k].len() == 1 {
                    val = possibilities.get_mut(k).unwrap().drain().nth(0).unwrap();
                    for k2 in &keys {
                        if k == k2 {
                            continue;
                        }
                        match possibilities.get_mut(k2) {
                            Some(hs) => {
                                hs.remove(&val);
                            }
                            None => {}
                        }
                    }
                    ordered_ops[val] = Computer::OPS[*k];
                    possibilities.remove(k);
                }
            }
        }

        return ordered_ops;
    }
}

struct Computer {}

impl Computer {
    const A: usize = 1;
    const B: usize = 2;
    const C: usize = 3;

    #[allow(unused)]
    pub fn parse_input_code(raw_code: &Vec<&str>) -> Vec<[usize; 4]> {
        let mut code = Vec::new();
        for line in raw_code {
            let mut inst = [0, 0, 0, 0];
            let parts: Vec<&str> = line.split(' ').collect();
            inst[0] = Self::op_num(parts[0]);
            inst[1] = parts[1].parse().unwrap();
            inst[2] = parts[2].parse().unwrap();
            inst[3] = parts[3].parse().unwrap();
            code.push(inst);
        }
        return code;
    }

    const OPS: [fn(&mut Vec<usize>, &[usize; 4]); 16] = [
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] + mem[inst[Self::B]],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] + inst[Self::B],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] * mem[inst[Self::B]],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] * inst[Self::B],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] & mem[inst[Self::B]],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] & inst[Self::B],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] | mem[inst[Self::B]],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]] | inst[Self::B],
        |mem, inst| mem[inst[Self::C]] = mem[inst[Self::A]],
        |mem, inst| mem[inst[Self::C]] = inst[Self::A],
        |mem, inst| mem[inst[Self::C]] = if inst[Self::A] > mem[inst[Self::B]] { 1 } else { 0 },
        |mem, inst| mem[inst[Self::C]] = if mem[inst[Self::A]] > inst[Self::B] { 1 } else { 0 },
        |mem, inst| mem[inst[Self::C]] = if mem[inst[Self::A]] > mem[inst[Self::B]] { 1 } else { 0 },
        |mem, inst| mem[inst[Self::C]] = if inst[Self::A] == mem[inst[Self::B]] { 1 } else { 0 },
        |mem, inst| mem[inst[Self::C]] = if mem[inst[Self::A]] == inst[Self::B] { 1 } else { 0 },
        |mem, inst| mem[inst[Self::C]] = if mem[inst[Self::A]] == mem[inst[Self::B]] { 1 } else { 0 },
    ];

    fn op_num(name: &str) -> usize {
        match name {
            "addr" => 1,
            "addi" => 2,
            "mulr" => 3,
            "muli" => 4,
            "banr" => 5,
            "bani" => 6,
            "borr" => 7,
            "bori" => 8,
            "setr" => 9,
            "seti" => 10,
            "gtir" => 11,
            "gtri" => 12,
            "gtrr" => 13,
            "eqir" => 14,
            "eqri" => 15,
            "eqrr" => 16,
            _ => panic!(),
        }
    }
}

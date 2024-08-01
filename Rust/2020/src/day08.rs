use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day08 {
    code: Vec<(String, isize)>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { code: Vec::new() };
    }

    fn validate_code(&self, code: &Vec<(String, isize)>) -> Option<isize> {
        let mut acc = 0;
        let mut line_no = 0isize;
        let mut consumed_lines = HashSet::new();

        loop {
            if line_no as usize > code.len() - 1 {
                break;
            }

            if !consumed_lines.insert(line_no) {
                return None;
            }

            let inst = &code[line_no as usize];

            match &*inst.0 {
                "nop" => {
                    line_no += 1;
                }
                "acc" => {
                    acc += inst.1;
                    line_no += 1;
                }
                "jmp" => {
                    line_no += inst.1;
                }
                _ => {}
            }
        }
        return Some(acc);
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut s = line.split(' ');
            let inst = s.next().unwrap().to_string();
            let val = s.next().unwrap().parse().unwrap();
            self.code.push((inst, val));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut acc = 0;
        let mut line_no = 0isize;
        let mut consumed_lines = HashSet::new();

        loop {
            if !consumed_lines.insert(line_no) {
                break;
            }
            let inst = &self.code[line_no as usize];

            match &*inst.0 {
                "nop" => {
                    line_no += 1;
                }
                "acc" => {
                    acc += inst.1;
                    line_no += 1;
                }
                "jmp" => {
                    line_no += inst.1;
                }
                _ => {
                    unreachable!()
                }
            }
        }

        return Box::new(acc);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut code = self.code.clone();

        for line_no in 0..self.code.len() {
            let inst = &self.code[line_no];

            let og = inst.0.clone();
            match &*inst.0 {
                "nop" => {
                    code[line_no].0 = String::from("jmp");
                }
                "jmp" => {
                    code[line_no].0 = String::from("nop");
                }
                _ => {}
            }

            match self.validate_code(&code) {
                Some(a) => return Box::new(a),
                None => code[line_no].0 = og,
            }
        }

        return Box::new("-");
    }
}

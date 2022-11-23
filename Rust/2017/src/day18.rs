use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day18 {
    pub input: String,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 {
            input: String::from(""),
        };
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let prog = self.input.split('\n').map(|x| x.to_string()).collect();
        let mut d = Duet::new(0, prog);
        d.run();
        let last = d.sent_vals.pop_back().unwrap();

        return Box::new(last);
    }

    fn part2(&self) -> Box<dyn Display> {
        // todo:
        return Box::new("-");
    }
}

struct Duet {
    input_queue: VecDeque<isize>,
    prog: Vec<String>,
    registers: [isize; 26],
    sent_vals: VecDeque<isize>,
}

impl Duet {
    pub fn new(id: isize, prog: Vec<String>) -> Self {
        let mut registers = [0; 26];
        registers[(b'p' - b'a') as usize] = id;

        return Duet {
            input_queue: VecDeque::new(),
            prog: prog,
            registers: [id; 26],
            sent_vals: VecDeque::new(),
        };
    }

    pub fn recv(&mut self, val: isize) {
        self.input_queue.push_back(val);
    }

    pub fn run(&mut self) -> bool {
        let mut line_num: isize = 0;
        while line_num < self.prog.len() as isize {
            let line = &self.prog[line_num as usize];
            let parts: Vec<&str> = line.split(' ').collect();
            line_num += 1;

            match parts[0] {
                "snd" => {
                    let v = self.get_value(parts[1]);
                    self.sent_vals.push_back(v);
                }
                "set" => {
                    self.registers[self.get_reg_ind(parts[1])] = self.get_value(parts[2]);
                }
                "add" => {
                    self.registers[self.get_reg_ind(parts[1])] += self.get_value(parts[2]);
                }
                "mul" => {
                    self.registers[self.get_reg_ind(parts[1])] *= self.get_value(parts[2]);
                }
                "mod" => {
                    self.registers[self.get_reg_ind(parts[1])] %= self.get_value(parts[2]);
                }
                "rcv" => {
                    if self.input_queue.len() == 0 {
                        return false;
                    }
                    self.registers[self.get_reg_ind(parts[1])] = self.input_queue.pop_front().unwrap();
                }
                "jgz" => {
                    if self.get_value(parts[1]) > 0 {
                        line_num += self.get_value(parts[2]) - 1;
                    }
                }
                _ => panic!(),
            }
        }
        return true;
    }

    fn get_reg_ind(&self, word: &str) -> usize {
        return word.chars().nth(0).unwrap() as usize - A;
    }

    fn get_value(&self, inst: &str) -> isize {
        let c = inst.chars().nth(0).unwrap() as usize;
        if c >= A && c <= Z as usize {
            return self.registers[c - A];
        } else {
            return inst.parse().unwrap();
        }
    }
}

const A: usize = b'a' as usize;
const Z: usize = b'z' as usize;

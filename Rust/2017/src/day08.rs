use crate::Base;
use std::{collections::HashMap, fmt::Display};

static mut PART2: isize = 0;

pub struct Day08 {
    pub input: String,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 {
            input: String::from(""),
        };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut registers = HashMap::<&str, isize>::new();
        let mut biggest: isize = isize::MIN;

        for line in self.input.split('\n') {
            let parts: Vec<&str> = line.split(' ').collect();
            if !registers.contains_key(parts[0]) {
                registers.insert(parts[0], 0);
            }
            if !registers.contains_key(parts[4]) {
                registers.insert(parts[4], 0);
            }

            if eval_condition(registers[parts[4]], parts[5], parts[6].parse().unwrap()) {
                let v: isize = parts[2].parse::<isize>().unwrap()
                    * if parts[1].chars().nth(0).unwrap() == 'i' { 1 } else { -1 };
                registers.entry(parts[0]).and_modify(|x| *x += v);
                biggest = biggest.max(registers[parts[0]]);
            }
        }

        unsafe {
            PART2 = biggest;
        }

        let mut max_val = isize::MIN;
        for v in registers.values() {
            max_val = max_val.max(*v);
        }

        return Box::new(max_val);
    }

    fn part2(&self) -> Box<dyn Display> {
        let p2: isize;
        unsafe {
            p2 = PART2;
        }

        return Box::new(p2);
    }
}

fn eval_condition(a: isize, op: &str, b: isize) -> bool {
    return match op {
        ">" => a > b,
        "<" => a < b,
        ">=" => a >= b,
        "<=" => a <= b,
        "==" => a == b,
        "!=" => a != b,
        _ => panic!(),
    };
}

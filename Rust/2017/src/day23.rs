use crate::Base;
use std::fmt::Display;

pub struct Day23 {
    pub input: Vec<String>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: Vec::new() };
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input.push(line.to_string());
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let val = run_code(&self.input);
        return Box::new(val);
    }

    fn part2(&self) -> Box<dyn Display> {
        /* Runs optimized/rewritten code instead of from raw_input
        tl;dr:
                                          1000
        How many non-prime numbers are in ∑ 106500+(n*17)
                                          n=0
        */

        // These are the only registers we need to start part 2
        // To run as an optimized part one, zero out all registers to start
        let mut b = 106500;
        let c = b + 17000;
        let mut h = 0;

        while b <= c {
            h += if is_prime(b) { 0 } else { 1 };
            b += 17;
        }

        return Box::new(h);
    }
}

fn is_prime(num: isize) -> bool {
    if num % 2 == 0 {
        return false;
    }

    for i in (3..num).step_by(2) {
        if num % i == 0 {
            return false;
        }
    }
    return true;
}

fn get_value(instruction: &str, registers: &[isize; 26]) -> isize {
    if instruction.as_bytes()[0] >= b'a' && instruction.as_bytes()[0] <= b'z' {
        return registers[(instruction.as_bytes()[0] - b'a') as usize];
    } else {
        return instruction.parse().unwrap();
    }
}

fn run_code(code: &Vec<String>) -> usize {
    let mut count = 0;
    let mut registers = [0; 26];

    let mut line_num: isize = 0;
    loop {
        let line = &code[line_num as usize];
        let parts: Vec<&str> = line.split(' ').collect();

        match code[line_num as usize].as_bytes()[2] {
            b't' => registers[(parts[1].as_bytes()[0] - b'a') as usize] = get_value(parts[2], &registers),

            b'b' => registers[(parts[1].as_bytes()[0] - b'a') as usize] -= get_value(parts[2], &registers),

            b'l' => {
                count += 1;
                registers[(parts[1].as_bytes()[0] - b'a') as usize] *= get_value(parts[2], &registers);
            }
            b'z' => {
                let v1 = get_value(parts[1], &registers);
                if v1 != 0 {
                    line_num += get_value(parts[2], &registers) - 1;
                }
            }

            _ => panic!(),
        }

        line_num += 1;
        if line_num >= code.len() as isize {
            break;
        }
    }

    return count;
}

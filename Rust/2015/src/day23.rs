use crate::Base;
use std::fmt::Display;

pub struct Day23 {
    pub input: Vec<String>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: Vec::new() };
    }

    fn run(&self, registers: &mut [u64; 2]) {
        let mut i: isize = 0;
        while i < self.input.len() as isize {
            let line = &self.input[i as usize];
            let chars: Vec<char> = line.chars().collect();
            match chars[2] {
                'f' => registers[chars[4] as usize - 97] /= 2, // hlf
                'l' => registers[chars[4] as usize - 97] *= 3, // tpl
                'c' => registers[chars[4] as usize - 97] += 1, // inc

                'p' => i += line.split(' ').last().unwrap().parse::<isize>().unwrap() - 1, // jmp

                'e' => {
                    // jie
                    if registers[chars[4] as usize - 97] % 2 == 0 {
                        i += line.split(' ').last().unwrap().parse::<isize>().unwrap() - 1;
                    }
                }
                'o' => {
                    // jio
                    if registers[chars[4] as usize - 97] == 1 {
                        i += line.split(' ').last().unwrap().parse::<isize>().unwrap() - 1;
                    }
                }
                _ => panic!(),
            }

            i += 1;
        }
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut registers = [0, 0];
        Self::run(&self, &mut registers);

        return Box::new(registers[1]);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut a: u64 = 113383;
        let mut b: u64 = 0;

        while a != 1 {
            b += 1;
            if a % 2 == 0 {
                a /= 2;
                continue;
            }
            a *= 3;
            a += 1;
        }

        return Box::new(b);
    }
}

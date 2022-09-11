use crate::Base;
use std::fmt::Display;

const SAFE: u8 = 0;
const TRAP: u8 = 1;

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
        return Box::new(count_safe(&self.input, 40));
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new(count_safe(&self.input, 400000));
    }
}

fn count_safe(input: &String, rows: usize) -> usize {
    let mut current = input
        .as_bytes()
        .iter()
        .map(|x| if *x == b'^' { TRAP } else { SAFE })
        .collect::<Vec<u8>>();
    let mut next = (0..input.len()).map(|_| SAFE).collect::<Vec<u8>>();

    let mut count = 0;

    count += current.iter().filter(|x| *x == &SAFE).count();

    for _ in 1..rows {
        for x in 0..current.len() as isize {
            let mut b = 0b000;
            for z in -1..2 {
                b <<= 1;
                if x + z < 0 || x + z >= input.len() as isize {
                    continue;
                }

                b += current[(x + z) as usize];
            }
            if b == 0b110 || b == 0b011 || b == 0b100 || b == 0b001 {
                next[x as usize] = TRAP;
            }
        }

        count += next.iter().filter(|x| *x == &SAFE).count();

        current = next.to_owned();
        next = (0..next.len()).map(|_| SAFE).collect();
    }

    return count;
}

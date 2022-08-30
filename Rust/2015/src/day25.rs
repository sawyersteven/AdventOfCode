use crate::Base;
use std::fmt::Display;

const SEED: usize = 20151125;

pub struct Day25 {
    pub input: Vec<String>,
    col: usize,
    row: usize,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 {
            input: Vec::new(),
            col: 0,
            row: 0,
        };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .split([' ', '.', ','])
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();

        self.col = self.input[self.input.len() - 1].parse().unwrap();
        self.row = self.input[self.input.len() - 3].parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let d = dist(self.row, self.col);

        let mut code = SEED;
        for _ in 0..d {
            code = (code * 252533) % 33554393;
        }

        return Box::new(code);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("*");
    }
}

fn dist(row: usize, col: usize) -> usize {
    let mut d = col - 1;
    for i in 0..(row + col - 1) {
        d += i;
    }
    return d;
}

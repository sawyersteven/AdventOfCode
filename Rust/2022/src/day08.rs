use crate::Base;
use std::fmt::Display;

pub struct Day08 {
    input: Vec<Vec<u8>>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: Vec::new() };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input
                .push(line.trim().as_bytes().iter().map(|x| *x as u8).collect());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;

        for y in 0..(self.input.len()) {
            for x in 0..(self.input[0].len()) {
                let h = self.input[y][x];
                if (0..x).all(|i| self.input[y][i] < h)
                    || ((x + 1)..self.input[0].len()).all(|i| self.input[y][i] < h)
                    || (0..y).all(|i| self.input[i][x] < h)
                    || ((y + 1)..self.input.len()).all(|i| self.input[i][x] < h)
                {
                    count += 1;
                }
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut best = 0;

        for y in 0..self.input.len() {
            for x in 0..self.input[0].len() {
                let h = self.input[y][x];

                let mut counts = [0, 0, 0, 0]; // w,e,n,s
                for i in (0..x).rev() {
                    counts[0] += 1;
                    if self.input[y][i] >= h {
                        break;
                    }
                }

                for i in (x + 1)..self.input[0].len() {
                    counts[1] += 1;
                    if self.input[y][i] >= h {
                        break;
                    }
                }

                for i in (0..y).rev() {
                    counts[2] += 1;
                    if self.input[i][x] >= h {
                        break;
                    }
                }

                for i in (y + 1)..self.input.len() {
                    counts[3] += 1;
                    if self.input[i][x] >= h {
                        break;
                    }
                }

                best = best.max(counts.iter().product());
            }
        }

        return Box::new(best);
    }
}

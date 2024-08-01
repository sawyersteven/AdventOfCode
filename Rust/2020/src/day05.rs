use crate::Base;
use std::fmt::Display;

pub struct Day05 {
    ids: Vec<usize>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 { ids: Vec::new() };
    }

    fn get_row(&self, code: &str) -> usize {
        let mut l: usize = 0;
        let mut u: usize = 127;

        for b in code.as_bytes() {
            let h: usize = (u + 1 - l) / 2;
            if *b == b'F' {
                u -= h;
            } else {
                l += h;
            }
        }
        if u != l {
            panic!()
        }
        return u;
    }

    fn get_seat(&self, code: &str) -> usize {
        let mut l = 0;
        let mut u = 7;

        for b in code.as_bytes() {
            let h: usize = (u + 1 - l) / 2;
            if *b == b'L' {
                u -= h;
            } else {
                l += h;
            }
        }
        if u != l {
            panic!();
        }
        return u;
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let r = self.get_row(&line[0..7]);
            let s = self.get_seat(&line[7..]);
            self.ids.push(r * 8 + s);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(*self.ids.iter().max().unwrap());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        self.ids.sort();
        for i in 0..(self.ids.len() - 1) {
            if self.ids[i] + 2 == self.ids[i + 1] {
                return Box::new(self.ids[i] + 1);
            }
        }
        return Box::new("-");
    }
}

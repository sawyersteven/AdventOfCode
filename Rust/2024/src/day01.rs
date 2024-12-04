use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day01 {
    l: Vec<isize>,
    r: Vec<isize>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 {
            l: Vec::new(),
            r: Vec::new(),
        };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut s = line.split("   ");
            self.l.push(s.next().unwrap().parse().unwrap());
            self.r.push(s.next().unwrap().parse().unwrap());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.l.sort();
        self.r.sort();

        let mut d = 0;
        for i in 0..self.l.len() {
            d += (self.l[i] - self.r[i]).abs();
        }
        return Box::new(d);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut ans = 0;
        let mut counts = HashMap::new();
        for n in &self.l {
            let c = counts.get(&n).cloned().unwrap_or_else(|| {
                let v = self.r.iter().filter(|x| *x == n).count() as isize;
                counts.insert(n, v);
                return v;
            });
            ans += n * c;
        }
        return Box::new(ans);
    }
}

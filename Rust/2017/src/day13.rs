use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day13 {
    pub ranges: HashMap<isize, isize>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 {
            ranges: HashMap::<isize, isize>::new(),
        };
    }

    fn calc_severity(&self) -> isize {
        let mut severity = 0;
        for (time, range) in &self.ranges {
            if self.scanner_at_zero(*time, *range) {
                severity += time * range;
            }
        }
        return severity;
    }

    fn scanner_at_zero(&self, time: isize, range: isize) -> bool {
        let rf = (range * 2) - 2;
        let ind = time % rf;
        return ind == 0 || (rf - ind) == 0;
    }

    fn caught_by_scanner(&self, offset: isize) -> bool {
        for (time, range) in &self.ranges {
            if self.scanner_at_zero(time + offset, *range) {
                return true;
            }
        }
        return false;
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts: Vec<isize> = line.split(": ").map(|x| x.parse().unwrap()).collect();
            self.ranges.insert(parts[0], parts[1]);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        return Box::new(self.calc_severity());
    }

    fn part2(&self) -> Box<dyn Display> {
        for offset in 0..isize::MAX {
            if self.scanner_at_zero(offset, self.ranges[&0]) {
                continue;
            }

            if !self.caught_by_scanner(offset) {
                return Box::new(offset);
            }
        }
        return Box::new("-");
    }
}

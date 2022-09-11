use crate::Base;
use std::{fmt::Display, ops::Range};

pub struct Day20 {
    pub ranges: Vec<Range<u32>>,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { ranges: Vec::new() };
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts: Vec<u32> = line.split('-').map(|x| x.parse().unwrap()).collect();
            self.ranges.push(parts[0]..parts[1]);
        }
        self.ranges.sort_by(|a, b| a.start.cmp(&b.start));
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut high = self.ranges[0].end;
        for i in 1..self.ranges.len() {
            if self.ranges[i].start > high + 1 {
                break;
            }
            high = high.max(self.ranges[i].end);
        }
        return Box::new(high + 1);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut count = 0;

        let mut outer_range = self.ranges[0].clone();

        for i in 0..self.ranges.len() {
            if outer_range.end == u32::MAX {
                break;
            }

            let current = &self.ranges[i];

            if current.start > outer_range.end + 1 {
                count += (outer_range.end - outer_range.start) + 1;
                (outer_range.start, outer_range.end) = (current.start, current.end);
                if outer_range.end == u32::MAX {
                    break;
                }
            }
            outer_range.end = outer_range.end.max(current.end);
        }
        count += outer_range.end - outer_range.start + 1;
        return Box::new(u32::MAX - count + 1);
    }
}

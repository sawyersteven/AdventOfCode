use crate::Base;
use std::{collections::HashSet, fmt::Display, hash::Hash, ops::Range};

#[derive(Debug, Eq, Hash, PartialEq)]
struct Sensor {
    x: isize,
    y: isize,
    r: isize,
}

pub fn manhattan(x: isize, y: isize, x2: isize, y2: isize) -> isize {
    return (x.abs_diff(x2) + y.abs_diff(y2)) as isize;
}

pub struct Day15 {
    sensors: Vec<Sensor>,
    beacons: HashSet<(isize, isize)>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 {
            sensors: Vec::new(),
            beacons: HashSet::new(),
        };
    }

    // These are going to be used as *inclusive* ranges, but the built-in
    // type RangeInclusive cannot have its start or end modified so
    // it will just be regular ranges
    fn find_unoccupied_ranges(&self, line: isize) -> Vec<Range<isize>> {
        let mut empty_ranges = Vec::<Range<isize>>::new();
        for s in &self.sensors {
            let line_dist = (s.y - line).abs();
            if line_dist > s.r {
                continue;
            }
            let half_w = s.r - line_dist;
            empty_ranges.push((s.x - half_w)..(s.x + half_w));
        }
        reduce_ranges(&mut empty_ranges);
        return empty_ranges;
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let mut s = Sensor { x: 0, y: 0, r: 0 };
            let parts: Vec<&str> = line.split(' ').collect();
            s.x = parts[2][2..(parts[2].len() - 1)].parse().unwrap();
            s.y = parts[3][2..(parts[3].len() - 1)].parse().unwrap();

            let b_x = parts[8][2..(parts[8].len() - 1)].parse().unwrap();
            let b_y = parts[9][2..].parse().unwrap();

            s.r = manhattan(s.x, s.y, b_x, b_y);
            self.sensors.push(s);
            self.beacons.insert((b_x, b_y));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const LINE: isize = 2000000;

        let empty_ranges = self.find_unoccupied_ranges(LINE);

        let mut count = 0;
        for r in empty_ranges {
            count += r.end - r.start;
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for line in 0..4_000_000 {
            let empty_ranges = self.find_unoccupied_ranges(line);
            if empty_ranges.len() == 2 {
                if empty_ranges[0].end < empty_ranges[1].start {
                    return Box::new(4000000 * (empty_ranges[0].end + 1) + line);
                } else {
                    return Box::new(4000000 * (empty_ranges[1].end + 1) + line);
                }
            }
        }
        return Box::new("-");
    }
}

fn reduce_ranges(ranges: &mut Vec<Range<isize>>) {
    // Would be easier with c-style loops, but this mess works.
    loop {
        let mut changed = false;
        for i in 0..(ranges.len() - 1) {
            for j in (i + 1)..ranges.len() {
                if ranges[i].start <= ranges[j].end && ranges[j].start <= ranges[i].end {
                    ranges[i].start = ranges[i].start.min(ranges[j].start);
                    ranges[i].end = ranges[i].end.max(ranges[j].end);
                    ranges.remove(j);
                    changed = true;
                    break;
                }
            }
        }
        if !changed {
            break;
        }
    }
}

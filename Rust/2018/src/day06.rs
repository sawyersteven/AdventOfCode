use crate::Base;
use std::fmt::Display;

pub struct Day06 {
    points: Vec<(isize, isize)>,
    min_x: isize,
    min_y: isize,
    max_x: isize,
    max_y: isize,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 {
            points: Vec::new(),
            min_x: isize::MAX,
            min_y: isize::MAX,
            max_x: 0,
            max_y: 0,
        };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        let lines = raw_input.split('\n');

        for line in lines {
            let parts: Vec<&str> = line.split(", ").collect();
            let point = (parts[0].parse().unwrap(), parts[1].parse().unwrap());
            self.max_x = self.max_x.max(point.0);
            self.min_x = self.min_x.min(point.0);
            self.max_y = self.max_y.max(point.1);
            self.min_y = self.min_y.min(point.1);
            self.points.push(point);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut coverage = vec![0; self.points.len()];
        for x in self.min_x..(self.max_x + 1) {
            for y in self.min_y..(self.max_y + 1) {
                let mut solo = false;
                let mut best_pt = 0;
                let mut shortest_dist = isize::MAX;

                for i in 0..self.points.len() {
                    let (px, py) = self.points[i];
                    let dist = (x - px).abs() + (y - py).abs();

                    if dist == shortest_dist {
                        solo = false;
                    } else if dist < shortest_dist {
                        solo = true;
                        shortest_dist = dist;
                        best_pt = i;
                    }
                }
                if solo {
                    coverage[best_pt] += 1;
                }
            }
        }
        return Box::new(coverage.iter().max().unwrap().clone());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const MAX_DIST: isize = 10000;
        let mut counter = 0;

        for x in self.min_x..(self.max_x + 1) {
            for y in self.min_y..(self.max_y + 1) {
                let mut cell_dist = 0;
                for (px, py) in &self.points {
                    cell_dist += (x - px).abs() + (y - py).abs();
                }
                if cell_dist < MAX_DIST {
                    counter += 1;
                }
            }
        }

        return Box::new(counter);
    }
}

use crate::Base;
use std::fmt::Display;

pub struct Day25 {
    og_points: Vec<[isize; 4]>,
}
const MINDIST: isize = 3;

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { og_points: Vec::new() };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.og_points.push(
                line.split(',')
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<isize>>()
                    .try_into()
                    .unwrap(),
            )
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut points = self.og_points.clone();

        let mut total = 0;
        while !points.is_empty() {
            let mut constellation = Vec::new();
            constellation.push(points.pop().unwrap());

            let mut changes = true;
            while changes {
                changes = false;
                for i in 0..constellation.len() {
                    let mut ii = 0;
                    while ii < points.len() {
                        if manhattan(&points[ii], &constellation[i]) <= MINDIST {
                            constellation.push(points[ii]);
                            points.remove(ii);
                            changes = true;
                            continue;
                        }
                        ii += 1;
                    }
                }
            }
            total += 1;
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("*");
    }
}

fn manhattan(a: &[isize; 4], b: &[isize; 4]) -> isize {
    return (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs() + (a[3] - b[3]).abs();
}

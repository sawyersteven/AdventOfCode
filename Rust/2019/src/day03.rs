use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day03 {
    w1: String,
    w2: String,
    w1_path: Vec<Vector2Int>,
    w2_path: Vec<Vector2Int>,
    intersections: Vec<Vector2Int>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 {
            w1: String::new(),
            w2: String::new(),
            w1_path: Vec::new(),
            w2_path: Vec::new(),
            intersections: Vec::new(),
        };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.w1 = raw_input.split('\n').nth(0).unwrap().to_string();
        self.w2 = raw_input.split('\n').nth(1).unwrap().to_string();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.w1_path = trace_wire(&self.w1);
        self.w2_path = trace_wire(&self.w2);

        let mut min_dist = isize::MAX;
        let mut set: HashSet<Vector2Int> = HashSet::from_iter(self.w1_path.clone());

        for element in &self.w2_path {
            if set.remove(&element) {
                let d = element.x.abs() + element.y.abs();
                min_dist = min_dist.min(d);
                self.intersections.push(*element);
            }
        }
        return Box::new(min_dist);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut min_dist = usize::MAX;
        for intersection in &self.intersections {
            let d = self.w1_path.iter().position(|x| x == intersection).unwrap()
                + self.w2_path.iter().position(|x| x == intersection).unwrap()
                + 2;
            min_dist = min_dist.min(d);
        }
        return Box::new(min_dist);
    }
}

fn trace_wire(path: &String) -> Vec<Vector2Int> {
    let mut points = Vec::new();

    let mut current = Vector2Int::ZERO;
    for inst in path.split(',') {
        let dir = inst.as_bytes()[0];
        let dist = inst[1..].parse().unwrap();
        match dir {
            b'R' => {
                for _ in 0..dist {
                    current.y += 1;
                    points.push(current);
                }
            }
            b'L' => {
                for _ in 0..dist {
                    current.y -= 1;
                    points.push(current);
                }
            }
            b'U' => {
                for _ in 0..dist {
                    current.x += 1;
                    points.push(current);
                }
            }
            b'D' => {
                for _ in 0..dist {
                    current.x -= 1;
                    points.push(current);
                }
            }
            _ => panic!(),
        }
    }

    return points;
}

use shared::v2i::Vector2Int;

use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day20 {
    dirs: HashMap<u8, Vector2Int>,
    distances: HashMap<Vector2Int, usize>,
    instructions: Vec<u8>,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 {
            dirs: HashMap::new(),
            distances: HashMap::new(),
            instructions: Vec::new(),
        };
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.instructions = raw_input[1..(raw_input.len() - 1)].bytes().collect();
        self.dirs.insert(b'N', Vector2Int::DOWN);
        self.dirs.insert(b'E', Vector2Int::RIGHT);
        self.dirs.insert(b'S', Vector2Int::UP);
        self.dirs.insert(b'W', Vector2Int::LEFT);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut dist = 0;
        let mut forks = VecDeque::new();
        let mut pos = Vector2Int::ZERO;

        for b in &self.instructions {
            match b {
                b'(' => {
                    forks.push_back((dist, pos));
                }
                b')' => {
                    (dist, pos) = forks.pop_back().unwrap();
                }
                b'|' => {
                    (dist, pos) = *forks.back().unwrap();
                }
                _ => {
                    dist += 1;
                    pos += self.dirs[b];
                    if !self.distances.contains_key(&pos) {
                        self.distances.insert(pos, dist);
                    } else if dist < self.distances[&pos] {
                        self.distances.insert(pos, dist);
                    }
                }
            }
        }

        return Box::new(*self.distances.values().max().unwrap());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut counter = 0;
        for v in self.distances.values() {
            if *v >= 1000 {
                counter += 1;
            }
        }

        return Box::new(counter);
    }
}

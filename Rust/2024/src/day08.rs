use shared::v2i::Vector2Int;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day08 {
    antennae: HashMap<u8, Vec<Vector2Int>>,
    map_size: Vector2Int,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 {
            antennae: HashMap::new(),
            map_size: Vector2Int::ZERO,
        };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.lines().collect();
        self.map_size.y = lines.len() as isize - 1;
        self.map_size.x = lines[0].len() as isize - 1;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.bytes().enumerate() {
                if c == b'.' {
                    continue;
                }
                if !self.antennae.contains_key(&c) {
                    self.antennae.insert(c, Vec::new());
                }
                self.antennae
                    .get_mut(&c)
                    .unwrap()
                    .push(Vector2Int::new(x as isize, y as isize));
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut nodes = HashSet::new();

        for freq in self.antennae.values() {
            for i in 0..(freq.len() - 1) {
                for j in (i + 1)..freq.len() {
                    let n = freq[i] + freq[i] - freq[j];
                    if n.in_range(&Vector2Int::ZERO, &self.map_size) {
                        nodes.insert(n);
                    }
                    let n = freq[j] - freq[i] + freq[j];
                    if n.in_range(&Vector2Int::ZERO, &self.map_size) {
                        nodes.insert(n);
                    }
                }
            }
        }

        return Box::new(nodes.len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut nodes = HashSet::new();

        for freq in self.antennae.values() {
            for i in 0..(freq.len() - 1) {
                for j in (i + 1)..freq.len() {
                    let delta = freq[i] - freq[j];
                    let mut n = freq[i] - delta;
                    while n.in_range(&Vector2Int::ZERO, &self.map_size) {
                        nodes.insert(n);
                        n -= delta;
                    }
                    let mut n = freq[j] + delta;
                    while n.in_range(&Vector2Int::ZERO, &self.map_size) {
                        nodes.insert(n);
                        n += delta;
                    }
                }
            }
        }

        return Box::new(nodes.len());
    }
}

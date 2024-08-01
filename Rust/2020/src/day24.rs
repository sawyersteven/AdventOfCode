use shared::v2i::Vector2Int;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day24 {
    directions: Vec<Vec<Vector2Int>>,
    dir_map: HashMap<String, Vector2Int>,
    black_tiles: HashSet<Vector2Int>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            directions: Vec::new(),
            dir_map: HashMap::from([
                ("se".to_string(), Vector2Int::DOWN),
                ("nw".to_string(), Vector2Int::UP),
                ("ne".to_string(), Vector2Int::UR),
                ("w".to_string(), Vector2Int::LEFT),
                ("e".to_string(), Vector2Int::RIGHT),
                ("sw".to_string(), Vector2Int::DL),
            ]),
            black_tiles: HashSet::new(),
        };
    }

    fn neighbors(&self, loc: &Vector2Int) -> [Vector2Int; 6] {
        return [
            *loc + Vector2Int::UP,
            *loc + Vector2Int::UR,
            *loc + Vector2Int::RIGHT,
            *loc + Vector2Int::DOWN,
            *loc + Vector2Int::DL,
            *loc + Vector2Int::LEFT,
        ];
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let mut d = Vec::new();
            let mut i = 0;
            while i < line.len() {
                let c = line.as_bytes()[i];
                if c == b's' || c == b'n' {
                    d.push(self.dir_map[&line[i..(i + 2)]]);
                    i += 1;
                } else {
                    d.push(self.dir_map[&line[i..(i + 1)]]);
                }
                i += 1;
            }
            self.directions.push(d);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        for dirs in &self.directions {
            let mut loc = Vector2Int::ZERO;
            for d in dirs {
                loc += *d;
            }
            if self.black_tiles.contains(&loc) {
                self.black_tiles.remove(&loc);
            } else {
                self.black_tiles.insert(loc);
            }
        }
        return Box::new(self.black_tiles.len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const SIM_DAYS: usize = 100;

        let mut black_neighbor_counts = HashMap::new();
        let mut changes = HashSet::new();

        for _ in 0..SIM_DAYS {
            for bt in &self.black_tiles {
                if !black_neighbor_counts.contains_key(bt) {
                    black_neighbor_counts.insert(*bt, 0);
                }
                for n in self.neighbors(&bt) {
                    let c = *black_neighbor_counts.get(&n).unwrap_or(&0);
                    black_neighbor_counts.insert(n, c + 1);
                }
            }
            for (k, v) in black_neighbor_counts.drain() {
                if self.black_tiles.contains(&k) {
                    if v == 0 || v > 2 {
                        changes.insert(k);
                    }
                } else {
                    if v == 2 {
                        changes.insert(k);
                    }
                }
            }

            self.black_tiles = self.black_tiles.symmetric_difference(&changes).cloned().collect();
            changes.clear();
        }

        return Box::new(self.black_tiles.len());
    }
}

use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day25 {
    rights: HashSet<Vector2Int>,
    downs: HashSet<Vector2Int>,
    map_size: Vector2Int,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 {
            rights: HashSet::new(),
            downs: HashSet::new(),
            map_size: Vector2Int::ZERO,
        };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        for (y, line) in raw_input.split('\n').enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '>' => {
                        self.rights.insert(Vector2Int::new(x as isize, y as isize));
                    }
                    'v' => {
                        self.downs.insert(Vector2Int::new(x as isize, y as isize));
                    }
                    _ => {}
                }
                self.map_size.x = x as isize;
            }
            self.map_size.y = y as isize;
        }
        self.map_size += Vector2Int::UR;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut rights = self.rights.clone();
        let mut downs = self.downs.clone();

        let mut moves = HashSet::<Vector2Int>::new();

        for i in 0..usize::MAX {
            let mut move_count = 0;

            for right in &rights {
                let mut next = *right + Vector2Int::RIGHT;
                next.x %= self.map_size.x;
                if !rights.contains(&next) && !downs.contains(&next) {
                    moves.insert(*right);
                }
            }
            move_count += moves.len();
            for mut m in moves.drain() {
                rights.remove(&m);
                m.x = (m.x + 1) % self.map_size.x;
                rights.insert(m);
            }

            for down in &downs {
                let mut next = *down + Vector2Int::UP;
                next.y %= self.map_size.y;
                if !rights.contains(&next) && !downs.contains(&next) {
                    moves.insert(*down);
                }
            }

            move_count += moves.len();
            for mut m in moves.drain() {
                downs.remove(&m);
                m.y = (m.y + 1) % self.map_size.y;
                downs.insert(m);
            }

            if move_count == 0 {
                return Box::new(i + 1);
            }
        }

        return Box::new('-');
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

impl Day25 {
    #[allow(unused)]
    fn print_map(&self, rights: &HashSet<Vector2Int>, downs: &HashSet<Vector2Int>) {
        for y in 0..self.map_size.y {
            let mut row = Vec::<char>::new();
            for x in 0..self.map_size.x {
                if rights.contains(&Vector2Int::new(x, y)) {
                    row.push('>');
                } else if downs.contains(&Vector2Int::new(x, y)) {
                    row.push('v');
                } else {
                    row.push('.');
                }
            }
            println!("{}", String::from_iter(row));
        }
    }
}

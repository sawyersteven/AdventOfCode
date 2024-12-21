use shared::{bitset::Bitset2D, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

pub struct Day20 {
    input: String,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { input: String::new() };
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut start = Vector2Int::ZERO;
        let mut end = Vector2Int::ZERO;
        let sz = (self.input.len() as f64).sqrt() as usize;

        let mut walls = Bitset2D::new(sz, sz);
        for (y, line) in self.input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                match b {
                    b'S' => start = Vector2Int::new(x as isize, y as isize),
                    b'E' => end = Vector2Int::new(x as isize, y as isize),
                    b'#' => walls.set(x, y),
                    _ => {}
                }
            }
        }

        let mut path = Vec::with_capacity((sz * sz) / 2);
        path.push(start);
        let mut current = start.clone();
        let mut last = current;
        while current != end {
            for n in Vector2Int::CARDINALS {
                let next = current + n;
                if walls.is_setv(next) || next == last {
                    continue;
                }
                last = current;
                current = next;
                path.push(current);
            }
        }

        let mut count = 0;
        for a in 0..(path.len() - 102) {
            for b in (a + 102)..(path.len()) {
                if path[a].manhattan(&path[b]) != 2 {
                    continue;
                }
                count += 1;
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut start = Vector2Int::ZERO;
        let mut end = Vector2Int::ZERO;
        let sz = (self.input.len() as f64).sqrt() as usize;

        let mut walls = Bitset2D::new(sz, sz);
        for (y, line) in self.input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                match b {
                    b'S' => start = Vector2Int::new(x as isize, y as isize),
                    b'E' => end = Vector2Int::new(x as isize, y as isize),
                    b'#' => walls.set(x, y),
                    _ => {}
                }
            }
        }

        let mut path = Vec::with_capacity((sz * sz) / 2);
        path.push(start);
        let mut current = start.clone();
        let mut last = current;
        while current != end {
            for n in Vector2Int::CARDINALS {
                let next = current + n;
                if walls.is_setv(next) || next == last {
                    continue;
                }
                last = current;
                current = next;
                path.push(current);
            }
        }

        let mut count = 0;
        for a in 0..(path.len() - 102) {
            for b in (a + 102)..(path.len()) {
                let md = path[a].manhattan(&path[b]) as usize;
                if md > 20 || md < 2 {
                    continue;
                }
                let d = b - a - md;
                if d < 100 {
                    continue;
                }
                count += 1;
            }
        }

        return Box::new(count);
    }
}

trait B2DV {
    fn is_setv(&self, v: Vector2Int) -> bool;
}

impl B2DV for Bitset2D {
    fn is_setv(&self, v: Vector2Int) -> bool {
        return self.is_set(v.x as usize, v.y as usize);
    }
}

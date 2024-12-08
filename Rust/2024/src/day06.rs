use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day06 {
    guard: Vector2Int,
    walls: HashSet<Vector2Int>,
    bounds: Vector2Int,
    orig_path: HashSet<Vector2Int>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 {
            guard: Vector2Int::ZERO,
            walls: HashSet::new(),
            bounds: Vector2Int::ZERO,
            orig_path: HashSet::new(),
        };
    }

    fn is_loop(&self, obstruction: Vector2Int) -> bool {
        let mut dir = 2;
        let mut cur = self.guard.clone();
        let mut path = HashSet::new();

        loop {
            while !self.walls.contains(&cur) && cur != obstruction {
                cur += Vector2Int::CARDINALS[dir];
                if !cur.in_range(&Vector2Int::ZERO, &self.bounds) {
                    return false;
                }
            }
            cur -= Vector2Int::CARDINALS[dir];

            dir = (dir - 1) % 4;

            if !path.insert((cur, dir)) {
                return true;
            }
        }
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        for (y, line) in raw_input.lines().enumerate() {
            for (x, c) in line.as_bytes().iter().enumerate() {
                match c {
                    b'#' => {
                        self.walls.insert(Vector2Int::new(x as isize, y as isize));
                    }
                    b'^' => {
                        self.guard = Vector2Int::new(x as isize, y as isize);
                    }
                    _ => {}
                }
                self.bounds.x = x as isize;
            }
            self.bounds.y = y as isize;
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut dir = 2;
        let mut cur = self.guard.clone();
        self.orig_path.insert(cur.clone());

        loop {
            if self.walls.contains(&(cur + Vector2Int::CARDINALS[dir])) {
                dir -= 1;
                dir %= 4;
                continue;
            }

            cur += Vector2Int::CARDINALS[dir];
            if !cur.in_range(&Vector2Int::ZERO, &self.bounds) {
                break;
            }
            self.orig_path.insert(cur);
        }

        return Box::new(self.orig_path.len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        self.orig_path.remove(&self.guard);

        for p in &self.orig_path {
            if self.is_loop(*p) {
                t += 1;
            }
        }

        return Box::new(t);
    }
}

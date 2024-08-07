use shared::{v2i::Vector2Int, v3i::Vector3Int};

use crate::Base;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

pub struct Day18 {
    walls: HashSet<Vector2Int>,
    doors: HashMap<Vector2Int, usize>,
    keys: HashMap<Vector2Int, usize>,
    all_keys: isize,
    start: Vector3Int,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 {
            walls: HashSet::new(),
            doors: HashMap::new(),
            keys: HashMap::new(),
            all_keys: 0,
            start: Vector3Int::ZERO,
        };
    }

    fn search(&self, start: &Vector3Int, owned_keys: isize) -> usize {
        let mut start = *start;
        let mut q = VecDeque::new();
        let mut visited = HashSet::new();
        start.z = owned_keys;
        q.push_back((start, 0));

        while let Some((current, steps)) = q.pop_front() {
            if current.z & self.all_keys == self.all_keys {
                return steps;
            }

            for v2 in Vector2Int::CARDINALS {
                let mut next = current + v2;
                let nv2 = next.v2();
                if self.walls.contains(&nv2) || visited.contains(&next) {
                    continue;
                }
                // check if next step is door and if we have the key
                match self.doors.get(&nv2) {
                    Some(&d) => {
                        if next.z as usize & d != d {
                            continue;
                        }
                    }
                    None => {}
                }

                visited.insert(next);

                // if picking up key
                match self.keys.get(&nv2) {
                    Some(k) => next.z |= *k as isize,
                    None => {}
                }

                q.push_back((next, steps + 1));
            }
        }

        unreachable!();
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                match b {
                    b'@' => self.start = Vector3Int::new(x as isize, y as isize, 0),
                    b'#' => {
                        self.walls.insert(Vector2Int::new(x as isize, y as isize));
                    }
                    b'.' => (),
                    &b => {
                        let mut k = 1;
                        if b < b'a' {
                            k <<= b - b'A';
                            self.doors.insert(Vector2Int::new(x as isize, y as isize), k);
                        } else {
                            k <<= b - b'a';
                            self.keys.insert(Vector2Int::new(x as isize, y as isize), k);
                        }
                        self.all_keys |= k as isize;
                    }
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let ans = self.search(&self.start, 0);
        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for v2 in Vector2Int::CARDINALS {
            self.walls.insert((self.start + v2).v2());
        }

        // top left
        let mut total = 0;
        let mut owned_keys = self.all_keys;
        for y in 0..self.start.y {
            for x in 0..self.start.x {
                match self.keys.get(&Vector2Int::new(x, y)) {
                    Some(k) => owned_keys ^= *k as isize,
                    None => {}
                }
            }
        }

        let mut start = self.start;
        start.x -= 1;
        start.y -= 1;
        total += self.search(&start, owned_keys);

        // top right
        owned_keys = self.all_keys;
        for y in 0..self.start.y {
            for x in (self.start.x + 1)..(self.start.x * 2) {
                match self.keys.get(&Vector2Int::new(x, y)) {
                    Some(k) => owned_keys ^= *k as isize,
                    None => {}
                }
            }
        }

        start = self.start;
        start.x += 1;
        start.y -= 1;
        total += self.search(&start, owned_keys);

        // bottom right
        owned_keys = self.all_keys;

        for y in (self.start.y + 1)..(self.start.y * 2) {
            for x in (self.start.x + 1)..(self.start.x * 2) {
                match self.keys.get(&Vector2Int::new(x, y)) {
                    Some(k) => owned_keys ^= *k as isize,
                    None => {}
                }
            }
        }

        start = self.start;
        start.x += 1;
        start.y += 1;
        total += self.search(&start, owned_keys);

        // bottom left
        owned_keys = self.all_keys;
        for y in (self.start.x + 1)..(self.start.y * 2) {
            for x in 0..self.start.x {
                match self.keys.get(&Vector2Int::new(x, y)) {
                    Some(k) => owned_keys ^= *k as isize,
                    None => {}
                }
            }
        }

        start = self.start;
        start.x -= 1;
        start.y += 1;
        total += self.search(&start, owned_keys);

        return Box::new(total);
    }
}

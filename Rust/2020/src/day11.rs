use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const FLOOR: u8 = b'.';
const EMPTY: u8 = b'L';
const OCCUPIED: u8 = b'#';

pub struct Day11 {
    map: Grid2D<u8>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 { map: Grid2D::new(0, 0) };
    }

    fn can_fill_seat(&self, map: &Grid2D<u8>, neighbors: &Vec<Vector2Int>) -> bool {
        for n in neighbors {
            if map[*n] == OCCUPIED {
                return false;
            }
        }
        return true;
    }

    fn can_empty_seat(&self, map: &Grid2D<u8>, limit: usize, neighbors: &Vec<Vector2Int>) -> bool {
        let mut count = 0;
        for n in neighbors {
            if map[*n] == OCCUPIED {
                count += 1;
            }
        }
        return count >= limit;
    }

    fn adj_map(&self, size: &Vector2Int) -> HashMap<Vector2Int, Vec<Vector2Int>> {
        let mut map = HashMap::new();
        let size = *size - Vector2Int::ONE;
        for r in 0..=size.y {
            for c in 0..=size.x {
                let mut adj = Vec::new();
                for n in Vector2Int::ALL_DIRS.iter().map(|a| *a + Vector2Int::new(c, r)) {
                    if n.in_range(&Vector2Int::ZERO, &size) {
                        adj.push(n);
                    }
                }
                map.insert(Vector2Int::new(c, r), adj);
            }
        }
        return map;
    }

    fn adj_map_2(&self, map: &Grid2D<u8>) -> HashMap<Vector2Int, Vec<Vector2Int>> {
        let mut adj_map = HashMap::new();
        let size = map.size().to_vector2();

        for r in 0..=size.y {
            for c in 0..=size.x {
                let mut adj = Vec::new();
                for dir in Vector2Int::ALL_DIRS {
                    let mut dist = 0;
                    loop {
                        dist += 1;
                        let mut next = dir * dist;
                        next.x += c;
                        next.y += r;
                        if !next.in_range(&Vector2Int::ZERO, &(size - Vector2Int::ONE)) {
                            break;
                        }

                        if map[next] != FLOOR {
                            adj.push(next);
                            break;
                        }
                    }
                }
                adj_map.insert(Vector2Int::new(c, r), adj);
            }
        }

        return adj_map;
    }

    fn run_sim(&self, map: &mut Grid2D<u8>, adj_map: &HashMap<Vector2Int, Vec<Vector2Int>>, limit: usize) {
        let sz = map.size().to_vector2();
        let mut changes = HashSet::new();

        loop {
            let mut current = Vector2Int::ZERO;
            for row in 0..sz.y {
                current.y = row;
                for col in 0..sz.x {
                    current.x = col;
                    match map[current] {
                        EMPTY => {
                            if self.can_fill_seat(&map, &adj_map[&current]) {
                                changes.insert(current.clone());
                            }
                        }
                        OCCUPIED => {
                            if self.can_empty_seat(&map, limit, &adj_map[&current]) {
                                changes.insert(current.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }

            if changes.len() == 0 {
                return;
            }

            for ch in changes.drain() {
                let og = map[ch];
                map[ch] = if og == OCCUPIED { EMPTY } else { OCCUPIED };
            }
        }
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut map = self.map.clone();
        let sz = map.size().to_vector2();
        let adj_map = self.adj_map(&sz);

        self.run_sim(&mut map, &adj_map, 4);

        let count = map.count(|x| *x == OCCUPIED);
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut map = self.map.clone();
        let adj_map = self.adj_map_2(&map);

        self.run_sim(&mut map, &adj_map, 5);

        let count = map.count(|x| *x == OCCUPIED);
        return Box::new(count);
    }
}

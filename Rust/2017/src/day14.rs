use shared::v2i::Vector2Int;

use crate::day10::KnotHash;
use crate::utils::vec2d;
use crate::Base;
use std::collections::{HashSet, VecDeque};
use std::fmt::Display;

static mut DENSE_HASHES: Vec<Vec<u8>> = Vec::new();

pub struct Day14 {
    pub input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 {
            input: String::from(""),
        };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut dense_hashes = Vec::<Vec<u8>>::with_capacity(128);

        let mut total_bits = 0;
        for i in 0..128 {
            let dense_hash = KnotHash::new(&format!("{}-{}", self.input, i)).dense_hash();
            for b in &dense_hash {
                total_bits += bits_in_byte(&b);
            }
            dense_hashes.push(dense_hash);
        }

        unsafe {
            DENSE_HASHES = dense_hashes;
        }
        return Box::new(total_bits);
    }

    fn part2(&self) -> Box<dyn Display> {
        let dense_hashes: Vec<Vec<u8>>;
        unsafe {
            dense_hashes = DENSE_HASHES.clone();
        }

        let mut grid = vec2d(false, 128, 128);

        for y in 0..128 {
            let row_bytes = &dense_hashes[y];
            let mut x = 0;
            for i in 0..row_bytes.len() {
                let mut b = row_bytes[i];
                for _ in 0..8 {
                    grid[y][x] = (b & 0b1000_0000) != 0;
                    b <<= 1;
                    x += 1;
                }
            }
        }

        let mut region_count = 0;
        let mut visited = HashSet::<Vector2Int>::new();
        let mut start = Vector2Int::new(0, 0);

        for y in 0..128 {
            start.y = y;
            for x in 0..128 {
                start.x = x;

                if visited.contains(&start) || grid[start.y as usize][start.x as usize] == false {
                    continue;
                }

                visited.extend(find_region(&grid, &start));
                region_count += 1;
            }
        }

        return Box::new(region_count);
    }
}

fn find_region(grid: &Vec<Vec<bool>>, start: &Vector2Int) -> HashSet<Vector2Int> {
    let mut visited = HashSet::<Vector2Int>::new();
    let mut q = VecDeque::<Vector2Int>::new();
    q.push_back(start.clone());

    while q.len() > 0 {
        let current = q.pop_back().unwrap();
        visited.insert(current.clone());

        for v in Vector2Int::CARDINALS {
            let next = current + v;
            if next.x < 0 || next.x > 127 || next.y < 0 || next.y > 127 {
                continue;
            }
            if visited.contains(&next) || grid[next.y as usize][next.x as usize] == false {
                continue;
            }
            q.push_front(next);
        }
    }
    return visited;
}

fn bits_in_byte(b: &u8) -> usize {
    let mut bits: usize = 0;
    let mut b = *b;
    while b != 0x0 {
        bits += (b & 1) as usize;
        b >>= 1;
    }
    return bits;
}

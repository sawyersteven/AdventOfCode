use shared::{v2i::Vector2Int, v3i::Vector3Int};

use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

const MAGIC: usize = 1350;
const WALL: u8 = 1;
const EMPTY: u8 = 0;
const GOAL: Vector2Int = Vector2Int { x: 31, y: 39 };
const START: Vector3Int = Vector3Int { x: 1, y: 1, z: 0 };

pub struct Day13 {
    pub grid: Vec<Vec<u8>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 { grid: Vec::new() };
    }
}

impl Base for Day13 {
    #[allow(unused)]
    fn parse_input(&mut self, raw_input: String) {
        self.grid = make_grid(GOAL.x as usize * 2, GOAL.y as usize * 2);
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<Vector3Int>::new();
        let mut visited = HashSet::<Vector2Int>::new();

        let h = self.grid.len() as isize;
        let w = self.grid[0].len() as isize;

        q.push_back(START.clone());

        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            if current.x == GOAL.x && current.y == GOAL.y {
                return Box::new(current.z);
            }

            for step in Vector2Int::CARDINALS {
                let mut next = current + step;
                let next2 = next.v2();
                if visited.contains(&next2) || next.x < 0 || next.y < 0 || next.x >= w || next.y >= h {
                    continue;
                }
                if self.grid[next.y as usize][next.x as usize] == WALL {
                    continue;
                }
                next.z += 1;
                visited.insert(next2);
                q.push_back(next);
            }
        }

        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<Vector3Int>::new();
        let mut visited = HashSet::<Vector2Int>::new();

        let h = self.grid.len() as isize;
        let w = self.grid[0].len() as isize;

        const LIMIT: isize = 50;
        let mut tile_count = 0;

        q.push_back(START.clone());

        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            if current.z == LIMIT {
                continue;
            }

            for step in Vector2Int::CARDINALS {
                let mut next = current + step;
                let next2 = next.v2();
                if visited.contains(&next2) || next.x < 0 || next.y < 0 || next.x >= w || next.y >= h {
                    continue;
                }
                if self.grid[next.y as usize][next.x as usize] == WALL {
                    continue;
                }
                next.z += 1;
                visited.insert(next2);
                q.push_back(next);
                tile_count += 1;
            }
        }
        return Box::new(tile_count);
    }
}

fn make_grid(w: usize, h: usize) -> Vec<Vec<u8>> {
    let mut grid = Vec::<Vec<u8>>::new();
    for y in 0..h {
        let mut row = Vec::<u8>::new();
        for x in 0..w {
            row.push(if is_wall(x, y) { WALL } else { EMPTY });
        }
        grid.push(row);
    }
    return grid;
}

fn is_wall(x: usize, y: usize) -> bool {
    let mut z = x * x + 3 * x + 2 * x * y + y + y * y + MAGIC;
    let mut bitcount = 0;
    while z != 0 {
        bitcount += z & 1;
        z >>= 1;
    }
    return bitcount % 2 == 1;
}

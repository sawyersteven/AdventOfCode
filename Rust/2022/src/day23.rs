use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

const TURNS: usize = 10;

// encode neighbors as u8 bitmask where directions are
// S | SE | E | NE | N | NW | W | SW
const MASK_N: u8 = 0b00011100;
const MASK_S: u8 = 0b11000001;
const MASK_E: u8 = 0b01110000;
const MASK_W: u8 = 0b00000111;

pub struct Day23 {
    pub elves: HashSet<Vector2Int>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { elves: HashSet::new() };
    }

    fn check_moves(neighbors: u8, turn: usize) -> Option<Vector2Int> {
        const PAIRS: [(u8, Vector2Int); 4] = [
            (MASK_N, Vector2Int::DOWN),
            (MASK_S, Vector2Int::UP),
            (MASK_W, Vector2Int::LEFT),
            (MASK_E, Vector2Int::RIGHT),
        ];
        for mut i in turn..(turn + 4) {
            i = i % 4;
            if neighbors & PAIRS[i].0 == 0 {
                return Some(PAIRS[i].1);
            }
        }
        return None;
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        for (y, line) in raw_input.lines().enumerate() {
            for (x, cell) in line.chars().enumerate() {
                if cell == '#' {
                    self.elves.insert(Vector2Int::new(x as isize, y as isize));
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut elves = self.elves.clone();
        let mut proposed_moves = Vec::with_capacity(elves.len());
        let mut collisions: HashSet<Vector2Int> = HashSet::new();

        for turn in 0..TURNS {
            proposed_moves.clear();
            collisions.clear();

            for elf in &elves {
                let mut neighbors: u8 = 0;
                for d in Vector2Int::ALL_DIRS {
                    neighbors <<= 1;
                    if elves.contains(&(*elf + d)) {
                        neighbors |= 1;
                    }
                }

                if neighbors != 0 {
                    match Day23::check_moves(neighbors, turn) {
                        Some(m) => {
                            let d = *elf + m;
                            if proposed_moves.iter().any(|(_, dest)| *dest == d) {
                                collisions.insert(d);
                            }
                            proposed_moves.push((*elf, d));
                        }
                        None => {}
                    }
                }
            }

            for (cur, dst) in proposed_moves.iter().filter(|(_, d)| !collisions.contains(&d)) {
                elves.remove(&cur);
                elves.insert(*dst);
            }
        }

        let mut xmin = isize::MAX;
        let mut xmax = isize::MIN;
        let mut ymin = isize::MAX;
        let mut ymax = isize::MIN;
        for e in &elves {
            xmin = xmin.min(e.x);
            xmax = xmax.max(e.x + 1);
            ymin = ymin.min(e.y);
            ymax = ymax.max(e.y + 1);
        }

        let rect_area = ((xmax - xmin).abs() * (ymax - ymin).abs()) as usize;
        let ans = rect_area - elves.len();

        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut elves = self.elves.clone();
        let mut proposed_moves = Vec::with_capacity(elves.len());
        let mut collisions: HashSet<Vector2Int> = HashSet::new();

        for turn in 0..usize::MAX {
            proposed_moves.clear();
            collisions.clear();

            for elf in &elves {
                let mut neighbors: u8 = 0;
                for d in Vector2Int::ALL_DIRS {
                    neighbors <<= 1;
                    if elves.contains(&(*elf + d)) {
                        neighbors |= 1;
                    }
                }

                if neighbors != 0 {
                    match Day23::check_moves(neighbors, turn) {
                        Some(m) => {
                            let d = *elf + m;
                            if proposed_moves.iter().any(|(_, dest)| *dest == d) {
                                collisions.insert(d);
                            }
                            proposed_moves.push((*elf, d));
                        }
                        None => {}
                    }
                }
            }

            let mut done = true;
            for (cur, dst) in proposed_moves.iter().filter(|(_, d)| !collisions.contains(&d)) {
                elves.remove(&cur);
                elves.insert(*dst);
                done = false;
            }
            if done {
                return Box::new(turn + 1);
            }
        }
        return Box::new("?");
    }
}

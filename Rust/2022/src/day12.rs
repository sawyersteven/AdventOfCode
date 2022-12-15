use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::VecDeque, fmt::Display};

#[derive(Debug)]
struct State {
    coord: Vector2Int,
    cost: usize,
    elevation: u8,
}

pub struct Day12 {
    map: Vec<Vec<u8>>,
    start: Vector2Int,
    end: Vector2Int,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            map: Vec::new(),
            start: Vector2Int::ZERO,
            end: Vector2Int::ZERO,
        };
    }

    fn find_path<F>(&self, goal_check: F) -> usize
    where
        F: Fn(&State) -> bool,
    {
        let mut map = self.map.clone();
        let mut q = VecDeque::<State>::new();
        q.push_back(State {
            coord: self.end,
            cost: 0,
            elevation: b'z',
        });

        let map_size = Vector2Int::new(map[0].len() as isize, map.len() as isize);

        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            if goal_check(&current) {
                return current.cost;
            }

            for dir in Vector2Int::CARDINALS {
                let next = current.coord + dir;
                if next.x < 0 || next.y < 0 || next.x == map_size.x || next.y == map_size.y {
                    continue;
                }

                let next_elevation = map[next.y as usize][next.x as usize];
                if next_elevation == b'.' || next_elevation < current.elevation - 1 {
                    continue;
                }

                map[next.y as usize][next.x as usize] = b'.';
                q.push_back(State {
                    coord: next,
                    cost: current.cost + 1,
                    elevation: next_elevation,
                });
            }
        }
        return usize::MAX;
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.map.push(line.bytes().collect());
        }
        for y in 0..self.map.len() {
            for x in 0..self.map[0].len() {
                if self.map[y][x] == b'S' {
                    self.start = Vector2Int::new(x as isize, y as isize);
                    self.map[y][x] = b'a';
                } else if self.map[y][x] == b'E' {
                    self.end = Vector2Int::new(x as isize, y as isize);
                    self.map[y][x] = b'z';
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.find_path(|a| a.coord == self.start));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.find_path(|a| a.elevation == b'a'));
    }
}

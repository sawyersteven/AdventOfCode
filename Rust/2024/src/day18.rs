use shared::{bitset::Bitset2D, v2i::Vector2Int};

use crate::Base;
use std::{collections::VecDeque, fmt::Display};

const SZ: usize = 70;
const END: Vector2Int = Vector2Int::new(SZ as isize, SZ as isize);
const START: Vector2Int = Vector2Int::ZERO;

pub struct Day18 {
    input: String,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { input: String::new() };
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const WALL_COUNT: usize = 1024;
        let mut walls = Bitset2D::new(SZ + 1, SZ + 1);
        for (i, line) in self.input.lines().enumerate() {
            if i == WALL_COUNT {
                break;
            }
            let (x, y) = line.split_once(',').unwrap();
            walls.set(x.parse().unwrap(), y.parse().unwrap());
        }

        let mut stack = VecDeque::new();
        let mut visited = Bitset2D::new(SZ + 1, SZ + 1);
        stack.push_back((START, 0));

        while let Some((c_pos, c_steps)) = stack.pop_back() {
            if c_pos == END {
                return Box::new(c_steps);
            }

            for n in Vector2Int::CARDINALS {
                let n_pos = c_pos + n;
                if walls.is_set(n_pos.x as usize, n_pos.y as usize) {
                    continue;
                }
                if visited.is_set(n_pos.x as usize, n_pos.y as usize)
                    || n_pos.x < 0
                    || n_pos.y < 0
                    || n_pos.x > SZ as isize
                    || n_pos.y > SZ as isize
                {
                    continue;
                }
                visited.set(n_pos.x as usize, n_pos.y as usize);
                stack.push_front((n_pos, c_steps + 1));
            }
        }

        return Box::new("-"); // 1544 too high
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const WALL_COUNT: usize = 1024;
        let mut walls = Bitset2D::new(SZ + 1, SZ + 1);
        for (i, line) in self.input.lines().enumerate() {
            if i == WALL_COUNT {
                break;
            }
            let (x, y) = line.split_once(',').unwrap();
            walls.set(x.parse().unwrap(), y.parse().unwrap());
        }

        let mut path = can_reach_end(&walls).unwrap();

        for line in self.input.lines().skip(1024) {
            let (x, y) = line.split_once(',').unwrap();
            let (x, y) = (x.parse().unwrap(), y.parse().unwrap());
            walls.set(x, y);
            // new block lands on path, try for new path
            if path.is_set(x, y) {
                match can_reach_end(&walls) {
                    Some(p) => path = p,
                    None => return Box::new(format!("{},{}", x, y)),
                }
            }
        }

        return Box::new("-");
    }
}

fn can_reach_end(walls: &Bitset2D) -> Option<Bitset2D> {
    let mut stack = VecDeque::new();
    let mut visited = Bitset2D::new(SZ + 1, SZ + 1);
    let mut start_path = Bitset2D::new(SZ + 1, SZ + 1);
    start_path.set(0, 0);
    stack.push_back((START, start_path));

    while let Some((c_pos, path)) = stack.pop_back() {
        if c_pos == END {
            return Some(path);
        }

        for n in Vector2Int::CARDINALS {
            let n_pos = c_pos + n;
            if walls.is_set(n_pos.x as usize, n_pos.y as usize) {
                continue;
            }
            if visited.is_set(n_pos.x as usize, n_pos.y as usize)
                || n_pos.x < 0
                || n_pos.y < 0
                || n_pos.x > SZ as isize
                || n_pos.y > SZ as isize
            {
                continue;
            }
            visited.set(n_pos.x as usize, n_pos.y as usize);
            let mut n_path = path.clone();
            n_path.set(n_pos.x as usize, n_pos.y as usize);
            stack.push_back((n_pos, n_path));
        }
    }
    return None;
}

use shared::{bitset::Bitset2D, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { input: String::new() };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let (map, path) = self.input.split_once("\n\n").unwrap();
        let sz = (map.len() as f64).sqrt() as isize;

        let mut walls = Bitset2D::new(sz as usize, sz as usize);
        let mut boxes = Bitset2D::new(sz as usize, sz as usize);
        let mut bot = Vector2Int::new(0, 0);

        for (y, line) in map.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => walls.set(x, y),
                    b'O' => boxes.set(x, y),
                    b'@' => bot = Vector2Int::new(x as isize, y as isize),
                    _ => {}
                }
            }
        }

        for m in path.bytes() {
            let dir = match m {
                b'^' => Vector2Int::new(0, -1),
                b'>' => Vector2Int::new(1, 0),
                b'v' => Vector2Int::new(0, 1),
                b'<' => Vector2Int::new(-1, 0),
                b'\n' => continue,
                _ => unreachable!(),
            };
            let move_to = bot + dir;
            if walls.is_set(move_to.x as usize, move_to.y as usize) {
                continue;
            }
            if !boxes.is_set(move_to.x as usize, move_to.y as usize) {
                bot = move_to;
                continue;
            }

            // find closest open space
            let mut gap_dist = 1;
            loop {
                let step = bot + (dir * gap_dist);
                if walls.is_set(step.x as usize, step.y as usize) {
                    gap_dist = 0;
                    break;
                }
                if !boxes.is_set(step.x as usize, step.y as usize) {
                    break;
                }
                gap_dist += 1;
            }
            if gap_dist == 0 {
                continue;
            }
            // swap first and last box
            let f = bot + (dir * gap_dist);
            boxes.set(f.x as usize, f.y as usize);
            boxes.unset(move_to.x as usize, move_to.y as usize);
            bot = move_to;
        }

        let mut sum = 0;
        for y in 1..(sz - 1) as usize {
            for x in 1..(sz - 1) as usize {
                if boxes.is_set(x, y) {
                    sum += (y * 100) + x;
                }
            }
        }

        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

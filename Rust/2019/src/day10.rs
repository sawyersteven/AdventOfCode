use shared::{utils::parse_to_u8_grid, v2i::Vector2Int};

use crate::{utils, Base};
use std::{collections::HashSet, fmt::Display};

pub struct Day10 {
    asteroids: Vec<Vector2Int>,
    station_loc: Vector2Int,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 {
            asteroids: Vec::new(),
            station_loc: Vector2Int::ZERO,
        };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        let grid = parse_to_u8_grid(raw_input, b' ', 0);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == b'#' {
                    self.asteroids.push(Vector2Int::new(x as isize, y as isize));
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut max_count = 0;
        let mut consumed_slopes = HashSet::new();

        for a in 0..self.asteroids.len() {
            for b in 0..self.asteroids.len() {
                if a == b {
                    continue;
                }
                let slope = self.asteroids[b] - self.asteroids[a];
                consumed_slopes.insert(slope / utils::gcd(slope.x, slope.y));
            }
            if consumed_slopes.len() > max_count {
                max_count = consumed_slopes.len();
                self.station_loc = self.asteroids[a];
            }
            consumed_slopes.clear();
        }

        return Box::new(max_count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut relative_positions = Vec::new();

        for other in &self.asteroids {
            if *other == self.station_loc {
                continue;
            }
            let dist = (other.x - self.station_loc.x).abs() + (other.y - self.station_loc.y).abs();
            let angle = (360f64 - (self.station_loc - *other).angle_to(&Vector2Int::UP)) % 360f64;
            relative_positions.push((angle, dist, other));
        }
        relative_positions.sort_by(|a, b| {
            if a.0 == b.0 {
                return a.1.cmp(&b.1);
            }
            return a.0.partial_cmp(&b.0).unwrap();
        });

        let mut removed = 0;
        let mut current = 0;
        loop {
            let angle = relative_positions[current].0;
            removed += 1;
            if removed == 200 {
                break;
            }
            relative_positions.remove(current);
            while relative_positions[current].0 == angle {
                current += 1;
                current %= relative_positions.len();
            }
        }

        let answer = relative_positions[current].2;
        return Box::new(answer.x * 100 + answer.y);
    }
}

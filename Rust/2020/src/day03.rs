use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    map: Grid2D<u8>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { map: Grid2D::new(0, 0) };
    }

    fn find_trees(&self, dir: Vector2Int) -> usize {
        let sz = self.map.size().to_vector2();
        let mut trees = 0;
        let mut pos = Vector2Int::ZERO;
        while pos.y < sz.y - 1 {
            pos += dir;
            if pos.x >= sz.x {
                pos.x %= sz.x;
            }
            if self.map[pos] == b'#' {
                trees += 1;
            }
        }
        return trees;
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.find_trees(Vector2Int { x: 3, y: 1 }));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = self.find_trees(Vector2Int::new(1, 1));
        t *= self.find_trees(Vector2Int::new(3, 1));
        t *= self.find_trees(Vector2Int::new(5, 1));
        t *= self.find_trees(Vector2Int::new(7, 1));
        t *= self.find_trees(Vector2Int::new(1, 2));

        return Box::new(t);
    }
}

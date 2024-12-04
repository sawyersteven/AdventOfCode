use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

const MS: u16 = ((b'M' as u16) << 8) + b'S' as u16;
const SM: u16 = ((b'S' as u16) << 8) + b'M' as u16;

pub struct Day04 {
    input: Grid2D<u8>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 {
            input: Grid2D::<u8>::new(0, 0),
        };
    }

    fn count_xmas(&self, x: usize, y: usize) -> usize {
        if self.input.get(x, y) != &b'X' {
            return 0;
        }
        let mut t = 0;
        let sz = self.input.size().to_vector2() - Vector2Int::ONE;
        let og = Vector2Int::new(x as isize, y as isize);
        for d in Vector2Int::ALL_DIRS {
            let m = og + d;
            if m.in_range(&Vector2Int::ZERO, &sz) && self.input[&m] == b'M' {
                let a = m + d;
                if a.in_range(&Vector2Int::ZERO, &sz) && self.input[&a] == b'A' {
                    let s = a + d;
                    if s.in_range(&Vector2Int::ZERO, &sz) && self.input[&s] == b'S' {
                        t += 1;
                    }
                }
            }
        }
        return t;
    }

    // doesn't check bounds
    fn is_x_mas(&self, x: usize, y: usize) -> bool {
        if self.input.get(x, y) != &b'A' {
            return false;
        }

        let d1 = ((*self.input.get(x + 1, y + 1) as u16) << 8) + *self.input.get(x - 1, y - 1) as u16;
        if d1 != MS && d1 != SM {
            return false;
        }

        let d2 = ((*self.input.get(x - 1, y + 1) as u16) << 8) + *self.input.get(x + 1, y - 1) as u16;
        if d2 != MS && d1 != SM {
            return false;
        }
        return true;
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = Grid2D::<u8>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        let sz = self.input.size();
        for x in 0..(sz.x) {
            for y in 0..(sz.y) {
                t += self.count_xmas(x, y);
            }
        }
        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        let sz = self.input.size();
        for x in 1..(sz.x - 1) {
            for y in 1..(sz.y - 1) {
                if self.is_x_mas(x, y) {
                    t += 1;
                };
            }
        }
        return Box::new(t);
    }
}

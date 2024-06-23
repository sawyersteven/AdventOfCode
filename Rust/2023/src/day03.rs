use shared::grid2d::Grid2D;

use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    grid: Grid2D<char>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 {
            grid: Grid2D::new(0, 0),
        };
    }

    // returns list of valid numbers and the coord of their matching symbol
    fn find_nums(&self, symbol_filter: impl Fn(&char) -> bool) -> Vec<(u32, (isize, isize))> {
        let mut nums = Vec::new();
        let gs = self.grid.size();

        for y in 0..gs.y {
            let mut x = 0;
            while x < gs.x {
                let start;
                if self.grid.get(x, y).is_digit(10) {
                    start = x;
                    while self.grid.get(x, y).is_digit(10) {
                        x += 1;
                    }
                    let mut symbol = None;
                    let mut num = 0;
                    for (i, dx) in (start..=(x - 1)).rev().enumerate() {
                        num += self.grid.get(dx, y).to_digit(10).unwrap() * 10_u32.pow(i as u32);
                        for neighbor in [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)] {
                            let p = (dx as isize + neighbor.0, y as isize + neighbor.1);
                            if p.0 < 0 || p.1 < 0 || p.0 == gs.x as isize || p.1 == gs.y as isize {
                                continue;
                            }
                            if symbol_filter(self.grid.get(p.0 as usize, p.1 as usize)) {
                                symbol = Some((p.0, p.1));
                                break;
                            }
                        }
                    }

                    match symbol {
                        Some(p) => nums.push((num, p)),
                        None => {}
                    }
                    x += 1;
                } else {
                    x += 1;
                }
            }
        }
        return nums;
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.grid = Grid2D::<char>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let sum: u32 = self
            .find_nums(|x| !x.is_digit(10) && *x != '.')
            .iter()
            .map(|(n, _)| n)
            .sum();

        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let nums = self.find_nums(|x| *x == '*');

        let mut sum = 0;
        for i in 0..(nums.len() - 1) {
            for j in (i + 1)..nums.len() {
                if nums[i].1 == nums[j].1 {
                    sum += nums[i].0 * nums[j].0;
                }
            }
        }
        return Box::new(sum);
    }
}

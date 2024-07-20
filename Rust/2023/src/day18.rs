use shared::v2i::Vector2Int;

use crate::Base;
use std::fmt::Display;

pub struct Day18 {
    input: String,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { input: String::new() };
    }

    // Shoestring + Pick's
    fn solve_area(&self, plans: &Vec<Plan>) -> isize {
        let mut walls_count = 0;
        let mut pos = Vector2Int::ZERO;

        let mut area = 0;
        for plan in plans {
            walls_count += plan.len;
            for _ in 0..plan.len {
                let npos = pos + plan.direction;
                area += pos.x * npos.y;
                area -= pos.y * npos.x;
                pos = npos;
            }
        }

        area = (area.abs() + 2 - (walls_count - 1) as isize) / 2;
        area += walls_count as isize;
        return area;
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut plans = Vec::new();
        for line in self.input.lines() {
            let parts = line.split(' ').collect::<Vec<&str>>();
            plans.push(Plan {
                direction: match parts[0] {
                    "U" => Vector2Int::DOWN,
                    "D" => Vector2Int::UP,
                    "R" => Vector2Int::RIGHT,
                    "L" => Vector2Int::LEFT,
                    _ => panic!(),
                },
                len: parts[1].parse().unwrap(),
            });
        }
        return Box::new(self.solve_area(&plans));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut plans = Vec::new();
        for line in self.input.lines() {
            let parts = line.split(' ').collect::<Vec<&str>>();
            plans.push(Plan {
                direction: match parts[2].as_bytes().iter().nth_back(1).unwrap() {
                    b'3' => Vector2Int::DOWN,
                    b'1' => Vector2Int::UP,
                    b'0' => Vector2Int::RIGHT,
                    b'2' => Vector2Int::LEFT,
                    _ => panic!(),
                },
                len: usize::from_str_radix(&parts[2][2..(parts[2].len() - 2)], 16).unwrap(),
            });
        }

        // This takes ~3 seconds but I'm ok enough with that
        return Box::new(self.solve_area(&plans)); // 97874103749720
    }
}

struct Plan {
    direction: Vector2Int,
    len: usize,
}

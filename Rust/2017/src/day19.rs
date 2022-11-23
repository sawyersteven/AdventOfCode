use shared::v2i::Vector2Int;

use crate::{utils::vec2d, Base};
use std::fmt::Display;

static mut STEPS: usize = 0;

pub struct Day19 {
    pub input: String,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            input: String::from(""),
        };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let lines: Vec<&str> = self.input.split('\n').collect();

        let mut grid = vec2d(' ', lines.len() + 2, lines[0].len() + 2);

        for y in 1..(grid.len() - 1) {
            let row_chars: Vec<char> = lines[y - 1].chars().collect();
            for x in 1..(grid[0].len() - 1) {
                grid[y][x] = row_chars[x - 1];
            }
        }

        let mut current = Vector2Int::new(1, 1);
        for i in 0..lines[0].len() {
            if grid[1][i] == '|' {
                current.x = i as isize;
                break;
            }
        }

        let mut collected = Vec::<char>::new();
        let mut current_dir = Vector2Int::up();
        let mut steps = 0;

        loop {
            let mut next = current + current_dir;
            while grid[next.y as usize][next.x as usize] != ' ' {
                let n = grid[next.y as usize][next.x as usize];

                if n >= 'A' && n <= 'Z' {
                    collected.push(n);
                }
                current = next;
                steps += 1;
                next = current + current_dir;
            }

            let mut next_dir = current_dir;
            for dir in Vector2Int::CARDINALS {
                if dir == current_dir * -1 {
                    continue;
                }
                next = current + dir;
                if grid[next.y as usize][next.x as usize] != ' ' {
                    next_dir = dir;
                    break;
                }
            }
            if next_dir == current_dir {
                break;
            }
            current_dir = next_dir;
        }

        unsafe {
            STEPS = steps;
        }

        return Box::new(String::from_iter(collected.iter().map(|x| x.to_string())));
    }

    fn part2(&self) -> Box<dyn Display> {
        let steps: usize;
        unsafe {
            steps = STEPS;
        }
        return Box::new(steps + 1);
    }
}

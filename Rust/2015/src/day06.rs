use shared::v2i::Vector2Int;

use crate::Base;
use std::fmt::Display;

enum Command {
    On,
    Off,
    Toggle,
}

pub struct Day06 {
    input: Vec<(Command, Vector2Int, Vector2Int)>,
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 { input: Vec::new() };
    }

    fn turn_off(_: bool) -> bool {
        return false;
    }

    fn turn_on(_: bool) -> bool {
        return true;
    }

    fn toggle(val: bool) -> bool {
        return !val;
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = Vec::new();

        for line in raw_input.split("\n") {
            let parts: Vec<&str> = line.trim().split(" ").collect();

            let start: Vec<isize> = parts[parts.len() - 3]
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();
            let end: Vec<isize> = parts[parts.len() - 1]
                .split(",")
                .map(|x| x.parse().unwrap())
                .collect();

            let cmd = match parts[parts.len() - 4].chars().last().unwrap() {
                'f' => Command::Off,
                'e' => Command::Toggle,
                'n' => Command::On,
                _ => panic!(),
            };

            self.input.push((
                cmd,
                Vector2Int::new(start[0], start[1]),
                Vector2Int::new(end[0], end[1]),
            ))
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut grid = [[false; 1000]; 1000];

        let mut _cmd: fn(bool) -> bool;

        for (c, s, e) in &self.input {
            _cmd = match c {
                Command::Off => Day06::turn_off,
                Command::On => Day06::turn_on,
                Command::Toggle => Day06::toggle,
            };

            for y in s.y..e.y + 1 {
                for x in s.x..e.x + 1 {
                    grid[y as usize][x as usize] = _cmd(grid[y as usize][x as usize]);
                }
            }
        }

        return Box::new(grid.iter().flatten().filter(|x| x == &&true).count());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut grid: [[i8; 1000]; 1000] = [[0; 1000]; 1000];

        let mut _diff = 0;

        for (c, s, e) in &self.input {
            _diff = match c {
                Command::Off => -1,
                Command::On => 1,
                Command::Toggle => 2,
            };

            for y in s.y..e.y + 1 {
                for x in s.x..e.x + 1 {
                    grid[y as usize][x as usize] =
                        std::cmp::max(grid[y as usize][x as usize] + _diff, 0);
                }
            }
        }

        let m: isize = grid.iter().flatten().map(|x| *x as isize).sum();

        return Box::new(m);
    }
}

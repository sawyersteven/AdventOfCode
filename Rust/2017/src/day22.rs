use shared::{utils::parse_to_char2d, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

const CLEAN: char = '.';
const INFECTED: char = '#';
const WEAK: char = 'W';
const FLAGGED: char = 'F';

pub struct Day22 {
    pub input: Vec<Vec<char>>,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 { input: Vec::new() };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = parse_to_char2d(raw_input, CLEAN, 1000);
    }

    fn part1(&self) -> Box<dyn Display> {
        const TURNS: usize = 10000;
        let mut direction = 2;

        let mut grid = self.input.clone();

        let mut pos = Vector2Int::new(((grid[0].len() - 1) / 2) as isize, ((grid.len() - 1) / 2) as isize);
        let mut infections_created = 0;

        for _ in 0..TURNS {
            let current = grid[pos.y as usize][pos.x as usize];
            if current == INFECTED {
                direction = (direction + 3) % 4;
                grid[pos.y as usize][pos.x as usize] = CLEAN;
            } else {
                direction = (direction + 1) % 4;
                grid[pos.y as usize][pos.x as usize] = INFECTED;
                infections_created += 1;
            }
            pos += Vector2Int::CARDINALS[direction];
        }

        return Box::new(infections_created);
    }

    fn part2(&self) -> Box<dyn Display> {
        const TURNS: usize = 10000000;
        let mut direction = 2;

        let mut grid = self.input.clone();

        let mut pos = Vector2Int::new(((grid[0].len() - 1) / 2) as isize, ((grid.len() - 1) / 2) as isize);

        let mut infections_created = 0;
        for _ in 0..TURNS {
            let current = grid[pos.y as usize][pos.x as usize];
            match current {
                INFECTED => {
                    grid[pos.y as usize][pos.x as usize] = FLAGGED;
                    direction = (direction + 3) % 4;
                }
                FLAGGED => {
                    grid[pos.y as usize][pos.x as usize] = CLEAN;
                    direction = (direction + 2) % 4;
                }
                CLEAN => {
                    grid[pos.y as usize][pos.x as usize] = WEAK;
                    direction = (direction + 1) % 4;
                }
                WEAK => {
                    grid[pos.y as usize][pos.x as usize] = INFECTED;
                    infections_created += 1;
                }

                _ => panic!(),
            }

            pos += Vector2Int::CARDINALS[direction];
        }
        return Box::new(infections_created);
    }
}

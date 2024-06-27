use shared::v2i::Vector2Int;

use crate::{
    intcode::{self, Emulator, Response},
    Base,
};
use std::{collections::HashSet, fmt::Display};

const N: usize = 1;
const S: usize = 2;
const W: usize = 3;
const E: usize = 4;

const WALL: u8 = b'#';
const EMPTY: u8 = b'.';
const AIR: u8 = b'A';

const GRID_SZ: usize = 50;

const DIRS: [Vector2Int; 4] = [Vector2Int::UP, Vector2Int::DOWN, Vector2Int::LEFT, Vector2Int::RIGHT];

pub struct Day15 {
    grid: [[u8; GRID_SZ]; GRID_SZ],
    end_pos: Vector2Int,
    code: Vec<isize>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 {
            grid: [[EMPTY; GRID_SZ]; GRID_SZ],
            end_pos: Vector2Int::ZERO,
            code: Vec::new(),
        };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.code = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emu = Emulator::new(self.code.clone());
        let mut resp: Response;

        let mut current_pos = Vector2Int::new((GRID_SZ / 2) as isize, (GRID_SZ / 2) as isize);
        let mut start_pos = current_pos;
        let mut current_dir = W;

        let mut walls = HashSet::new();

        let mut t = 0;
        let mut changes = 0;
        loop {
            let right_hand = current_pos + DIRS[ccw(current_dir) - 1];

            if self.grid[right_hand.y as usize][right_hand.x as usize] != WALL {
                current_dir = ccw(current_dir);
            }

            let next_pos = current_pos + DIRS[current_dir - 1];

            if next_pos.x == -1
                || next_pos.x >= GRID_SZ as isize
                || next_pos.y == -1
                || next_pos.y >= GRID_SZ as isize
            {
                current_dir = cw(cw(current_dir));
                continue;
            }

            emu.queue_input(&[current_dir as isize]);
            resp = emu.run();
            match resp.1 {
                0 => {
                    changes += if self.grid[next_pos.y as usize][next_pos.x as usize] == WALL {
                        0
                    } else {
                        1
                    };
                    walls.insert(Vector2Int::new(next_pos.x, next_pos.y));
                    self.grid[next_pos.y as usize][next_pos.x as usize] = WALL;
                    current_dir = cw(current_dir);
                }
                1 => {
                    changes += if self.grid[next_pos.y as usize][next_pos.x as usize] == EMPTY {
                        0
                    } else {
                        1
                    };
                    self.grid[current_pos.y as usize][current_pos.x as usize] =
                        if self.grid[current_pos.y as usize][current_pos.x as usize] == AIR {
                            AIR
                        } else {
                            EMPTY
                        };
                    current_pos = next_pos;
                }
                2 => {
                    self.grid[next_pos.y as usize][next_pos.x as usize] = AIR;
                    self.end_pos = next_pos;
                    current_pos = next_pos;
                }
                _ => {}
            }
            t += 1;
            if t % GRID_SZ == 0 {
                if changes == 0 {
                    break;
                }
                changes = 0;
            }
        }
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

fn ccw(d: usize) -> usize {
    return match d {
        1 => 3,
        2 => 4,
        3 => 2,
        4 => 1,
        _ => panic!(),
    };
}

fn cw(d: usize) -> usize {
    return match d {
        1 => 4,
        2 => 3,
        3 => 1,
        4 => 2,
        _ => panic!(),
    };
}

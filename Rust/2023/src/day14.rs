use shared::{grid2d::Grid2D, utils};

use crate::Base;
use std::fmt::Display;

const ROCK: u8 = b'O';
const GROUND: u8 = b'.';

pub struct Day14 {
    grid: Grid2D<u8>,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 {
            grid: Grid2D::new(0, 0),
        };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        self.grid = Grid2D::<u8>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut grid = self.grid.clone();

        tilt_north(&mut grid);

        return Box::new(north_load(&grid));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const OFFSET: usize = 150;

        let mut grid = self.grid.clone();

        // Tweak these params for maximum speed
        match utils::find_pattern(OFFSET, 72, 72, 2, &mut || {
            tilt_nwse(&mut grid);
            return north_load(&grid);
        }) {
            None => return Box::new("Could not find stable pattern"),
            Some(p) => {
                println!("{}", p.len());
                let r = (1_000_000_000 - OFFSET - 1) % p.len();
                return Box::new(p[r]);
            }
        }
    }
}

fn north_load(grid: &Grid2D<u8>) -> usize {
    let mut total = 0;
    let sz = grid.size();
    for y in 0..sz.y {
        for b in grid.iter_row(y) {
            if *b == ROCK {
                total += sz.y - y;
            }
        }
    }
    return total;
}

fn tilt_north(grid: &mut Grid2D<u8>) {
    let sz = grid.size();
    for y in 1..sz.y {
        for x in 0..sz.x {
            if *grid.get(x, y) == ROCK {
                let mut move_to_y = y;
                while move_to_y > 0 && *grid.get(x, move_to_y - 1) == GROUND {
                    move_to_y -= 1;
                }
                grid.set(x, y, GROUND);
                grid.set(x, move_to_y, ROCK);
            }
        }
    }
}

fn tilt_nwse(grid: &mut Grid2D<u8>) {
    let sz = grid.size();
    for y in 1..sz.y {
        for x in 0..sz.x {
            if *grid.get(x, y) == ROCK {
                let mut move_to_y = y;
                while move_to_y > 0 && *grid.get(x, move_to_y - 1) == GROUND {
                    move_to_y -= 1;
                }
                grid.set(x, y, GROUND);
                grid.set(x, move_to_y, ROCK);
            }
        }
    }
    for x in 1..sz.x {
        for y in 0..sz.y {
            if *grid.get(x, y) == ROCK {
                let mut move_to_x = x;
                while move_to_x > 0 && *grid.get(move_to_x - 1, y) == GROUND {
                    move_to_x -= 1;
                }
                grid.set(x, y, GROUND);
                grid.set(move_to_x, y, ROCK);
            }
        }
    }
    for y in (0..(sz.y - 1)).rev() {
        for x in 0..sz.x {
            if *grid.get(x, y) == ROCK {
                let mut move_to_y = y;
                while move_to_y < sz.y - 1 && *grid.get(x, move_to_y + 1) == GROUND {
                    move_to_y += 1;
                }
                grid.set(x, y, GROUND);
                grid.set(x, move_to_y, ROCK);
            }
        }
    }
    for x in (0..(sz.x - 1)).rev() {
        for y in 0..sz.y {
            if *grid.get(x, y) == ROCK {
                let mut move_to_x = x;
                while move_to_x < sz.x - 1 && *grid.get(move_to_x + 1, y) == GROUND {
                    move_to_x += 1;
                }
                grid.set(x, y, GROUND);
                grid.set(move_to_x, y, ROCK);
            }
        }
    }
}

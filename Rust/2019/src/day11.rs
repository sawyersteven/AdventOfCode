use shared::v2i::Vector2Int;

use crate::{
    intcode::{self, Emulator, StatusCode},
    Base,
};
use std::{collections::HashSet, fmt::Display};

const GRID_SIZE: usize = 100;
const BLACK: u8 = 0;
const WHITE: u8 = 1;

const DIRS: [Vector2Int; 4] = [Vector2Int::DOWN, Vector2Int::RIGHT, Vector2Int::UP, Vector2Int::LEFT];

pub struct Day11 {
    code: Vec<isize>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 { code: Vec::new() };
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.code = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut grid = [[BLACK; GRID_SIZE]; GRID_SIZE];
        let count = self.run_bot(&mut grid);
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut grid = [[BLACK; GRID_SIZE]; GRID_SIZE];

        grid[GRID_SIZE / 2][GRID_SIZE / 2] = WHITE;

        self.run_bot(&mut grid);

        let mut output = String::new();
        for i in 0..GRID_SIZE {
            let mut line = String::new();

            for j in 0..GRID_SIZE {
                line.push(if grid[i][j] == BLACK { ' ' } else { '#' });
            }

            if !line.as_bytes().iter().all(|x| *x == b' ') {
                output.push('\n');
                output.push_str(&*line);
            }
        }
        return Box::new(output);
    }
}

impl Day11 {
    fn run_bot(&mut self, grid: &mut [[u8; GRID_SIZE]; GRID_SIZE]) -> usize {
        let mut emu = Emulator::new(self.code.clone());
        let mut resp: (StatusCode, isize);
        let mut dir: isize = 0;
        let mut loc = Vector2Int::new((GRID_SIZE / 2) as isize, (GRID_SIZE / 2) as isize);

        let mut painted_tiles = HashSet::new();
        for _ in 0..isize::MAX {
            emu.queue_input(&[grid[loc.y as usize][loc.x as usize] as isize]);
            resp = emu.run();
            if resp.0 != StatusCode::OutputDelivery {
                break;
            }

            grid[loc.y as usize][loc.x as usize] = resp.1 as u8;
            painted_tiles.insert(loc);

            resp = emu.run();
            let dir_delta = if resp.1 == 1 { 1 } else { -1 };

            dir = (dir + dir_delta + DIRS.len() as isize) % DIRS.len() as isize;
            loc += DIRS[dir as usize];
        }
        return painted_tiles.len();
    }
}

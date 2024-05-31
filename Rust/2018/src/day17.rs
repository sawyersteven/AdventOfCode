use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::VecDeque, fmt::Display, fs::File, io::Write};

const CLAY: u8 = b'#';
const EMPTY: u8 = b' ';
const WATER: u8 = b'~';
const DROP: u8 = b'|';

pub struct Day17 {
    grid: Vec<Vec<u8>>,
    drop_points: VecDeque<Vector2Int>,
    flood_points: VecDeque<Vector2Int>,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 {
            grid: Vec::new(),
            drop_points: VecDeque::new(),
            flood_points: VecDeque::new(),
        };
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        let mut clay_cells = Vec::new();
        let mut min_x = isize::MAX;
        let mut max_x = 0;
        let mut max_y = 0;

        for line in raw_input.split('\n') {
            if line.as_bytes()[0] == b'x' {
                let x: isize = line[2..].split(',').nth(0).unwrap().parse().unwrap();
                let y_range: Vec<&str> = line.split("y=").nth(1).unwrap().split("..").collect();

                let mut y_min = y_range[0].parse().unwrap();
                let y_max = y_range[1].parse().unwrap();
                while y_min <= y_max {
                    clay_cells.push(Vector2Int::new(x, y_min));
                    y_min += 1;
                }
            } else if line.as_bytes()[0] == b'y' {
                let y: isize = line[2..].split(',').nth(0).unwrap().parse().unwrap();
                let x_range: Vec<&str> = line.split("x=").nth(1).unwrap().split("..").collect();

                let mut x_min = x_range[0].parse().unwrap();
                let x_max = x_range[1].parse().unwrap();
                while x_min <= x_max {
                    clay_cells.push(Vector2Int::new(x_min, y));
                    x_min += 1;
                }
            }
        }

        for v in &clay_cells {
            min_x = min_x.min(v.x);
            max_x = max_x.max(v.x);
            max_y = max_y.max(v.y);
        }

        max_y += 1;
        min_x -= 1;

        let width = max_x - min_x + 1;

        self.grid = vec![vec![EMPTY; width as usize]; max_y as usize];

        for c in &clay_cells {
            self.grid[c.y as usize][(c.x - min_x) as usize] = CLAY;
        }

        self.grid[0][(500 - min_x) as usize] = b'+';
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.run_sim();
        let mut s = String::new();
        for y in 0..self.grid.len() {
            for x in 0..self.grid[0].len() {
                s.push(self.grid[y][x] as char);
            }
        }

        let mut f = File::create(r"G:\Users\Steven\Desktop\rust.txt").unwrap();
        f.write_all(s.as_bytes()).unwrap();

        return Box::new(self.count_water());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.count_water_p2());
    }
}

impl Day17 {
    fn count_water(&self) -> usize {
        let mut count = 0;

        let mut start_y = 0;
        while start_y < self.grid.len() {
            if self.grid[start_y].contains(&CLAY) {
                break;
            }
            start_y += 1;
        }

        for y in start_y..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let c = self.grid[y][x];
                if c == WATER || c == DROP {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn count_water_p2(&self) -> usize {
        let mut count = 0;

        let mut start_y = 0;
        while start_y < self.grid.len() {
            if self.grid[start_y].contains(&CLAY) {
                break;
            }
            start_y += 1;
        }

        for y in start_y..self.grid.len() {
            for x in 0..self.grid[0].len() {
                let c = self.grid[y][x];
                if c == WATER {
                    count += 1;
                }
            }
        }

        return count;
    }

    fn run_sim(&mut self) {
        let mut start = Vector2Int::new(0, 0);
        for i in 0..self.grid.len() {
            if self.grid[0][i] == b'+' {
                start.x = i as isize;
                break;
            }
        }

        self.drop_points.push_front(start);

        while self.drop_points.len() + self.flood_points.len() > 0 {
            while let Some(fp) = self.flood_points.pop_front() {
                self.flood_fill(fp);
            }
            let dp = self.drop_points.pop_front().unwrap();
            self.fill_down(dp);
        }
    }

    fn fill_down(&mut self, start: Vector2Int) {
        let mut start = start;
        if self.grid[start.y as usize + 1][start.x as usize] != EMPTY {
            return;
        }

        while self.grid[start.y as usize + 1][start.x as usize] == EMPTY {
            start.y += 1;
            self.grid[start.y as usize][start.x as usize] = DROP;
            if start.y as usize + 1 == self.grid.len() || self.grid[start.y as usize + 1][start.x as usize] == DROP
            {
                return;
            }
        }
        self.flood_points.push_back(start);
        return;
    }

    fn flood_fill(&mut self, start: Vector2Int) {
        let mut start = start;
        let mut drop_found = false;
        while !drop_found {
            self.grid[start.y as usize][start.x as usize] = DROP;

            let y = start.y as usize;
            let mut x_left = start.x as usize;
            let mut x_right = start.x as usize;

            loop {
                let next = self.grid[y][x_left - 1];
                if next == CLAY {
                    break;
                }

                x_left -= 1;
                let next_below = self.grid[y + 1][x_left];
                if next_below == DROP {
                    break;
                }
                if next_below == EMPTY {
                    drop_found = true;
                    self.drop_points.push_back(Vector2Int::new(x_left as isize, start.y));
                    break;
                }
            }

            loop {
                let next = self.grid[y][x_right + 1];
                if next == CLAY {
                    break;
                }

                x_right += 1;
                let below = self.grid[y + 1][x_right];
                if below == EMPTY {
                    drop_found = true;
                    self.drop_points.push_back(Vector2Int::new(x_right as isize, start.y));
                    break;
                }
            }

            let fill_char = if drop_found { DROP } else { WATER };

            for x in x_left..=x_right {
                self.grid[y][x] = fill_char;
            }
            if drop_found {
                break;
            }
            start.y -= 1;
        }
    }
}

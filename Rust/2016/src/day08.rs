use crate::Base;
use std::fmt::Display;

const ON: char = '█';
const OFF: char = ' ';

static mut PANEL: Vec<Vec<char>> = Vec::new();

pub struct Day08 {
    pub input: Vec<String>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { input: Vec::new() };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut panel: Vec<Vec<char>> = Vec::new();
        for _ in 0..6 {
            let mut row = Vec::new();
            for _ in 0..50 {
                row.push(OFF);
            }
            panel.push(row);
        }

        let mut vals: Vec<usize>;
        for line in &self.input {
            if line.starts_with("rect") {
                vals = line
                    .split(' ')
                    .nth(1)
                    .unwrap()
                    .split('x')
                    .map(|x| x.parse().unwrap())
                    .collect();
                write_rect(&mut panel, vals[0], vals[1]);
                continue;
            }

            vals = line
                .split('=')
                .nth(1)
                .unwrap()
                .split(" by ")
                .map(|x| x.parse().unwrap())
                .collect();

            if line.contains("row") {
                rot_row(&mut panel, vals[0], vals[1]);
            } else {
                rot_col(&mut panel, vals[0], vals[1]);
            }
        }

        let mut pixels = 0;
        for y in 0..6 {
            for x in 0..50 {
                if panel[y][x] == ON {
                    pixels += 1;
                }
            }
        }

        // I don't want self to be mutable, and this is safe.
        unsafe {
            PANEL = panel.clone();
        }

        return Box::new(pixels);
    }

    fn part2(&self) -> Box<dyn Display> {
        let panel: Vec<Vec<char>>;
        unsafe {
            panel = PANEL.clone();
        }

        let mut display = String::new();
        display.push('\n');
        for row in panel {
            let mut line = String::new();
            for c in row {
                line.push(c);
            }
            display.push_str(line.as_str());
            display.push('\n');
        }

        return Box::new(display);
    }
}

fn write_rect(panel: &mut Vec<Vec<char>>, w: usize, h: usize) {
    for y in 0..h {
        for x in 0..w {
            panel[y][x] = ON;
        }
    }
}

fn rot_col(panel: &mut Vec<Vec<char>>, col: usize, by: usize) {
    let h = panel.len();
    for _ in 0..by {
        let last = panel[h - 1][col];
        for y in (1..h).rev() {
            panel[y][col] = panel[y - 1][col];
        }
        panel[0][col] = last;
    }
}

fn rot_row(panel: &mut Vec<Vec<char>>, row: usize, by: usize) {
    let w = panel[0].len();
    for _ in 0..by {
        let last = panel[row][w - 1];
        for x in (1..w).rev() {
            panel[row][x] = panel[row][x - 1];
        }
        panel[row][0] = last;
    }
}

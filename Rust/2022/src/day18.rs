use crate::Base;
use std::fmt::Display;

type Cell = (isize, isize, isize);

pub struct Day18 {
    coords: Vec<Cell>,
    grid_3d: Vec<Vec<Vec<bool>>>,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 {
            coords: Vec::new(),
            grid_3d: Vec::new(),
        };
    }

    fn make_grid(&mut self) {
        let mut x_range = 0..0;
        let mut y_range = 0..0;
        let mut z_range = 0..0;
        for cell in &self.coords {
            x_range.end = x_range.end.max(cell.0);
            y_range.end = y_range.end.max(cell.1);
            z_range.end = z_range.end.max(cell.2);
        }

        x_range.end += 4;
        y_range.end += 4;
        z_range.end += 4;

        self.grid_3d = x_range
            .map(|_| {
                y_range
                    .clone()
                    .map(|_| z_range.clone().map(|_| false).collect())
                    .collect()
            })
            .collect();
    }

    fn count_surface<'a, F>(&self, coords: F) -> usize
    where
        F: Iterator<Item = &'a Cell>,
    {
        let mut count = 0;

        for cell in coords {
            let cell = (cell.0, cell.1, cell.2);
            for f in FACES {
                if !self.grid_3d[(f.0 + cell.0) as usize][(f.1 + cell.1) as usize][(f.2 + cell.2) as usize] {
                    count += 1;
                }
            }
        }
        return count;
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let nums: Vec<isize> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            self.coords.push((nums[0], nums[1], nums[2]));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.make_grid();

        for cell in self.coords.iter_mut() {
            cell.0 += 1;
            cell.1 += 1;
            cell.2 += 2;
            self.grid_3d[cell.0 as usize][cell.1 as usize][cell.2 as usize] = true;
        }

        let count = self.count_surface(self.coords.iter());

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

const FACES: [(isize, isize, isize); 6] = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];

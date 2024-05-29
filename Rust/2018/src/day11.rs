use crate::Base;
use std::fmt::Display;

const GRIDSIZE: usize = 300;

pub struct Day11 {
    serial_num: isize,
    power_grid: Vec<Vec<isize>>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            serial_num: 0,
            power_grid: vec![vec![0; GRIDSIZE]; GRIDSIZE],
        };
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.serial_num = raw_input.parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        for y in 0..GRIDSIZE {
            for x in 0..GRIDSIZE {
                self.power_grid[y][x] = self.cell_power(x, y);
            }
        }
        let mut best_cell = (0, 0);
        let mut best_cell_val = 0;

        for y in 0..297 {
            for x in 0..297 {
                let p = self.chunk_power(x, y);
                if p > best_cell_val {
                    best_cell_val = p;
                    best_cell = (x, y);
                }
            }
        }

        return Box::new(format!("{}, {}", best_cell.0, best_cell.1));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let ii = IntegralImage::new(&self.power_grid);
        let mut gridbox = (0, 0, 0);
        let mut best_box_val = isize::MIN;

        for size in 1..GRIDSIZE {
            for y in 0..(GRIDSIZE - size) {
                for x in 0..(GRIDSIZE - size) {
                    let v = ii.area_sum(x, y, size, size);
                    if v > best_box_val {
                        best_box_val = v;
                        gridbox = (x - size + 1, y - size + 1, size);
                    }
                }
            }
        }

        return Box::new(format!("{}, {}, {}", gridbox.0, gridbox.1, gridbox.2));
    }
}

impl Day11 {
    fn cell_power(&self, x: usize, y: usize) -> isize {
        let rid = x as isize + 10;
        let mut power = rid * y as isize;
        power += self.serial_num;
        power *= rid;
        power = (power / 100) % 10;
        power -= 5;
        return power;
    }

    pub fn chunk_power(&self, x: usize, y: usize) -> isize {
        let mut total = 0;
        for yy in y..(y + 3) {
            for xx in x..(x + 3) {
                total += self.power_grid[yy][xx];
            }
        }
        return total;
    }
}

struct IntegralImage {
    table: Vec<Vec<isize>>,
    width: usize,
    height: usize,
}

impl IntegralImage {
    pub fn new(original: &Vec<Vec<isize>>) -> Self {
        let h = original.len();
        let w = original[0].len();

        let mut ii = IntegralImage {
            table: vec![vec![0; w]; h],
            width: w,
            height: h,
        };

        for y in 0..h {
            for x in 0..w {
                ii.table[y][x] = original[y][x] + ii.get(x - 1, y) + ii.get(x, y - 1) - ii.get(x - 1, y - 1);
            }
        }

        return ii;
    }

    pub fn get(&self, x: usize, y: usize) -> isize {
        if x >= self.width || y >= self.height {
            return 0;
        }
        return self.table[y][x];
    }

    pub fn area_sum(&self, x: usize, y: usize, w: usize, h: usize) -> isize {
        return self.get(x, y) - self.get(x, y - h) - self.get(x - w, y) + self.get(x - w, y - h);
    }
}

use shared::v2i::Vector2Int;

use crate::Base;
use std::fmt::Display;

pub struct Day03 {
    pub input: String,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 {
            input: String::from(""),
        };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let addr = self.input.parse().unwrap();

        let mut ring_end: isize = 1;
        let mut ring_num: isize = 0;
        while ring_end < addr {
            ring_num += 1;
            ring_end += ring_num * 2 * 4;
        }

        let mut steps_to_cardinal = isize::MAX;
        for i in 0..4 {
            let d = (addr - (ring_end - (ring_num * i * 2) - ring_num)).abs();
            steps_to_cardinal = steps_to_cardinal.min(d);
        }
        return Box::new(ring_num + steps_to_cardinal);
    }

    fn part2(&self) -> Box<dyn Display> {
        let addr = self.input.parse().unwrap();

        let mut grid = Vec::<Vec<usize>>::new();
        for _ in 0..30 {
            let mut row = Vec::new();
            for __ in 0..30 {
                row.push(0);
            }
            grid.push(row);
        }

        grid[5][5] = 1;

        let mut current = Vector2Int::new(6, 5);
        let mut dir = 1;
        let mut lhs = Vector2Int::CARDINALS[2];

        let mut num = 1;
        while num < addr {
            num = sum_neighbors(&grid, &current);
            grid[current.y as usize][current.x as usize] = num;

            if grid[(current.y + lhs.y) as usize][(current.x + lhs.x) as usize] == 0 {
                current += lhs;
                dir = (dir + 1) % 4;
                lhs = Vector2Int::CARDINALS[(dir + 1) % 4];
            } else {
                current += Vector2Int::CARDINALS[dir];
            }
        }

        return Box::new(num);
    }
}

fn sum_neighbors(grid: &Vec<Vec<usize>>, center: &Vector2Int) -> usize {
    let mut sum = 0;
    let y = center.y as usize;
    let x = center.x as usize;
    sum += grid[y - 1][x];
    sum += grid[y + 1][x];
    sum += grid[y][x - 1];
    sum += grid[y + 1][x - 1];
    sum += grid[y - 1][x - 1];
    sum += grid[y][x + 1];
    sum += grid[y + 1][x + 1];
    sum += grid[y - 1][x + 1];
    return sum;
}

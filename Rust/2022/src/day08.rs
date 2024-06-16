use shared::grid2d::Grid2D;

use crate::Base;
use std::fmt::Display;

pub struct Day08 {
    map: Grid2D<u8>,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 { map: Grid2D::new(0, 0) };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.lines().collect();
        let h = lines.len();
        let w = lines[0].len();
        self.map = Grid2D::from_iter(w, h, raw_input.bytes().filter(|x| *x != b'\n')).unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;

        let map_size = self.map.size();
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let h = self.map[(x, y)];
                if (0..x).all(|i| self.map[(i, y)] < h)
                    || ((x + 1)..map_size.x).all(|i| self.map[(i, y)] < h)
                    || (0..y).all(|i| self.map[(x, i)] < h)
                    || ((y + 1)..map_size.y).all(|i| self.map[(x, i)] < h)
                {
                    count += 1;
                }
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut best = 0;
        let grid_size = self.map.size();

        for y in 0..grid_size.y {
            for x in 0..grid_size.x {
                let h = self.map[(x, y)];

                let mut counts = [0, 0, 0, 0]; // w,e,n,s
                for i in (0..x).rev() {
                    counts[0] += 1;
                    if self.map[(i, y)] >= h {
                        break;
                    }
                }

                for i in (x + 1)..grid_size.x {
                    counts[1] += 1;
                    if self.map[(i, y)] >= h {
                        break;
                    }
                }

                for i in (0..y).rev() {
                    counts[2] += 1;
                    if self.map[(x, i)] >= h {
                        break;
                    }
                }

                for i in (y + 1)..grid_size.y {
                    counts[3] += 1;
                    if self.map[(x, i)] >= h {
                        break;
                    }
                }

                best = best.max(counts.iter().product());
            }
        }

        return Box::new(best);
    }
}

use std::fmt::Display;

use shared::v2i::Vector2Int;

use crate::Base;

pub struct Day11 {
    galaxies: Vec<Vector2Int>,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 { galaxies: Vec::new() };
    }

    // Find an empty column, then every galaxy to its right is offset by factor
    fn expand_space(&self, factor: isize) -> Vec<Vector2Int> {
        let factor = factor - 1;
        let mut galaxies = self.galaxies.clone();

        let mut xmin = isize::MAX;
        let mut xmax = isize::MIN;
        let mut ymin = isize::MAX;
        let mut ymax = isize::MIN;

        for g in &self.galaxies {
            xmin = xmin.min(g.x);
            xmax = xmax.max(g.x);

            ymin = ymin.min(g.y);
            ymax = ymax.max(g.y + 1);
        }

        while xmin < xmax {
            if galaxies.iter().any(|g| g.x == xmin) {
                xmin += 1;
                continue;
            }
            galaxies.iter_mut().filter(|g| g.x > xmin).for_each(|g| g.x += factor);
            xmax += factor;
            xmin += 1 + factor;
        }
        while ymin < ymax {
            if galaxies.iter().any(|g| g.y == ymin) {
                ymin += 1;
                continue;
            }
            galaxies.iter_mut().filter(|g| g.y > ymin).for_each(|g| g.y += factor);
            ymax += factor;
            ymin += 1 + factor;
        }

        return galaxies;
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                if *b == b'#' {
                    self.galaxies.push(Vector2Int::new(x as isize, y as isize));
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let galaxies = self.expand_space(2);
        let mut total = 0;
        for i in 0..(galaxies.len() - 1) {
            for j in (i + 1)..galaxies.len() {
                total += galaxies[i].manhattan(&galaxies[j]);
            }
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let galaxies = self.expand_space(1000000);

        let mut total = 0;
        for i in 0..(galaxies.len() - 1) {
            for j in (i + 1)..galaxies.len() {
                total += galaxies[i].manhattan(&galaxies[j]);
            }
        }

        return Box::new(total);
    }
}

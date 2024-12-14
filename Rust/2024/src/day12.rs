use shared::{bitset::Bitset2D, grid2d::Grid2D};

use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day12 {
    input: String,
    plots: Vec<Plot>,
}
/// This is ugly and clunky and slow for several reasons. I don't intend to fix it.
/// 1) 2d grids are getting tedious and I just wanted to finish this
/// 2) For some reason I thought it would be better to use (usize, usize)
///  instead of Vec2s
/// 3) I'm using a Bitset struct I just made because it is much faster than
///  hashsets because I can store the linear index of the map cell instead of
///  hashing a vec2 and store them as single bits instead of hashes
impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            input: String::new(),
            plots: Vec::new(),
        };
    }

    fn make_plot(&self, map: &Grid2D<u8>, start: (usize, usize), handled: &mut Bitset2D) -> Plot {
        let sz = map.size();

        let mut p = Plot {
            perim: 0,
            map: Bitset2D::new(handled.rows(), handled.cols()),
        };

        let sz = (sz.x, sz.y);

        let c = map[start];

        let mut visited = HashSet::new();
        let mut stack = Vec::new();
        stack.push(start);
        visited.insert(start);

        while let Some(cur) = stack.pop() {
            visited.insert(cur);
            for next in neighbors(cur) {
                if visited.contains(&next) {
                    continue;
                }
                if !in_range(next, (0, 0), sz) || map[next] != c {
                    p.perim += 1;
                    continue;
                }
                stack.push(next);
                visited.insert(next);
            }

            handled.set(cur.0, cur.1);
            p.map.set(cur.0, cur.1);
        }
        return p;
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let map = Grid2D::<u8>::from_string(&self.input);
        let mut handled = Bitset2D::new(map.size().x, map.size().y);

        let sz = map.size();
        for y in 0..sz.y {
            for x in 0..sz.x {
                if handled.is_set(x, y) {
                    continue;
                }

                let p = self.make_plot(&map, (x, y), &mut handled);
                self.plots.push(p);
            }
        }

        let mut cost = 0;
        for p in &self.plots {
            cost += p.perim * p.map.count_set();
        }

        return Box::new(cost);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        if self.plots.len() == 0 {
            return Box::new("Run part1 first to populate plots");
        }
        let mut cost = 0;
        for plot in &self.plots {
            let mut corners = 0;
            for x in 0..plot.map.cols() {
                for y in 0..plot.map.rows() {
                    if !plot.map.is_set(x, y) {
                        continue;
                    }
                    // Find open neighbor cells

                    // corners
                    let sw = !plot.map.is_set(x.wrapping_sub(1), y.wrapping_sub(1)); // nw
                    let se = !plot.map.is_set(x + 1, y.wrapping_sub(1)); // ne
                    let nw = !plot.map.is_set(x.wrapping_sub(1), y + 1); // sw
                    let ne = !plot.map.is_set(x + 1, y + 1); // se

                    // cardinals
                    let n = !plot.map.is_set(x, y + 1); // n
                    let e = !plot.map.is_set(x + 1, y); // e
                    let s = !plot.map.is_set(x, y.wrapping_sub(1)); // s
                    let w = !plot.map.is_set(x.wrapping_sub(1), y); // w

                    // if any two adjacent cardinal direction are open it is an exterior corner
                    corners += r#u(w & s) + r#u(s & e) + r#u(e & n) + r#u(n & w);

                    // if any two adjacent cardinal directions are blocked AND the
                    // diagonal between in open it is an inside corner
                    corners += r#u(!w & !s & sw) + r#u(!s & !e & se) + r#u(!e & !n & ne) + r#u(!n & !w & nw);
                }
            }
            cost += corners * plot.map.count_set();
        }
        return Box::new(cost);
    }
}

fn r#u(b: bool) -> usize {
    return b as usize;
}

struct Plot {
    perim: usize,
    map: Bitset2D,
}

#[inline(always)]
fn in_range(val: (usize, usize), min: (usize, usize), max: (usize, usize)) -> bool {
    return val.0 >= min.0 && val.1 >= min.1 && val.0 < max.0 && val.1 < max.1;
}

/// Iters neighbors up, right, down, left and doesn't care about underflows
#[inline(always)]
fn neighbors(orig: (usize, usize)) -> [(usize, usize); 4] {
    return [
        (orig.0, orig.1 + 1),
        (orig.0 + 1, orig.1),
        (orig.0, orig.1.wrapping_sub(1)),
        (orig.0.wrapping_sub(1), orig.1),
    ];
}

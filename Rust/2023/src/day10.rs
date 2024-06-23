use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day10 {
    map: Grid2D<u8>,
    start: Vector2Int,
    loop_vertices: Vec<Vector2Int>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 {
            map: Grid2D::new(0, 0),
            start: Vector2Int::ZERO,
            loop_vertices: Vec::new(),
        };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
        let sz = self.map.size();
        'outer: for y in 0..sz.y {
            for (x, b) in self.map.iter_row(y).enumerate() {
                if *b == b'S' {
                    self.start = Vector2Int::new(x as isize, y as isize);
                    break 'outer;
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let connections = HashMap::from([
            (b'F', [Vector2Int::new(1, 0), Vector2Int::new(0, 1)]),
            (b'7', [Vector2Int::new(-1, 0), Vector2Int::new(0, 1)]),
            (b'J', [Vector2Int::new(-1, 0), Vector2Int::new(0, -1)]),
            (b'L', [Vector2Int::new(1, 0), Vector2Int::new(0, -1)]),
            (b'-', [Vector2Int::new(-1, 0), Vector2Int::new(1, 0)]),
            (b'|', [Vector2Int::new(0, -1), Vector2Int::new(0, 1)]),
        ]);

        let sz = self.map.size();
        let szi = Vector2Int::new(sz.x as isize, sz.y as isize);

        self.loop_vertices.push(self.start);
        let mut prev = self.start;
        let mut current = Vector2Int::ZERO;

        'outer: for next in self.start.neighbors() {
            if !next.in_range(&Vector2Int::ZERO, &szi) {
                continue;
            }

            let c = self.map.get(next.x as usize, next.y as usize);
            if *c == b'.' {
                continue;
            }

            for nn in connections[&c] {
                if next + nn == self.start {
                    current = next;
                    break 'outer;
                }
            }
        }

        while current != self.start {
            for step in connections[&self.map.get(current.x as usize, current.y as usize)] {
                let neighbor = current + step;
                if neighbor == prev || !neighbor.in_range(&Vector2Int::ZERO, &szi) {
                    continue;
                }
                self.loop_vertices.push(current);
                prev = current;
                current = neighbor;
                break;
            }
        }
        return Box::new(self.loop_vertices.len() / 2);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        /*
        Shoelace algo for area, Picks to remove pipe tiles
        Took a bit of work until I realized the vertices needs to end with the
        start node instead of stopping at the node that leads back to start
        */
        self.loop_vertices.push(self.start);
        let mut area = 0;

        for i in 0..(self.loop_vertices.len() - 1) {
            area += self.loop_vertices[i].x * self.loop_vertices[i + 1].y;
            area -= self.loop_vertices[i].y * self.loop_vertices[i + 1].x;
        }
        area = (area.abs() + 2 - (self.loop_vertices.len() - 1) as isize) / 2;
        return Box::new(area);
    }
}

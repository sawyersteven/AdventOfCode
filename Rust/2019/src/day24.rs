use shared::{grid2d::Grid2D, v2i::Vector2Int, v3i::Vector3Int};

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const GRID_SZ: usize = 5;

pub struct Day24 {
    grid: Grid2D<bool>,
    p2_bugs: HashSet<Vector3Int>,
    adj_cache: HashMap<Vector2Int, Vec<Vector2Int>>,
    adj_cache_3d: HashMap<Vector3Int, Vec<Vector3Int>>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            grid: Grid2D::new(GRID_SZ, GRID_SZ),
            p2_bugs: HashSet::new(),
            adj_cache: HashMap::new(),
            adj_cache_3d: HashMap::new(),
        };
    }

    fn make_adj_cache(&mut self) {
        for y in 0..GRID_SZ as isize {
            for x in 0..GRID_SZ as isize {
                let mut adj = Vec::with_capacity(6);

                for v in Vector2Int::CARDINALS {
                    let next = Vector2Int::new(x, y) + v;
                    if (0..GRID_SZ as isize).contains(&next.x) && (0..GRID_SZ as isize).contains(&next.y) {
                        adj.push(next);
                    }
                }
                self.adj_cache.insert(Vector2Int::new(x, y), adj);
            }
        }
    }

    fn adj_tiles_3d(&mut self, center: &Vector3Int) -> &Vec<Vector3Int> {
        if !self.adj_cache_3d.contains_key(center) {
            let mut adj = Vec::new();

            // North tiles
            if center.y == 0 {
                adj.push(Vector3Int::new(2, 1, center.z - 1));
            } else if center.x == 2 && center.y == 3 {
                adj.push(Vector3Int::new(0, 4, center.z + 1));
                adj.push(Vector3Int::new(1, 4, center.z + 1));
                adj.push(Vector3Int::new(2, 4, center.z + 1));
                adj.push(Vector3Int::new(3, 4, center.z + 1));
                adj.push(Vector3Int::new(4, 4, center.z + 1));
            } else {
                adj.push(Vector3Int::new(center.x, center.y - 1, center.z));
            }

            // South tiles
            if center.y == 4 {
                adj.push(Vector3Int::new(2, 3, center.z - 1));
            } else if center.x == 2 && center.y == 1 {
                adj.push(Vector3Int::new(0, 0, center.z + 1));
                adj.push(Vector3Int::new(1, 0, center.z + 1));
                adj.push(Vector3Int::new(2, 0, center.z + 1));
                adj.push(Vector3Int::new(3, 0, center.z + 1));
                adj.push(Vector3Int::new(4, 0, center.z + 1));
            } else {
                adj.push(Vector3Int::new(center.x, center.y + 1, center.z));
            }

            // East tiles
            if center.x == 4 {
                adj.push(Vector3Int::new(3, 2, center.z - 1));
            } else if center.x == 1 && center.y == 2 {
                adj.push(Vector3Int::new(0, 0, center.z + 1));
                adj.push(Vector3Int::new(0, 1, center.z + 1));
                adj.push(Vector3Int::new(0, 2, center.z + 1));
                adj.push(Vector3Int::new(0, 3, center.z + 1));
                adj.push(Vector3Int::new(0, 4, center.z + 1));
            } else {
                adj.push(Vector3Int::new(center.x + 1, center.y, center.z));
            }

            // West tiles
            if center.x == 0 {
                adj.push(Vector3Int::new(1, 2, center.z - 1));
            } else if center.x == 3 && center.y == 2 {
                adj.push(Vector3Int::new(4, 0, center.z + 1));
                adj.push(Vector3Int::new(4, 1, center.z + 1));
                adj.push(Vector3Int::new(4, 2, center.z + 1));
                adj.push(Vector3Int::new(4, 3, center.z + 1));
                adj.push(Vector3Int::new(4, 4, center.z + 1));
            } else {
                adj.push(Vector3Int::new(center.x - 1, center.y, center.z));
            }

            self.adj_cache_3d.insert(*center, adj);
        }
        return &self.adj_cache_3d[center];
    }

    fn bio_rating(&self, grid: &mut Grid2D<bool>) -> usize {
        let mut r = 0;
        let mut v = 1;
        for y in 0..GRID_SZ {
            for x in 0..GRID_SZ {
                if grid[(x, y)] {
                    r += v;
                }
                v <<= 1;
            }
        }
        return r;
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        self.grid = Grid2D::from_iter(
            GRID_SZ,
            GRID_SZ,
            raw_input.bytes().filter(|x| *x != b'\n').map(|x| x == b'#'),
        )
        .unwrap();

        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                if *b == b'#' {
                    self.p2_bugs.insert(Vector3Int::new(x as isize, y as isize, 0));
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut grid = self.grid.clone();
        self.make_adj_cache();

        let mut bio_ratings = HashSet::new();

        let mut current = Vector2Int::ZERO;

        let mut changes = HashSet::new();
        loop {
            for y in 0..GRID_SZ {
                current.y = y as isize;
                for x in 0..GRID_SZ {
                    current.x = x as isize;
                    let mut adj_bugs = 0;
                    for adj in &self.adj_cache[&current] {
                        if grid[adj] {
                            adj_bugs += 1;
                        }
                    }

                    if grid[&current] && adj_bugs != 1 {
                        changes.insert(current);
                    } else if !grid[&current] && (adj_bugs == 1 || adj_bugs == 2) {
                        changes.insert(current);
                    }
                }
            }

            for v in changes.drain() {
                grid[&v] ^= true;
            }

            let br = self.bio_rating(&mut grid);
            if !bio_ratings.insert(br) {
                return Box::new(br);
            }
        }
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut changes = HashSet::new();
        let mut layers = HashSet::from([-1, 0, 1]);

        for _ in 0..200 {
            for z in &layers {
                for y in 0..GRID_SZ {
                    for x in 0..GRID_SZ {
                        if x == 2 && y == 2 {
                            continue;
                        }

                        let c = Vector3Int::new(x as isize, y as isize, *z);

                        let mut adj_bugs = 0;
                        let adjs = self.adj_tiles_3d(&c).clone();
                        for adj in adjs {
                            if self.p2_bugs.contains(&adj) {
                                adj_bugs += 1;
                            }
                        }
                        if self.p2_bugs.contains(&c) && adj_bugs != 1 {
                            changes.insert(c);
                        } else if !self.p2_bugs.contains(&c) && ((1..=2).contains(&adj_bugs)) {
                            changes.insert(c);
                        }
                    }
                }
            }
            for v in changes.drain() {
                layers.insert(v.z - 1);
                layers.insert(v.z + 1);
                if self.p2_bugs.contains(&v) {
                    self.p2_bugs.remove(&v);
                } else {
                    self.p2_bugs.insert(v);
                }
            }
        }
        return Box::new(self.p2_bugs.len());
    }
}

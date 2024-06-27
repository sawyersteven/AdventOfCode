use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub struct Day16 {
    grid: Grid2D<u8>,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            grid: Grid2D::new(0, 0),
        };
    }

    fn sim_from_location(&self, start: LightBeam) -> usize {
        let mut visited = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(start);

        let grid_max: Vector2Int = self.grid.size().to_vector2() - Vector2Int::ONE;
        while q.len() > 0 {
            let mut current = q.pop_back().unwrap();
            loop {
                if !current.pos.in_range(&Vector2Int::ZERO, &grid_max) {
                    break;
                }
                if visited.contains(&current) {
                    break;
                }
                visited.insert(current.clone());

                // select next position and enqueue a split if neccessary
                match self.grid[current.pos] {
                    b'-' => {
                        if ![Vector2Int::RIGHT, Vector2Int::LEFT].contains(&current.dir) {
                            q.push_back(LightBeam {
                                pos: current.pos,
                                dir: Vector2Int::LEFT,
                            });

                            current.dir = Vector2Int::RIGHT;
                        }
                    }
                    b'|' => {
                        if ![Vector2Int::UP, Vector2Int::DOWN].contains(&current.dir) {
                            q.push_back(LightBeam {
                                pos: current.pos,
                                dir: Vector2Int::UP,
                            });
                            current.dir = Vector2Int::DOWN;
                        }
                    }
                    b'\\' => {
                        current.dir = match current.dir {
                            Vector2Int::UP => Vector2Int::RIGHT,
                            Vector2Int::RIGHT => Vector2Int::UP,
                            Vector2Int::DOWN => Vector2Int::LEFT,
                            Vector2Int::LEFT => Vector2Int::DOWN,
                            _ => panic!(),
                        };
                    }
                    b'/' => {
                        current.dir = match current.dir {
                            Vector2Int::UP => Vector2Int::LEFT,
                            Vector2Int::RIGHT => Vector2Int::DOWN,
                            Vector2Int::DOWN => Vector2Int::RIGHT,
                            Vector2Int::LEFT => Vector2Int::UP,
                            _ => panic!(),
                        };
                    }
                    _ => {}
                }
                current.pos += current.dir;
            }
        }
        let mut energized = HashSet::new();
        for lb in visited {
            energized.insert(lb.pos);
        }
        return energized.len();
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        self.grid = Grid2D::<u8>::from_string(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let start = LightBeam {
            pos: Vector2Int { x: 0, y: 0 },
            dir: Vector2Int::RIGHT,
        };

        let nrg = self.sim_from_location(start);
        return Box::new(nrg);
    }

    /* This could be optimized with some path memoization, but at 350ms its ok
    enough. A thought is that memoizing certain mirrors might help. Eg if a beam
    hits a mirror from a certain direction it will always result in the same set
    of energized tiles which can be merged with the visited set without following
    their path
    */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut best: usize;
        let sz = self.grid.size();
        best = self.sim_from_location(LightBeam {
            pos: Vector2Int { x: 3, y: 0 },
            dir: Vector2Int::UP,
        });
        // top/bottom edges
        for x in 0..sz.x {
            best = best.max(self.sim_from_location(LightBeam {
                pos: Vector2Int { x: x as isize, y: 0 },
                dir: Vector2Int::UP,
            }));

            best = best.max(self.sim_from_location(LightBeam {
                pos: Vector2Int {
                    x: x as isize,
                    y: sz.y as isize,
                },
                dir: Vector2Int::DOWN,
            }));
        }

        // left/right edges
        for y in 0..sz.y {
            best = best.max(self.sim_from_location(LightBeam {
                pos: Vector2Int { x: 0, y: y as isize },
                dir: Vector2Int::RIGHT,
            }));

            best = best.max(self.sim_from_location(LightBeam {
                pos: Vector2Int {
                    x: sz.x as isize,
                    y: y as isize,
                },
                dir: Vector2Int::LEFT,
            }));
        }
        return Box::new(best);
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct LightBeam {
    pos: Vector2Int,
    dir: Vector2Int,
}

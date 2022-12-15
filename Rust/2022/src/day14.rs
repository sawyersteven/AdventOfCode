use crate::Base;
use std::{collections::HashSet, fmt::Display};

const SAND_ORIG: Cell = Cell { x: 500, y: 0 };

#[derive(Eq, Hash, PartialEq, Clone, Debug)]
struct Cell {
    x: isize,
    y: isize,
}

pub struct Day14 {
    rock: HashSet<Cell>,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { rock: HashSet::new() };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let points: Vec<Cell> = line
                .split(" -> ")
                .map(|p| p.split_once(',').unwrap())
                .map(|p| Cell {
                    x: p.0.parse().unwrap(),
                    y: -1 * p.1.parse::<isize>().unwrap(),
                })
                .collect();

            let mut current = points[0].clone();

            self.rock.insert(current.clone());
            for next in points.iter().skip(1) {
                while current != *next {
                    current.x += (next.x - current.x).signum();
                    current.y += (next.y - current.y).signum();
                    self.rock.insert(current.clone());
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let low_point = self.rock.iter().map(|r| r.y).min().unwrap();

        let mut sand = HashSet::<Cell>::new();

        loop {
            let mut s = SAND_ORIG.clone();
            loop {
                // drop down as far as possible
                while !self.rock.contains(&s) && !sand.contains(&s) {
                    s.y -= 1;
                    if s.y < low_point {
                        return Box::new(sand.len());
                    }
                }

                // down-left
                s.x -= 1;
                if self.rock.contains(&s) || sand.contains(&s) {
                    // down-right
                    s.x += 2;
                    if self.rock.contains(&s) || sand.contains(&s) {
                        // can't move
                        s.x -= 1;
                        s.y += 1;
                        sand.insert(s);
                        break;
                    }
                }
            }
        }
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut sand = HashSet::<Cell>::new();

        let floor_y = self.rock.iter().map(|r| r.y).min().unwrap() - 2;

        loop {
            let mut s = SAND_ORIG.clone();
            loop {
                // drop down as far as possible
                while s.y > floor_y && !self.rock.contains(&s) && !sand.contains(&s) {
                    s.y -= 1;
                }

                if s.y == floor_y {
                    s.y += 1;
                    sand.insert(s);
                    break;
                }

                // down-left
                s.x -= 1;
                if self.rock.contains(&s) || sand.contains(&s) {
                    // down-right
                    s.x += 2;
                    if self.rock.contains(&s) || sand.contains(&s) {
                        // can't move
                        s.x -= 1;
                        s.y += 1;
                        if s.x == 500 && s.y == 0 {
                            return Box::new(sand.len() + 1);
                        }
                        sand.insert(s);
                        break;
                    }
                }
            }
        }
    }
}

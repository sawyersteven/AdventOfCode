use shared::{v2i::Vector2Int, v3i::Vector3Int};

use crate::Base;
use std::{collections::HashSet, fmt::Display};

const MOVES: [Vector2Int; 5] = [
    Vector2Int::RIGHT,
    Vector2Int::DOWN,
    Vector2Int::UP,
    Vector2Int::LEFT,
    Vector2Int::ZERO,
];

pub struct Day24 {
    blizzards: Vec<Blizzard>,
    start: Vector2Int,
    end: Vector2Int,
    map_size: Vector2Int,
    blizz_bounds: Vector2Int,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            blizzards: Vec::new(),
            start: Vector2Int::ZERO,
            end: Vector2Int::ZERO,
            map_size: Vector2Int::ZERO,
            blizz_bounds: Vector2Int::ZERO,
        };
    }

    /*
        This would be faster if I assume that the quickest full-circuit path
        contains the fastest path to the end, then the fastest to the start,
        but knowing how AoC works the fastest solution probably uses the
        second-fastest of the first leg. So this makes so assumptions.
    */
    fn find_path_time_2(&mut self) -> usize {
        let mut elf_positions = HashSet::new();
        let start = Vector3Int::new(self.start.x, self.start.y, 0);
        elf_positions.insert(start);

        for turn in 1..usize::MAX {
            let mut next_steps = HashSet::new();
            for b in self.blizzards.iter_mut() {
                b.step(&self.blizz_bounds);
            }

            for e_pos in elf_positions {
                for dir in MOVES {
                    let mut next_step = e_pos + dir;

                    let is_at_start = next_step.x == self.start.x && next_step.y == self.start.y;
                    // When returning to start after reaching end, set Z to 2
                    if next_step.z == 1 && is_at_start {
                        next_step.z = 2;
                    }

                    let is_at_end = next_step.x == self.end.x && next_step.y == self.end.y;
                    if is_at_end {
                        // Z is 2 if we've already seen the end and start once
                        if next_step.z == 2 {
                            return turn;
                        }
                        // Set Z == 1 once we reach the end
                        if next_step.z == 0 {
                            next_step.z = 1;
                        }
                    }

                    if !(is_at_end || is_at_start)
                        && (next_step.x < 1
                            || next_step.y < 1
                            || next_step.x > self.blizz_bounds.x
                            || next_step.y > self.blizz_bounds.y)
                    {
                        continue;
                    }

                    if self
                        .blizzards
                        .iter()
                        .any(|x| x.pos.x == next_step.x && x.pos.y == next_step.y)
                    {
                        continue;
                    }

                    next_steps.insert(next_step);
                }
            }
            elf_positions = next_steps;
        }
        unreachable!();
    }

    fn find_path_time(&mut self) -> usize {
        let mut elf_positions = HashSet::new();
        elf_positions.insert(self.start);
        let mut next_steps = HashSet::new();

        for turn in 1..usize::MAX {
            for b in self.blizzards.iter_mut() {
                b.step(&self.blizz_bounds);
            }

            for e_pos in elf_positions {
                for dir in MOVES {
                    let next_step: Vector2Int = e_pos + dir;

                    if next_step == self.end {
                        return turn;
                    }

                    if next_step.x < 1
                        || next_step.y < 1
                        || next_step.x > self.blizz_bounds.x
                        || next_step.y > self.blizz_bounds.y
                    {
                        continue;
                    }

                    if self.blizzards.iter().any(|x| x.pos == next_step) {
                        continue;
                    }

                    next_steps.insert(next_step);
                }
            }
            elf_positions = next_steps.drain().collect();
        }
        unreachable!();
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        self.map_size = Vector2Int::new(
            raw_input.lines().nth(0).unwrap().as_bytes().len() as isize,
            raw_input.lines().count() as isize,
        );

        for (x, b) in raw_input.lines().nth(0).unwrap().as_bytes().iter().enumerate() {
            if *b == b'.' {
                self.start = Vector2Int::new(x as isize, 0);
                break;
            }
        }

        for (x, b) in raw_input.lines().last().unwrap().as_bytes().iter().enumerate() {
            if *b == b'.' {
                self.end = Vector2Int::new(x as isize, self.map_size.y - 1);
                break;
            }
        }

        self.blizz_bounds = self.map_size - Vector2Int::new(2, 2);

        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                match b {
                    b'>' => self.blizzards.push(Blizzard {
                        pos: Vector2Int::new(x as isize, y as isize),
                        dir: Vector2Int::RIGHT,
                    }),
                    b'^' => self.blizzards.push(Blizzard {
                        pos: Vector2Int::new(x as isize, y as isize),
                        dir: Vector2Int::DOWN,
                    }),
                    b'v' => self.blizzards.push(Blizzard {
                        pos: Vector2Int::new(x as isize, y as isize),
                        dir: Vector2Int::UP,
                    }),
                    b'<' => self.blizzards.push(Blizzard {
                        pos: Vector2Int::new(x as isize, y as isize),
                        dir: Vector2Int::LEFT,
                    }),
                    _ => {}
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let t = self.find_path_time();

        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let t = self.find_path_time_2();

        return Box::new(t);
    }
}

struct Blizzard {
    pos: Vector2Int,
    dir: Vector2Int,
}

impl Blizzard {
    // Bounds is allowable area starting at 1..b.X, 1..b.Y
    fn step(&mut self, bounds: &Vector2Int) {
        self.pos = self.pos - Vector2Int::ONE + self.dir;
        self.pos.x = self.pos.x.rem_euclid(bounds.x) + 1;
        self.pos.y = self.pos.y.rem_euclid(bounds.y) + 1;
    }
}

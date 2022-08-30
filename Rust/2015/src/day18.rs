use crate::Base;
use std::{collections::HashSet, fmt::Display};

const STEPS: usize = 100;

pub struct Day18 {
    pub grid: Vec<Vec<bool>>,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 {
            grid: Vec::<Vec<bool>>::new(),
        };
    }

    fn go(mut lights: Vec<Vec<bool>>, corners_on: bool) -> usize {
        let neighbors = [(0, 1), (1, 1), (1, 0), (1, -1), (0, -1), (-1, -1), (-1, 0), (-1, 1)];

        let max_x = lights[0].len() as isize;
        let max_y = lights.len() as isize;

        for _ in 0..STEPS {
            let mut changes: HashSet<(isize, isize)> = HashSet::new();
            let mut current: (isize, isize) = (0, 0);
            for x in 0..max_x {
                current.0 = x;
                for y in 0..max_y {
                    current.1 = y;

                    let mut on_count = 0;
                    for n in neighbors {
                        let nbr = (current.0 + n.0, current.1 + n.1);
                        if nbr.0 < 0 || nbr.0 > max_x - 1 || nbr.1 < 0 || nbr.1 > max_y - 1 {
                            continue;
                        }

                        if lights[nbr.1 as usize][nbr.0 as usize] {
                            on_count += 1;
                        }
                    }

                    let current_state = lights[current.1 as usize][current.0 as usize];
                    if current_state && (on_count != 2 && on_count != 3) {
                        changes.insert(current);
                    } else if !current_state && on_count == 3 {
                        changes.insert(current);
                    }
                }
            }

            for change in &changes {
                lights[change.1 as usize][change.0 as usize] ^= true;
            }

            changes.clear();

            if corners_on {
                lights[0][0] = true;
                lights[0][(max_y - 1) as usize] = true;
                lights[(max_x - 1) as usize][(max_y - 1) as usize] = true;
                lights[(max_x - 1) as usize][0] = true;
            }
        }

        let mut count = 0;
        for x in 0..max_x as usize {
            for y in 0..max_y as usize {
                if lights[y][x] {
                    count += 1;
                }
            }
        }
        return count;
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        self.grid.clear();
        for line in raw_input.split('\n') {
            self.grid.push(line.chars().map(|x| x == '#').collect());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let lights = self.grid.clone();

        return Box::new(Self::go(lights, false));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let lights = self.grid.clone();
        return Box::new(Self::go(lights, true));
    }
}

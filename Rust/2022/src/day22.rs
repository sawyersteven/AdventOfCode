use shared::{utils::parse_to_u8_grid, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

const WALL: u8 = b'#';
const FLOOR: u8 = b'.';
const ABYSS: u8 = b' ';

pub struct Day22 {
    map: Vec<Vec<u8>>,
    path: Vec<u8>,
    sz: Vector2Int,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 {
            map: Vec::new(),
            path: Vec::new(),
            sz: Vector2Int::ZERO,
        };
    }

    fn step_to_end(&self, current_pos: &Vector2Int, direction: &Vector2Int, steps: u8) -> Vector2Int {
        let mut end_pos = *current_pos;
        let mut last_floor = *current_pos;

        let mut steps_taken = 0;
        while steps_taken < steps {
            let mut step = end_pos + *direction;
            step.x = step.x.rem_euclid(self.map[0].len() as isize);
            step.y = step.y.rem_euclid(self.map.len() as isize);

            match self.map[step.y as usize][step.x as usize] {
                WALL => {
                    // in case wall is at edge of nothing
                    return last_floor;
                }
                FLOOR => {
                    steps_taken += 1;
                    last_floor = step;
                }
                _ => {}
            }
            end_pos = step;
        }
        return end_pos;
    }

    fn step_to_end_wrapped(&self, pos: &Vector2Int, direction: usize, steps: u8) -> (Vector2Int, usize) {
        let mut d = Vector2Int::CARDINALS[direction];
        let mut pos = *pos;

        for _ in 0..steps {
            let next = pos + d;
            if next.in_range(&Vector2Int::ZERO, &(self.sz - Vector2Int::ONE))
                && self.map[next.y as usize][next.x as usize] != ABYSS
            {
                if self.map[next.y as usize][next.x as usize] == FLOOR {
                    pos = next;
                } else {
                    break;
                }
            } else {
                let (new_pos, new_dir) = self.wrap_to_next_face(&pos, &d);
                if self.map[new_pos.y as usize][new_pos.x as usize] == FLOOR {
                    d = new_dir;
                    pos = new_pos;
                } else {
                    break;
                }
            }
        }

        return (pos, Vector2Int::CARDINALS.iter().position(|x| *x == d).unwrap());
    }

    // Returns new position and direction. Its a mess, but it works
    fn wrap_to_next_face(&self, exit_pos: &Vector2Int, exit_dir: &Vector2Int) -> (Vector2Int, Vector2Int) {
        match get_face(exit_pos) {
            1 => match *exit_dir {
                Vector2Int::LEFT => (Vector2Int::new(0, 149 - exit_pos.y), Vector2Int::RIGHT),
                Vector2Int::DOWN => (Vector2Int::new(0, exit_pos.x + 100), Vector2Int::RIGHT),
                _ => unreachable!(),
            },
            2 => match *exit_dir {
                Vector2Int::RIGHT => (Vector2Int::new(99, 149 - exit_pos.y), Vector2Int::LEFT),
                Vector2Int::UP => (Vector2Int::new(99, exit_pos.x - 50), Vector2Int::LEFT),
                Vector2Int::DOWN => (Vector2Int::new(exit_pos.x - 100, 199), Vector2Int::DOWN),
                _ => unreachable!(),
            },
            3 => match *exit_dir {
                Vector2Int::RIGHT => (Vector2Int::new(exit_pos.y + 50, 49), Vector2Int::DOWN),
                Vector2Int::LEFT => (Vector2Int::new(exit_pos.y - 50, 100), Vector2Int::UP),
                _ => unreachable!(),
            },
            4 => match *exit_dir {
                Vector2Int::RIGHT => (Vector2Int::new(149, 149 - exit_pos.y), Vector2Int::LEFT),
                Vector2Int::UP => (Vector2Int::new(49, 100 + exit_pos.x), Vector2Int::LEFT),
                _ => unreachable!(),
            },
            5 => match *exit_dir {
                Vector2Int::LEFT => (Vector2Int::new(50, 149 - exit_pos.y), Vector2Int::RIGHT),
                Vector2Int::DOWN => (Vector2Int::new(50, 50 + exit_pos.x), Vector2Int::RIGHT),
                _ => unreachable!(),
            },
            6 => match *exit_dir {
                Vector2Int::RIGHT => (Vector2Int::new(exit_pos.y - 100, 149), Vector2Int::DOWN),
                Vector2Int::UP => (Vector2Int::new(exit_pos.x + 100, 0), Vector2Int::UP),
                Vector2Int::LEFT => (Vector2Int::new(exit_pos.y - 100, 0), Vector2Int::UP),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

fn get_face(pos: &Vector2Int) -> isize {
    return match pos.y / 50 {
        0 => pos.x / 50,
        1 => 3,
        2 => {
            if pos.x / 50 == 0 {
                5
            } else {
                4
            }
        }
        _ => 6,
    };
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        let parts: Vec<&str> = raw_input.split("\n\n").collect();

        self.map = parse_to_u8_grid(&parts[0].to_string(), ABYSS, 0);
        let grid_w = self.map.iter().map(|x| x.len()).max().unwrap();
        for row in self.map.iter_mut() {
            let p = grid_w - row.len();
            for _ in 0..p {
                row.push(ABYSS);
            }
        }

        let mut dirs = parts[1].as_bytes().iter().filter(|x| **x == b'L' || **x == b'R');
        let mut lens = parts[1].split(|x: char| x == 'L' || x == 'R');
        self.path.push(lens.next().unwrap().parse().unwrap());

        while let Some(dir) = dirs.next() {
            self.path.push(*dir);
            self.path.push(lens.next().unwrap().parse().unwrap());
        }

        self.sz = Vector2Int::new(grid_w as isize, self.map.len() as isize);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut position = Vector2Int::ZERO;
        position.x = self.map[0].iter().position(|x| *x == b'.').unwrap() as isize;

        let mut current_dir = 1;

        for i in 0..self.path.len() {
            if i % 2 == 0 {
                position = self.step_to_end(&position, &Vector2Int::CARDINALS[current_dir], self.path[i]);
            } else {
                // rotate backward because -Y is *up* on the map
                current_dir = current_dir + if self.path[i] == b'R' { 3 } else { 1 };
                current_dir %= Vector2Int::CARDINALS.len();
            }
        }
        // Eric indexes maps starting at 1
        position += Vector2Int::ONE;
        // He also starts cardinals directions with RIGHT
        let eric_dir = ((current_dir - 1) % 4) as isize;
        return Box::new(1000 * position.y + 4 * position.x + eric_dir);
    }

    /*
    A general-purpose solution to this is outside of my capabilities and/or
    patience. All of the inputs I've seen have the same pattern, so this might
    be sufficient for everyone:

    ..1122
    ..1122
    ..33..
    ..33..
    4455..
    4455..
    66....
    66....

    The core of the problem is the relationship between the edges of each face.

    Travelling left off the edge of 1 results in travelling right from the left
    edge of face 4. The method "wrap to next face" calculates the new position
    and facing direction that would result from walking over the edge of any
    given location on an edge. If the cube net was in any other shape this would
    have to be modified.

    */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut position = Vector2Int::ZERO;
        position.x = self.map[0].iter().position(|x| *x == b'.').unwrap() as isize;
        let mut current_dir = 1;

        for i in 0..self.path.len() {
            if i % 2 == 0 {
                (position, current_dir) = self.step_to_end_wrapped(&position, current_dir, self.path[i]);
            } else {
                current_dir = current_dir + if self.path[i] == b'R' { 3 } else { 1 };
                current_dir %= Vector2Int::CARDINALS.len();
            }
        }
        // Eric indexes maps starting at 1
        position += Vector2Int::ONE;
        // He also starts cardinals directions with RIGHT
        let eric_dir = (current_dir as isize - 1) % 4;
        return Box::new(1000 * position.y + 4 * position.x + eric_dir);
    }
}

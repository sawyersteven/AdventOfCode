use crypto::{digest::Digest, md5::Md5};
use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::VecDeque, fmt::Display};

const MAPSIZE: isize = 4;
const START: Vector2Int = Vector2Int { x: 1, y: 1 };
const END: Vector2Int = Vector2Int { x: MAPSIZE, y: MAPSIZE };
const DIR_CARS: [char; 4] = ['U', 'D', 'L', 'R'];
const DIR_VECS: [Vector2Int; 4] = [
    Vector2Int { x: 0, y: -1 },
    Vector2Int { x: 0, y: 1 },
    Vector2Int { x: -1, y: 0 },
    Vector2Int { x: 1, y: 0 },
];

pub struct Day17 {
    pub input: String,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 {
            input: String::from("qzthpkfp"),
        };
    }
}

impl Base for Day17 {
    #[allow(unused)]
    fn parse_input(&mut self, raw_input: String) {
        //self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<(Vector2Int, String)>::new();
        q.push_back((START, String::new()));

        let mut hasher = Md5::new();

        while q.len() > 0 {
            let (location, steps) = q.pop_front().unwrap();
            if location == END {
                return Box::new(steps);
            }

            hasher.input_str(&format!("{}{}", self.input, steps));
            let hash_str = hasher.result_str();
            let hash = hash_str.as_bytes();
            hasher.reset();

            for i in 0..4 {
                if hash[i] >= b'b' && hash[i] <= b'f' {
                    let next = location + DIR_VECS[i];
                    if next.x == 0 || next.y == 0 || next.x > MAPSIZE || next.y > MAPSIZE {
                        continue;
                    }

                    let next_steps = format!("{}{}", steps, DIR_CARS[i]);
                    q.push_back((next, next_steps));
                }
            }
        }

        return Box::new("-");
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<(Vector2Int, String)>::new();
        q.push_back((START, String::new()));
        let mut longest = 0;

        let mut hasher = Md5::new();

        while q.len() > 0 {
            let (location, steps) = q.pop_front().unwrap();
            if location == END {
                longest = longest.max(steps.len());
                continue;
            }

            hasher.input_str(&format!("{}{}", self.input, steps));
            let hash_str = hasher.result_str();
            let hash = hash_str.as_bytes();
            hasher.reset();

            for i in 0..4 {
                if hash[i] >= b'b' && hash[i] <= b'f' {
                    let next = location + DIR_VECS[i];
                    if next.x == 0 || next.y == 0 || next.x > MAPSIZE || next.y > MAPSIZE {
                        continue;
                    }

                    let next_steps = format!("{}{}", steps, DIR_CARS[i]);
                    q.push_back((next, next_steps));
                }
            }
        }
        return Box::new(longest);
    }
}

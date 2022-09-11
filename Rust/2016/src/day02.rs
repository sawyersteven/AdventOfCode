use shared::v2i::Vector2Int;

use crate::Base;
use std::fmt::Display;

const KEYPAD: [[char; 7]; 7] = [
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
    [' ', ' ', ' ', '1', ' ', ' ', ' '],
    [' ', ' ', '2', '3', '4', ' ', ' '],
    [' ', '5', '6', '7', '8', '9', ' '],
    [' ', ' ', 'A', 'B', 'C', ' ', ' '],
    [' ', ' ', ' ', 'D', ' ', ' ', ' '],
    [' ', ' ', ' ', ' ', ' ', ' ', ' '],
];

pub struct Day02 {
    pub input: String,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 {
            input: String::from(""),
        };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut code: Vec<usize> = Vec::new();
        let mut key = 5;
        for line in self.input.split('\n') {
            for c in line.chars() {
                match c {
                    'L' => {
                        if key % 3 == 1 {
                            continue;
                        }
                        key -= 1;
                    }
                    'R' => {
                        if key % 3 == 0 {
                            continue;
                        }
                        key += 1;
                    }
                    'U' => {
                        if key <= 3 {
                            continue;
                        }
                        key -= 3;
                    }
                    'D' => {
                        if key >= 7 {
                            continue;
                        }
                        key += 3;
                    }
                    _ => panic!(),
                }
            }
            code.push(key);
        }

        return Box::new(format!("{:?}", code));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut code: Vec<char> = Vec::new();
        let mut location = Vector2Int::new(1, 3);

        for line in self.input.split('\n') {
            for c in line.chars() {
                match c {
                    'L' => {
                        if KEYPAD[location.y as usize][(location.x - 1) as usize] != ' ' {
                            location.x -= 1;
                        }
                    }
                    'R' => {
                        if KEYPAD[location.y as usize][(location.x + 1) as usize] != ' ' {
                            location.x += 1;
                        }
                    }
                    'U' => {
                        if KEYPAD[(location.y - 1) as usize][location.x as usize] != ' ' {
                            location.y -= 1;
                        }
                    }
                    'D' => {
                        if KEYPAD[(location.y + 1) as usize][location.x as usize] != ' ' {
                            location.y += 1;
                        }
                    }
                    _ => panic!(),
                }
            }
            code.push(KEYPAD[location.y as usize][location.x as usize]);
        }

        return Box::new(format!("{:?}", code));
    }
}

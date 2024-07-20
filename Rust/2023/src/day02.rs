use crate::Base;
use core::panic;
use std::{collections::HashMap, fmt::Display};

pub struct Day02 {
    pub input: Vec<String>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.lines().map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let maximums = HashMap::from([
            ("red".to_string(), 12),
            ("green".to_string(), 13),
            ("blue".to_string(), 14),
        ]);

        let mut sum = 0;
        for (id, game) in self.input.iter().enumerate() {
            let mut is_valid = true;
            for cube in game.split(": ").nth(1).unwrap().replace(';', ",").split(", ") {
                let (count, color) = cube.split_once(' ').unwrap();
                if count.parse::<usize>().unwrap() > maximums[color.trim()] {
                    is_valid = false;
                    break;
                }
            }
            sum += if is_valid { id + 1 } else { 0 };
        }
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for game in &self.input {
            let mut color_mins = [0, 0, 0];
            for cube in game.split(": ").nth(1).unwrap().replace(';', ",").split(", ") {
                let (count, color) = cube.split_once(' ').unwrap();
                let i = match color.trim() {
                    "red" => 0,
                    "blue" => 1,
                    "green" => 2,
                    _ => panic!(),
                };
                color_mins[i] = color_mins[i].max(count.parse::<usize>().unwrap());
            }
            sum += color_mins[0] * color_mins[1] * color_mins[2];
        }
        return Box::new(sum);
    }
}

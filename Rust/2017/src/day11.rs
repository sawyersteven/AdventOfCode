use shared::v3i::Vector3Int;

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day11 {
    pub input: String,
    d3: HashMap<String, Vector3Int>,
}

impl Day11 {
    pub fn new() -> Day11 {
        // https://www.redblobgames.com/grids/hexagons/#coordinates-cube
        let mut d3: HashMap<String, Vector3Int> = HashMap::new();
        d3.insert(String::from("n"), Vector3Int::new(0, 1, -1));
        d3.insert(String::from("s"), Vector3Int::new(0, -1, 1));
        d3.insert(String::from("nw"), Vector3Int::new(-1, 1, 0));
        d3.insert(String::from("ne"), Vector3Int::new(1, 0, -1));
        d3.insert(String::from("se"), Vector3Int::new(1, -1, 0));
        d3.insert(String::from("sw"), Vector3Int::new(-1, 0, 1));

        return Day11 {
            input: String::from(""),
            d3: d3,
        };
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut end = Vector3Int::new(0, 0, 0);
        for s in self.input.split(',') {
            end += self.d3[s];
        }

        return Box::new(end.manhattan(Vector3Int::new(0, 0, 0)) / 2);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut current = Vector3Int::new(0, 0, 0);
        let mut farthest = Vector3Int::new(0, 0, 0);
        let mut farthest_dist = 0;

        for s in self.input.split(',') {
            current += self.d3[s];

            let d = current.manhattan(Vector3Int::new(0, 0, 0));
            if d > farthest_dist {
                farthest_dist = d;
                farthest = current;
            }
        }

        return Box::new(farthest.manhattan(Vector3Int::new(0, 0, 0)) / 2);
    }
}

use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day22 {
    pub nodes: HashMap<Vector2Int, (isize, isize)>,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 { nodes: HashMap::new() };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n').skip(2) {
            let parts: Vec<&str> = line.split(' ').filter(|x| *x != "").collect();
            let coords: Vec<isize> = parts[0]
                .split('x')
                .nth(1)
                .unwrap()
                .split("-y")
                .map(|x| x.parse().unwrap())
                .collect();

            let v = Vector2Int::new(coords[0], coords[1]);

            let ud: isize = parts[2][0..(parts[2].len() - 1)].parse().unwrap();
            let av: isize = parts[3][0..(parts[3].len() - 1)].parse().unwrap();
            self.nodes.insert(v, (ud, av));
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut count = 0;

        let values: Vec<&(isize, isize)> = self.nodes.values().collect();

        for a in 0..(values.len() - 1) {
            for b in (a + 1)..values.len() {
                if values[a].0 != 0 && values[a].0 <= values[b].1 {
                    count += 1;
                }
                if values[b].0 != 0 && values[b].0 <= values[a].1 {
                    count += 1;
                }
            }
        }

        return Box::new(count);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("Topaz (AoC creator) said 2016.22.02 was meant to be solved without code. So if he can't be bothered I can't either. The answer is 215.");
    }
}

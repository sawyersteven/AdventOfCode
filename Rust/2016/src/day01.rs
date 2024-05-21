use crate::Base;
use shared::v2i::Vector2Int;
use std::{collections::HashSet, fmt::Display};

pub struct Day01 {
    pub input: Vec<String>,
}

impl Day01 {
    pub fn new() -> Day01 {
        return Day01 { input: Vec::new() };
    }
}

impl Base for Day01 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split(", ").map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut dir = 2;
        let mut location = Vector2Int::ZERO;

        for s in &self.input {
            dir += if &s[0..1] == "R" { 1 } else { 3 };
            dir %= 4;
            location += Vector2Int::CARDINALS[dir] * s[1..].parse::<isize>().unwrap();
        }

        return Box::new(location.manhattan(Vector2Int::ZERO));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut history: HashSet<Vector2Int> = HashSet::new();

        let mut dir = 2;
        let mut location = Vector2Int::ZERO;

        for s in &self.input {
            dir += if s.chars().nth(0).unwrap() == 'R' { 1 } else { 3 };
            dir %= 4;
            let dist = s[1..].parse::<isize>().unwrap();
            for _ in 0..dist {
                location += Vector2Int::CARDINALS[dir];
                if history.contains(&location) {
                    return Box::new(location.manhattan(Vector2Int::ZERO));
                }
                history.insert(location);
            }
        }

        return Box::new("-");
    }
}

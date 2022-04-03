use crate::Base;
use shared::v2i;
use shared::v2i::Vector2Int;
use std::{collections::HashSet, fmt::Display};

pub struct Day03 {
    pub input: Vec<char>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 { input: Vec::new() };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut visited: HashSet<Vector2Int> = HashSet::new();
        let mut current = Vector2Int::new(0, 0);

        for c in &self.input {
            current += match c {
                '^' => v2i::UP,
                'v' => v2i::DOWN,
                '<' => v2i::LEFT,
                '>' => v2i::RIGHT,
                _ => panic!(),
            };
            visited.insert(current);
        }

        return Box::new(visited.len());
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut visited = HashSet::<Vector2Int>::new();
        let mut santa = Vector2Int::new(0, 0);
        let mut robo_santa = Vector2Int::new(0, 0);

        let mut actor: &mut Vector2Int;
        for (i, c) in (&self.input).iter().enumerate() {
            if i % 2 == 0 {
                actor = &mut santa;
            } else {
                actor = &mut robo_santa;
            }
            *actor += match c {
                '^' => v2i::UP,
                'v' => v2i::DOWN,
                '<' => v2i::LEFT,
                '>' => v2i::RIGHT,
                _ => panic!(),
            };

            visited.insert(*actor);
        }
        return Box::new(visited.len());
    }
}

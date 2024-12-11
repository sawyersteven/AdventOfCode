use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub struct Day10 {
    input: String,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { input: String::new() };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let map = Grid2D::<u8>::from_string(&self.input);

        let mut score = 0;
        for v in map.find_all(&b'0') {
            score += follow_trail(&map, Vector2Int::from(v));
        }

        return Box::new(score);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let map = Grid2D::<u8>::from_string(&self.input);

        let mut score = 0;
        for v in map.find_all(&b'0') {
            let start = Vector2Int::from(v);
            score += follow_trail_2(&map, start);
        }

        return Box::new(score);
    }
}

fn follow_trail(map: &Grid2D<u8>, start: Vector2Int) -> usize {
    let mut q = VecDeque::new();
    let mut visited = HashSet::new();

    let sz = map.size().to_vector2() - Vector2Int::ONE;
    q.push_back((start, b'0'));

    let mut count = 0;
    while let Some((cur, next_val)) = q.pop_back() {
        let next_val = next_val + 1;

        if map[&cur] == b'9' && visited.insert(cur) {
            count += 1;
            continue;
        }

        for next in Vector2Int::CARDINALS
            .iter()
            .map(|c| *c + cur)
            .filter(|v| v.in_range(&Vector2Int::ZERO, &sz) && map[v] == next_val)
        {
            q.push_back((next, next_val));
        }
    }

    return count;
}

fn follow_trail_2(map: &Grid2D<u8>, start: Vector2Int) -> usize {
    let mut q = VecDeque::new();

    let sz = map.size().to_vector2() - Vector2Int::ONE;
    q.push_back(start);

    let mut count = 0;
    while let Some(cur) = q.pop_back() {
        let next_val = map[&cur] + 1;
        for next in Vector2Int::CARDINALS
            .iter()
            .map(|c| *c + cur)
            .filter(|v| v.in_range(&Vector2Int::ZERO, &sz) && map[v] == next_val)
        {
            if map[&next] == b'9' {
                count += 1;
                continue;
            }
            q.push_back(next);
        }
    }

    return count;
}

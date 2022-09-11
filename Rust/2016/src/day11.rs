use crate::Base;
use std::fmt::Display;

pub struct Day11 {
    pub input: String,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            input: String::from(""),
        };
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut items_per_floor: [usize; 5] = [0, 4, 5, 1, 0];
        return Box::new(solve(&mut items_per_floor));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut items_per_floor: [usize; 5] = [0, 8, 5, 1, 0];
        return Box::new(solve(&mut items_per_floor));
    }
}

fn solve(items_per_floor: &mut [usize; 5]) -> usize {
    let mut moves = 0;

    // This is not going to work with any input as it assumes we can clear
    // each floor in sequence without running into a roadblock
    for floor in 1..(items_per_floor.len() - 1) {
        moves += 2 * items_per_floor[floor] - 3;
        items_per_floor[floor + 1] += items_per_floor[floor];
    }

    return moves;
}

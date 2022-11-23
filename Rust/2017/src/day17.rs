use shared::circular_deque::CircularDeque;

use crate::Base;
use std::fmt::Display;

pub struct Day17 {
    pub input: String,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 {
            input: String::from(""),
        };
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let rot_steps = self.input.parse().unwrap();
        let mut circle = CircularDeque::<usize>::new();
        circle.add_last(0);
        for i in 1..2018 {
            circle.move_head_right(rot_steps);
            circle.insert_after(0, i);
            circle.move_head_right(1);
        }

        return Box::new(circle.to_vec()[1]);
    }

    fn part2(&self) -> Box<dyn Display> {
        let rot_steps: usize = self.input.parse().unwrap();
        let mut zero_pos = 0;
        let mut second_val = 0;
        for i in 1..50_000_000 {
            zero_pos = (zero_pos + rot_steps + 1) % i;
            if zero_pos == 0 {
                second_val = i;
            }
        }
        return Box::new(second_val);
    }
}

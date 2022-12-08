use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    pub input: Vec<(u8, u8)>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input
            .split('\n')
            .map(|x| (x.as_bytes()[0] - b'A', x.as_bytes()[2] - b'X'))
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for (opponent, player) in &self.input {
            if opponent == player {
                total += 3;
            }
            if (opponent + 1) % 3 == *player {
                total += 6
            }
            total += 1 + *player as usize;
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total: usize = 0;
        for (opponent, state) in &self.input {
            total += match state {
                0 => ((*opponent as usize + 2) % 3) + 1,
                1 => 3 + *opponent as usize + 1,
                2 => 6 + ((*opponent as usize + 1) % 3) + 1,
                _ => panic!(),
            }
        }
        return Box::new(total);
    }
}

use crate::Base;
use std::{collections::VecDeque, fmt::Display};

const KEY: isize = 811589153;

pub struct Day20 {
    pub input: Vec<isize>,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 { input: Vec::new() };
    }

    fn mix(&self, mixer: &mut VecDeque<(usize, isize)>) {
        for i in 0..mixer.len() {
            let pos = mixer.iter().position(|(pos, _)| *pos == i).unwrap();
            mixer.rotate_left(pos);

            let (og_position, value) = mixer.pop_front().unwrap();
            if value < 0 {
                mixer.rotate_right((value.abs() % mixer.len() as isize) as usize);
            } else {
                mixer.rotate_left((value % mixer.len() as isize) as usize);
            }
            mixer.push_front((og_position, value));
        }
    }

    fn sum_mixer(&self, mixer: &mut VecDeque<(usize, isize)>) -> isize {
        let zero_pos = mixer.iter().position(|(_, value)| *value == 0).unwrap();
        mixer.rotate_left(zero_pos);
        let mut sum = 0;
        for i in [1000usize, 2000, 3000] {
            sum += mixer[i % mixer.len()].1;
        }
        return sum;
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.lines().map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        // (original_position, &value)
        let mut mixer = VecDeque::from_iter(self.input.iter().cloned().enumerate());

        self.mix(&mut mixer);

        return Box::new(self.sum_mixer(&mut mixer));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut mixer = VecDeque::from_iter(self.input.iter().map(|x| x * KEY).enumerate());

        for _ in 0..10 {
            self.mix(&mut mixer);
        }

        return Box::new(self.sum_mixer(&mut mixer));
    }
}

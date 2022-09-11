use crate::Base;
use cdll::CircularList;
use std::fmt::Display;

pub struct Day19 {
    pub elf_count: usize,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 { elf_count: 0 };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        self.elf_count = raw_input.parse().unwrap();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut last = 0;
        for i in 1..self.elf_count {
            last = (last + 2) % (i + 1);
        }

        return Box::new(last - 1);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut elves = CircularList::default();

        for i in 1..self.elf_count {
            elves.add(i);
        }

        elves.left_rot(elves.len() / 2 + 1);

        while elves.len() > 1 {
            elves.remove().unwrap();
            if elves.len() % 2 == 0 {
                elves.left_rot(1);
            }
        }

        return Box::new(elves.remove().unwrap()); // 1423634
    }
}

use crate::Base;
use std::fmt::Display;

pub struct Day25 {
    input: String,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { input: String::new() };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut locks = Vec::new();
        let mut keys = Vec::new();

        for block in self.input.split("\n\n") {
            let is_lock: bool;
            let mut heights: [i8; 5];

            let mut lines = block.lines();

            if &lines.next().unwrap()[0..1] == "#" {
                heights = [0i8; 5];
                is_lock = true;
            } else {
                heights = [-1i8; 5];
                is_lock = false;
            }

            while let Some(line) = lines.next() {
                let b = line.as_bytes();
                for i in 0..5 {
                    if b[i] == b'#' {
                        heights[i] += 1;
                    }
                }
            }
            if is_lock {
                locks.push(heights);
            } else {
                keys.push(heights);
            }
        }

        let mut count = 0;
        for lock in &locks {
            for key in &keys {
                if fit(lock, key) {
                    count += 1;
                }
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("🌟");
    }
}

#[inline(always)]
fn fit(lock: &[i8; 5], key: &[i8; 5]) -> bool {
    for i in 0..5 {
        if lock[i] + key[i] >= 6 {
            return false;
        }
    }
    return true;
}

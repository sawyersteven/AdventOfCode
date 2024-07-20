use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day04 {
    pub input: Vec<(HashSet<usize>, HashSet<usize>)>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { input: Vec::new() };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let line = line.replace("  ", " ");
            let (a, b) = line.split(":").nth(1).unwrap().split_once(" | ").unwrap();
            let a_set = HashSet::from_iter(a.trim().split(" ").map(|x| x.parse().unwrap()));
            let b_set = HashSet::from_iter(b.trim().split(" ").map(|x| x.parse().unwrap()));
            self.input.push((a_set, b_set));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for card in &self.input {
            let matches = card.0.intersection(&card.1).count();
            if matches > 0 {
                sum += 2_usize.pow(matches as u32 - 1);
            }
        }
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut card_counts = vec![1; self.input.len()];

        for (i, card) in self.input.iter().enumerate() {
            let matches = card.0.intersection(&card.1).count();
            for ii in (i + 1)..(i + matches + 1) {
                card_counts[ii] += card_counts[i];
            }
        }
        return Box::new(card_counts.iter().sum::<usize>());
    }
}

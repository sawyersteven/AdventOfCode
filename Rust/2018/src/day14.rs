use shared::utils::read_user_input;

use crate::Base;
use std::fmt::Display;

pub struct Day14 {
    turns: usize,
    score_seq: Vec<usize>,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 {
            turns: 0,
            score_seq: Vec::new(),
        };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        self.turns = raw_input.split('\n').nth(0).unwrap().parse().unwrap();
        for i in raw_input.split('\n').nth(0).unwrap().chars() {
            self.score_seq.push(i.to_digit(10).unwrap() as usize);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut rp = RecipeMaker::new();
        while rp.recipe_scores.len() < (self.turns + 10) {
            rp.simulate_round();
        }
        let slice: String = rp.recipe_scores[self.turns..(self.turns + 10)]
            .iter()
            .map(|x| x.to_string())
            .collect();
        return Box::new(slice);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut rp = RecipeMaker::new();
        while (rp.recipe_scores.len() as isize) - 1 - (self.score_seq.len() as isize) < 0 {
            rp.simulate_round();
        }
        let mut last_len = rp.recipe_scores.len();
        loop {
            rp.simulate_round();

            // this mess is much quicker than comparing a List<T>.GetRange()
            let mut is_match = true;
            for i in 1..=self.score_seq.len() {
                is_match &=
                    rp.recipe_scores[rp.recipe_scores.len() - i] == self.score_seq[self.score_seq.len() - i];
                if !is_match {
                    break;
                }
            }
            if is_match {
                return Box::new(rp.recipe_scores.len() - self.score_seq.len());
            }

            if rp.recipe_scores.len() - last_len == 2 {
                is_match = true;
                for i in 1..=self.score_seq.len() {
                    is_match &= rp.recipe_scores[rp.recipe_scores.len() - (i + 1)]
                        == self.score_seq[self.score_seq.len() - i];
                    if !is_match {
                        break;
                    }
                }
                if is_match {
                    return Box::new(rp.recipe_scores.len() - self.score_seq.len() - 1);
                }
            }
            last_len = rp.recipe_scores.len();
        }
        panic!();
    }
}

struct RecipeMaker {
    elf_a: usize,
    elf_b: usize,
    recipe_scores: Vec<usize>,
}

impl RecipeMaker {
    pub fn new() -> Self {
        return RecipeMaker {
            elf_a: 0,
            elf_b: 1,
            recipe_scores: vec![3, 7],
        };
    }

    pub fn simulate_round(&mut self) {
        let new_recipe = self.recipe_scores[self.elf_a] + self.recipe_scores[self.elf_b];
        if new_recipe >= 10 {
            self.recipe_scores.push(1);
            self.recipe_scores.push(new_recipe - 10);
        } else {
            self.recipe_scores.push(new_recipe);
        }

        self.elf_a = (self.elf_a + self.recipe_scores[self.elf_a] + 1) % self.recipe_scores.len();
        self.elf_b = (self.elf_b + self.recipe_scores[self.elf_b] + 1) % self.recipe_scores.len();
    }
}

use crate::Base;
use std::fmt::Display;

const ATTR_COUNT: usize = 5;

pub struct Day15 {
    pub input: String,
    ingredients: Vec<[isize; ATTR_COUNT]>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 {
            input: String::from(""),
            ingredients: Vec::new(),
        };
    }

    fn score_recipe(&self, tsps: &[isize; 4]) -> isize {
        let mut score = 1;

        for attr in 0..ATTR_COUNT - 1 {
            let mut attr_score = 0;

            for ingredient in 0..self.ingredients.len() {
                let s = tsps[ingredient] * self.ingredients[ingredient][attr];
                attr_score += s;
            }
            if attr_score <= 0 {
                return 0;
            }
            score *= attr_score;
        }

        return score;
    }

    fn score_recipe_500(&self, tsps: &[isize; 4]) -> isize {
        let mut cals = 0;
        for i in 0..4 {
            cals += self.ingredients[i][ATTR_COUNT - 1] * tsps[i];
        }
        if cals != 500 {
            return 0;
        }
        return self.score_recipe(tsps);
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        for line in lines {
            let parts: Vec<&str> = line.split(|c| c == ' ' || c == ',').collect();
            let l = [
                parts[2].parse().unwrap(),
                parts[5].parse().unwrap(),
                parts[8].parse().unwrap(),
                parts[11].parse().unwrap(),
                parts[14].parse().unwrap(),
            ];
            self.ingredients.push(l);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const TOTAL: isize = 100;

        let mut best = 0;

        for a in (1..TOTAL).take_while(|i| i < &TOTAL) {
            for b in (1..TOTAL).take_while(|i| i < &(TOTAL - a)) {
                for c in (1..TOTAL).take_while(|i| i < &(TOTAL - a - b)) {
                    let d = TOTAL - a - b - c;

                    let score = Day15::score_recipe(self, &[a, b, c, d]);
                    if score > best {
                        best = score;
                    }
                }
            }
        }

        return Box::new(best);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const TOTAL: isize = 100;

        let mut best = 0;

        for a in (1..TOTAL).take_while(|i| i < &TOTAL) {
            for b in (1..TOTAL).take_while(|i| i < &(TOTAL - a)) {
                for c in (1..TOTAL).take_while(|i| i < &(TOTAL - a - b)) {
                    let d = TOTAL - a - b - c;

                    let score = Day15::score_recipe_500(self, &[a, b, c, d]);
                    if score > best {
                        best = score;
                    }
                }
            }
        }

        return Box::new(best);
    }
}

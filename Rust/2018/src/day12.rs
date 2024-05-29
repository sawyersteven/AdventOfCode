use crate::Base;
use std::{collections::HashSet, fmt::Display};

const SLICEW: usize = 5;
const PLANT: u8 = b'#';
const EMPTY: u8 = b'.';

pub struct Day12 {
    init_state: Vec<u8>,
    patterns: HashSet<String>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 {
            init_state: Vec::new(),
            patterns: HashSet::new(),
        };
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        self.init_state = lines[0].split(": ").nth(1).unwrap().as_bytes().to_vec();

        for i in 2..lines.len() {
            let parts: Vec<&str> = lines[i].split(" => ").collect();
            if parts[1].as_bytes()[0] == EMPTY {
                continue;
            }
            self.patterns.insert(parts[0].to_string());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const EXTRA_POTS: usize = 30; // this is mostly arbitrary
        let mut pots = [
            vec![EMPTY; EXTRA_POTS].as_slice(),
            self.init_state.as_slice(),
            vec![EMPTY; EXTRA_POTS].as_slice(),
        ]
        .concat();

        self.run_generation(&mut pots, 20);

        return Box::new(self.count_plants(&pots, EXTRA_POTS));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const SAMPLE_COUNT: usize = 50;
        const SAMPLE_RATE: usize = 10;

        const EXTRA_POTS: usize = 550; // this is mostly arbitrary
        let mut pots = [
            vec![EMPTY; EXTRA_POTS].as_slice(),
            self.init_state.as_slice(),
            vec![EMPTY; EXTRA_POTS].as_slice(),
        ]
        .concat();

        let mut results = Vec::new();
        for _ in 0..SAMPLE_COUNT {
            self.run_generation(&mut pots, SAMPLE_RATE);
            results.push(self.count_plants(&pots, EXTRA_POTS));
        }

        let stable_growth_rate = results[results.len() - 1] - results[results.len() - 2];

        for i in 0..10 {
            let sgr = results[results.len() - 1 - i] - results[results.len() - i - 2];
            if sgr != stable_growth_rate {
                return Box::new("Stable growth rate not reached -- increase SAMPLE_COUNT");
            }
        }

        let per_sample_rate = stable_growth_rate / SAMPLE_RATE;
        let sim_len = SAMPLE_COUNT * SAMPLE_RATE;

        let answer = results[results.len() - 1] + ((50_000_000_000 - sim_len) * per_sample_rate);

        return Box::new(answer);
    }
}

impl Day12 {
    fn count_plants(&self, pots: &Vec<u8>, extra_pots: usize) -> usize {
        let mut total = 0;
        for i in 0..pots.len() {
            if pots[i] == PLANT {
                total += i - extra_pots;
            }
        }
        return total;
    }

    fn run_generation(&self, pots: &mut Vec<u8>, iters: usize) {
        let mut next_gen = vec![EMPTY; pots.len()];
        for _ in 0..iters {
            for i in 2..(pots.len() - 2) {
                let slice = &pots[(i - 2)..(i - 2 + SLICEW)]
                    .iter()
                    .map(|x| *x as char)
                    .collect::<String>();
                if self.patterns.contains(slice) {
                    next_gen[i] = PLANT;
                } else {
                    next_gen[i] = EMPTY;
                }
            }

            for i in 0..next_gen.len() {
                pots[i] = next_gen[i];
                next_gen[i] = EMPTY;
            }
        }
    }
}

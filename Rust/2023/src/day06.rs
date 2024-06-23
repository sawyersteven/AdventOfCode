use crate::Base;
use std::fmt::Display;

pub struct Day06 {
    races: Vec<(usize, usize)>,
    race2: (usize, usize),
}

impl Day06 {
    pub fn new() -> Day06 {
        return Day06 {
            races: Vec::new(),
            race2: (0, 0),
        };
    }
}

impl Base for Day06 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<Vec<usize>> = raw_input
            .split('\n')
            .map(|x| {
                x.split(':')
                    .nth(1)
                    .unwrap()
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect()
            })
            .collect();

        for i in 0..lines[0].len() {
            self.races.push((lines[0][i], lines[1][i]));
        }
        let r2time = lines[0]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();
        let r2rec = lines[1]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse()
            .unwrap();

        self.race2 = (r2time, r2rec);

        //.parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut prod = 1;

        for (time, record) in &self.races {
            let mut wins = 0;
            for hold in 1..*time {
                if (time - hold) * hold > *record {
                    wins += 1;
                }
            }
            prod *= wins;
        }

        return Box::new(prod);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut wins = 0;
        let (time, record) = self.race2;
        for hold in 1..time {
            wins += if (time - hold) * hold > record { 1 } else { 0 };
        }
        return Box::new(wins);
    }
}

use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day10 {
    pub input: Vec<String>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { input: Vec::new() };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut q = VecDeque::<isize>::new();
        let mut swap_rules = HashMap::<isize, (isize, isize)>::new();
        let mut bots = HashMap::<isize, Vec<isize>>::new();

        for line in &self.input {
            let parts: Vec<&str> = line.split(' ').collect();
            let p1: isize = parts[1].parse().unwrap();
            if parts[0] == "bot" {
                let mut low_target: isize = parts[6].parse().unwrap();
                if parts[5] == "output" {
                    low_target -= 100;
                }
                swap_rules.insert(p1, (low_target, parts.last().unwrap().parse().unwrap()));
            } else {
                let p2: isize = parts.last().unwrap().parse().unwrap();

                bots.entry(p2).or_insert(Vec::with_capacity(2)).push(p1);

                if bots[&p2].len() == 2 {
                    q.push_back(p2);
                }
            }
        }

        let mut ans = 0;
        while q.len() > 0 {
            let bot = q.pop_front().unwrap();

            bots.entry(bot).and_modify(|f| f.sort());

            if bots[&bot][0] == 17 && bots[&bot][1] == 61 {
                ans = bot;
            }

            let (low, high) = swap_rules[&bot];

            let ins = bots[&bot][0];
            bots.entry(low).or_insert(Vec::with_capacity(2)).push(ins);
            if bots[&low].len() == 2 {
                q.push_back(low);
            }

            let ins = bots[&bot][1];
            bots.entry(high).or_insert(Vec::with_capacity(2)).push(ins);
            if bots[&high].len() == 2 {
                q.push_back(high);
            }
        }

        // doing part 2 here because hashmaps can't be static
        let pt2 = bots[&-100][0] * bots[&-99][0] * bots[&-98][0];

        return Box::new(format!("{}\t{}", ans, pt2));
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("─────┴─────┘");
    }
}

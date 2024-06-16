use crate::Base;
use std::fmt::Display;

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

pub struct Day19 {
    pub input: String,
    blueprints: Vec<Blueprint>,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            input: String::from(""),
            blueprints: Vec::new(),
        };
    }

    fn find_best(&self, bp: &Blueprint, max_time: u16) -> usize {
        let init_state = State::new();
        let mut max_bots = [0; 4];
        for i in 0..3 {
            for mats in bp {
                max_bots[i] = max_bots[i].max(mats[i]);
            }
        }
        max_bots[GEODE] = u16::MAX;

        return Day19::_find_best(init_state, bp, &max_bots, max_time) as usize;
    }

    fn _find_best(state: State, bp: &Blueprint, max_bots: &[u16; 4], max_time: u16) -> u16 {
        let mut best = 0;
        for bot_type in 0..4 {
            if state.bots[bot_type] == max_bots[bot_type] {
                continue;
            }
            let build_cost = &bp[bot_type];

            let ttb = time_to_build(&state.materials, &state.bots, build_cost);

            if state.minute + ttb + 1 >= max_time {
                continue;
            }

            let mut next = state.clone();
            next.minute += ttb + 1;

            for i in 0..state.bots.len() {
                next.materials[i] += state.bots[i] * (ttb + 1) - build_cost[i];
            }

            next.bots[bot_type] += 1;

            best = best.max(Day19::_find_best(next, bp, max_bots, max_time));
        }

        let geodes = state.materials[GEODE] + state.bots[GEODE] * (max_time - state.minute);
        best = best.max(geodes);

        return best;
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        // Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 4 ore and 8 clay. Each geode robot costs 2 ore and 10 obsidian.
        for line in raw_input.lines() {
            let mut bp: Blueprint = [[0; 4]; 4];
            let words: Vec<&str> = line.split(' ').collect();
            bp[ORE][ORE] = words[6].parse().unwrap();
            bp[CLAY][ORE] = words[12].parse().unwrap();
            bp[OBSIDIAN][ORE] = words[18].parse().unwrap();
            bp[OBSIDIAN][CLAY] = words[21].parse().unwrap();
            bp[GEODE][ORE] = words[27].parse().unwrap();
            bp[GEODE][OBSIDIAN] = words[30].parse().unwrap();
            self.blueprints.push(bp);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for i in 0..self.blueprints.len() {
            total += (i + 1) * self.find_best(&self.blueprints[i], 24);
        }

        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        // This is super slow with the test data
        // Only kind of slow with the real data
        let mut prod = 1;
        for i in 0..3 {
            prod *= self.find_best(&self.blueprints[i], 32);
        }

        return Box::new(prod);
    }
}

fn time_to_build(mats: &Materials, bots: &[u16; 4], cost: &Materials) -> u16 {
    let mut time = 0;
    for i in 0..4 {
        // already have what we need
        if cost[i] <= mats[i] {
            time = time.max(0);
        }
        // we can't produce what this build requires in time
        else if bots[i] == 0 {
            time = time.max(u16::MAX / 2);
        } else {
            time = time.max((cost[i] - mats[i]).div_ceil(bots[i]));
        }
    }
    return time;
}

// Nested array usage: Blueprint[Bot_type][Material_type] eg bp[OBSIDIAN][ORE]
type Blueprint = [Materials; 4];
type Materials = [u16; 4];

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    minute: u16,
    bots: [u16; 4],
    materials: [u16; 4],
}

impl State {
    pub fn new() -> Self {
        return State {
            minute: 0,
            bots: [1, 0, 0, 0],
            materials: [0; 4],
        };
    }
}

use shared::utils;

use crate::Base;
use std::fmt::Display;

pub struct Day13 {
    timestamp: usize,
    busses: Vec<Bus>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 {
            timestamp: 0,
            busses: Vec::new(),
        };
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        let mut lines = raw_input.lines();
        self.timestamp = lines.next().unwrap().parse().unwrap();
        for (offset, id) in lines.next().unwrap().split(',').enumerate() {
            if id == "x" {
                continue;
            }
            self.busses.push(Bus {
                offset,
                id: id.parse().unwrap(),
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut best_offset = 0;
        let mut best_wait = usize::MAX;

        for bus in &self.busses {
            let wait = (((self.timestamp / bus.id) + 1) * bus.id) - self.timestamp;
            if wait < best_wait {
                best_offset = bus.offset;
                best_wait = wait;
            }
        }

        return Box::new(best_offset * best_wait);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut ans = self.busses[0].id;
        let mut lcm = self.busses[0].id;

        for bus in &self.busses {
            while (ans + bus.offset) % bus.id != 0 {
                ans += lcm;
            }
            lcm = utils::lcm(lcm, bus.id);
        }
        return Box::new(ans);
    }
}

struct Bus {
    offset: usize,
    id: usize,
}

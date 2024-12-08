use crate::Base;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day05 {
    rules: HashMap<u16, HashSet<u16>>,
    updates: Vec<Vec<u16>>,
    bad_updates: Vec<usize>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            rules: HashMap::new(),
            updates: Vec::new(),
            bad_updates: Vec::new(),
        };
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        let mut parts = raw_input.split("\n\n");
        for rule in parts.next().unwrap().lines() {
            let a = rule[0..2].parse().unwrap();
            let b = rule[3..].parse().unwrap();
            if !self.rules.contains_key(&a) {
                self.rules.insert(a, HashSet::new());
            }
            self.rules.get_mut(&a).unwrap().insert(b);
        }

        for upd in parts.next().unwrap().lines() {
            self.updates.push(upd.split(',').map(|x| x.parse().unwrap()).collect());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        for (i, upd) in self.updates.iter().enumerate() {
            let mut ok = true;
            for j in 0..upd.len() {
                let afters = match self.rules.get(&upd[j]) {
                    Some(a) => a,
                    None => continue,
                };
                if afters.iter().any(|a| upd[..j].contains(a)) {
                    ok = false;
                    break;
                }
            }
            if ok {
                t += upd[upd.len() / 2];
            } else {
                self.bad_updates.push(i);
            }
        }
        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        for i in &self.bad_updates {
            let upd = &mut self.updates[*i];
            upd.sort_by(|a, b| {
                if self.rules.get(a).is_some_and(|v| v.contains(b)) {
                    return Ordering::Less;
                }
                return Ordering::Equal;
            });
            t += upd[upd.len() / 2];
        }

        return Box::new(t);
    }
}

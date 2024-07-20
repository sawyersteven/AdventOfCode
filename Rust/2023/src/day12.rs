use crate::Base;
use std::{collections::HashMap, fmt::Display, hash::Hash};

pub struct Day12 {
    rows: Vec<SpringRow>,
}

const EMPTY: char = '.';
const DAMAGED: char = '#';
const UNKNOWN: char = '?';

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 { rows: Vec::new() };
    }

    fn solve(
        &self,
        record: &[char],
        damages: &[usize],
        cache: &mut HashMap<(Vec<char>, Vec<usize>), usize>,
    ) -> usize {
        if damages.len() == 0 {
            if record.contains(&DAMAGED) {
                return 0;
            } else {
                return 1;
            };
        }

        if record.len() == 0 {
            return 0;
        }

        let k = (record.to_vec(), damages.to_vec());
        match cache.get(&k) {
            Some(v) => return *v,
            None => {}
        }

        let mut total = 0;
        let next_grp = damages[0];

        match record[0] {
            DAMAGED => {
                if record.len() >= next_grp
                    && record[..next_grp]
                        .iter()
                        .filter(|x| **x == DAMAGED || **x == UNKNOWN)
                        .count()
                        == next_grp
                {
                    // more damaged springs exist
                    if damages.len() > 1 {
                        if !((record.len() < next_grp + 1) || record[next_grp] == DAMAGED) {
                            // next spring exists and is fixable
                            total += self.solve(&record[(next_grp + 1)..], &damages[1..], cache);
                        } else {
                            cache.insert(k, 0);
                            return 0;
                        }
                    } else {
                        total += self.solve(&record[next_grp..], &damages[1..], cache);
                    }
                }
            }
            EMPTY => {
                total += self.solve(&record[1..], damages, cache);
            }
            UNKNOWN => {
                let mut r = record.to_vec();
                r[0] = DAMAGED;
                total += self.solve(&r, damages, cache);
                r[0] = EMPTY;
                total += self.solve(&r, damages, cache);
            }
            _ => unreachable!(),
        }

        cache.insert(k, total);

        return total;
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            self.rows.push(SpringRow::new(line));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut cache = HashMap::new();
        let mut rows = self.rows.clone();
        let mut total = 0;
        for r in rows.iter_mut() {
            total += self.solve(&r.record, &r.groups, &mut cache);
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut cache = HashMap::new();
        let mut rows = self.rows.clone();
        let mut total = 0;
        for r in rows.iter_mut() {
            r.convert_to_pt2();
            total += self.solve(&r.record, &r.groups, &mut cache);
        }
        return Box::new(total); // 4968620679637
    }
}

#[derive(Clone, Hash, PartialEq, Eq)]
struct SpringRow {
    groups: Vec<usize>,
    record: Vec<char>,
}

impl SpringRow {
    fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let groups: Vec<usize> = parts[1].split(',').map(|x| x.parse().unwrap()).collect();

        return SpringRow {
            groups: groups,
            record: parts[0].chars().collect(),
        };
    }

    fn convert_to_pt2(&mut self) {
        let mut springs = Vec::with_capacity(self.record.len() + 5);
        for _ in 0..4 {
            springs.append(&mut self.record.clone());
            springs.push(UNKNOWN);
        }
        springs.append(&mut self.record.clone());

        let mut groups = Vec::with_capacity(self.groups.len() * 5);
        for _ in 0..5 {
            groups.append(&mut self.groups.clone());
        }

        self.record = springs;
        self.groups = groups;
    }
}

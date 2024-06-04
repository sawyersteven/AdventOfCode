/*
There is something here I can't be bothered to figure out. Either in the
sorting or pruning of targets. Working around the borrow checker just to
get this to run at all was a pain and I've run out of steam for this day
*/

use itertools::Itertools;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

const IMMUNE: bool = true;
const INFECTION: bool = false;

pub struct Day24 {
    pub input: String,
    og_groups: Vec<Group>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            input: String::from(""),
            og_groups: Vec::new(),
        };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        let mut system = IMMUNE;
        let lines: Vec<&str> = raw_input.split('\n').skip(1).collect();
        let mut i = 0;
        while i < lines.len() {
            if lines[i].is_empty() {
                i += 2;
                system = INFECTION;
                continue;
            }
            self.og_groups.push(Group::new(system, lines[i], i));
            i += 1;
        }

        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let victors = self.sim_battle(0).unwrap();
        let mut u = 0;
        for v in victors.iter().filter(|x| x.units > 0) {
            u += v.units;
        }
        return Box::new(u);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

impl Day24 {
    // Returns tuple of (attaker.uid, target.uid)
    fn select_targets(&self, groups: &Vec<Group>) -> Vec<(usize, usize)> {
        let mut atk_map = Vec::new();

        for attacker in groups.iter().sorted_by(|a, b| {
            if a.power() == b.power() {
                b.power().cmp(&a.power())
            } else {
                b.power().cmp(&a.power())
            }
        }) {
            let mut best_dmg = 0;
            let mut best_target = attacker;
            for target in groups {
                if attacker.system == target.system || atk_map.iter().any(|(_, t)| *t == target.uid) {
                    continue;
                }

                let damage = attacker.calc_damage(&target);
                if damage == 0 {
                    continue;
                }
                if damage > best_dmg {
                    best_dmg = damage;
                    best_target = target;
                } else if damage == best_dmg {
                    if best_target.power() == target.power() {
                        if best_target.initiative <= target.initiative {
                            best_target = target;
                        };
                    } else if best_target.power() < target.power() {
                        best_target = target;
                    };
                }
            }
            atk_map.push((attacker.uid, best_target.uid));
        }
        return atk_map;
    }

    fn battle_over(&self, groups: &Vec<Group>) -> bool {
        let first = groups[0].system;
        return groups.iter().all(|x| x.system == first);
    }

    fn sim_battle(&self, boost: isize) -> Option<Vec<Group>> {
        let mut groups: Vec<Group> = self.og_groups.iter().map(|x| x.clone()).collect();

        for g in &mut groups {
            if g.system == IMMUNE {
                g.attack_dmg += boost;
            }
        }

        for _ in 0..usize::MAX {
            let mut stalemate = true;
            let atk_map = self.select_targets(&groups);

            // attcker's uids in order
            let atk_order: Vec<usize> = groups
                .iter()
                .sorted_by(|a, b| b.initiative.cmp(&a.initiative))
                .map(|x| x.uid)
                .collect();

            //for atk_ind in 0..groups.len() {
            for atk_uid in atk_order {
                let atk_ind = groups.iter().position(|x| x.uid == atk_uid).unwrap();

                if groups[atk_ind].units <= 0 {
                    continue;
                }

                let t_uid = atk_map.iter().find_or_first(|(a, _)| *a == atk_uid).unwrap().1;

                let tgt_ind = groups.iter().position(|x| x.uid == t_uid).unwrap();
                if tgt_ind == atk_ind {
                    continue;
                }

                let lost_units = groups[atk_ind].calc_damage(&groups[tgt_ind]) / groups[tgt_ind].hp;

                if lost_units > 0 {
                    stalemate = false;
                }
                groups[tgt_ind].units -= lost_units;
            }
            groups.retain(|x| x.units > 0);

            if stalemate {
                return None;
            }
            if self.battle_over(&groups) {
                break;
            }
        }
        return Some(groups);
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct Group {
    system: bool,
    units: isize,
    hp: isize,
    weaknesses: HashSet<String>,
    immunities: HashSet<String>,
    attack_dmg: isize,
    attack_typ: String,
    initiative: isize,
    uid: usize,
}

impl Group {
    pub fn new(system: bool, info: &str, uid: usize) -> Self {
        let words: Vec<&str> = info.split(' ').collect();
        let mut g = Group {
            system: system,
            units: words[0].parse().unwrap(),
            hp: words[4].parse().unwrap(),
            weaknesses: HashSet::new(),
            immunities: HashSet::new(),
            attack_dmg: words[words.len() - 6].parse().unwrap(),
            attack_typ: words[words.len() - 5].to_string(),
            initiative: words[words.len() - 1].parse().unwrap(),
            uid: uid,
        };

        if info.contains('(') {
            for substr in info.split('(').nth(1).unwrap().split(')').nth(0).unwrap().split("; ") {
                let hs = if substr.as_bytes()[0] == b'w' {
                    &mut g.weaknesses
                } else {
                    &mut g.immunities
                };
                for s in substr.split("to ").nth(1).unwrap().split(", ") {
                    hs.insert(s.to_string());
                }
            }
        }

        return g;
    }

    pub fn power(&self) -> isize {
        return self.units * self.attack_dmg;
    }

    pub fn calc_damage(&self, target: &Group) -> isize {
        if target.immunities.contains(&self.attack_typ) {
            return 0;
        } else if target.weaknesses.contains(&self.attack_typ) {
            return self.power() * 2;
        }
        return self.power();
    }
}

use crate::Base;
use std::{fmt::Display, os::raw};

struct Item {
    pub cost: isize,
    pub damage: isize,
    pub armor: isize,
}

impl Item {
    pub fn new(cost: isize, damage: isize, armor: isize) -> Self {
        return Item { cost, damage, armor };
    }
}

pub struct Day21 {
    weapons: Vec<Item>,
    armor: Vec<Item>,
    rings: Vec<Item>,
    boss_hp: isize,
    boss_armor: isize,
    boss_dmg: isize,
}

impl Day21 {
    pub fn new() -> Day21 {
        let mut day = Day21 {
            weapons: Vec::new(),
            armor: Vec::new(),
            rings: Vec::new(),
            boss_hp: 0,
            boss_armor: 0,
            boss_dmg: 0,
        };

        day.weapons.push(Item::new(8, 4, 0));
        day.weapons.push(Item::new(10, 5, 0));
        day.weapons.push(Item::new(25, 6, 0));
        day.weapons.push(Item::new(40, 7, 0));
        day.weapons.push(Item::new(74, 8, 0));

        day.armor.push(Item::new(0, 0, 0));
        day.armor.push(Item::new(13, 0, 1));
        day.armor.push(Item::new(31, 0, 2));
        day.armor.push(Item::new(53, 0, 3));
        day.armor.push(Item::new(75, 0, 4));
        day.armor.push(Item::new(102, 0, 5));

        day.rings.push(Item::new(0, 0, 0));
        day.rings.push(Item::new(0, 0, 0));
        day.rings.push(Item::new(25, 1, 0));
        day.rings.push(Item::new(50, 2, 0));
        day.rings.push(Item::new(100, 3, 0));
        day.rings.push(Item::new(20, 0, 1));
        day.rings.push(Item::new(40, 0, 2));
        day.rings.push(Item::new(80, 0, 3));

        return day;
    }

    fn sim_battle(&self, mut damage: isize, mut armor: isize) -> bool {
        let mut boss_hp = self.boss_hp.clone() as isize;
        let mut player_hp: isize = 100;

        while damage < self.boss_armor {
            damage += 1;
        }

        while armor > self.boss_dmg {
            armor -= 1;
        }

        loop {
            boss_hp -= damage - self.boss_armor;
            if boss_hp <= 0 {
                return true;
            }
            player_hp -= self.boss_dmg - armor;
            if player_hp <= 0 {
                return false;
            }
        }
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        self.boss_hp = lines[0].split(' ').last().unwrap().parse().unwrap();
        self.boss_dmg = lines[1].split(' ').last().unwrap().parse().unwrap();
        self.boss_armor = lines[2].split(' ').last().unwrap().parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut lowest_cost = isize::MAX;
        let mut highest_cost = isize::MIN;

        for wpn in &self.weapons {
            for armr in &self.armor {
                for i in 0..self.rings.len() - 1 {
                    let rng1 = &self.rings[i];
                    for j in (i + 1)..self.rings.len() {
                        let rng2 = &self.rings[j];

                        let damage = wpn.damage + rng1.damage + rng2.damage;
                        let armor = armr.armor + rng1.armor + rng2.armor;
                        let cost = wpn.cost + armr.cost + rng1.cost + rng2.cost;

                        if Self::sim_battle(&self, damage, armor) {
                            if cost < lowest_cost {
                                lowest_cost = cost;
                            }
                        } else {
                            if cost > highest_cost {
                                highest_cost = cost;
                            }
                        }
                    }
                }
            }
        }

        return Box::new(format!("{}\t{}", lowest_cost, highest_cost));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-------^");
    }
}

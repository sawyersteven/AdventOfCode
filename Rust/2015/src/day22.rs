use crate::Base;
use std::{collections::VecDeque, fmt::Display};

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum SpellName {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

struct Spell {
    id: SpellName,
    cost: isize,
    damage: isize,
    heal: isize,
    armor: isize,
    recharge: isize,
    duration: isize,
}

#[derive(Clone)]
struct State {
    player_hp: isize,
    player_mana: isize,
    total_consumed_mana: isize,
    boss_hp: isize,
    shield_timer: isize,
    poison_timer: isize,
    rechrg_timer: isize,
}

impl State {
    fn get_timer(&self, spell: SpellName) -> isize {
        match spell {
            SpellName::Shield => self.shield_timer,
            SpellName::Poison => self.poison_timer,
            SpellName::Recharge => self.rechrg_timer,
            _ => panic!(),
        }
    }

    fn set_timer(&mut self, spell: SpellName, t: isize) {
        match spell {
            SpellName::Shield => self.shield_timer = t,
            SpellName::Poison => self.poison_timer = t,
            SpellName::Recharge => self.rechrg_timer = t,
            _ => panic!(),
        }
    }

    fn apply_effects(&mut self) {
        if self.shield_timer > 0 {
            self.shield_timer -= 1;
        }
        if self.poison_timer > 0 {
            self.boss_hp -= 3;
            self.poison_timer -= 1;
        }
        if self.rechrg_timer > 0 {
            self.player_mana += 101;
            self.rechrg_timer -= 1;
        }
    }
}

pub struct Day22 {
    pub input: String,
    boss_start_hp: isize,
    boss_dmg: isize,
    spell_book: Vec<Spell>,
}

impl Day22 {
    pub fn new() -> Day22 {
        let mut d = Day22 {
            input: String::from(""),
            boss_start_hp: 55,
            boss_dmg: 8,
            spell_book: Vec::new(),
        };

        d.spell_book.push(Spell {
            id: SpellName::Missile,
            cost: 53,
            damage: 4,
            heal: 0,
            armor: 0,
            recharge: 0,
            duration: 0,
        });

        d.spell_book.push(Spell {
            id: SpellName::Drain,
            cost: 73,
            damage: 2,
            heal: 2,
            armor: 0,
            recharge: 0,
            duration: 0,
        });

        d.spell_book.push(Spell {
            id: SpellName::Shield,
            cost: 113,
            damage: 0,
            heal: 0,
            armor: 7,
            recharge: 0,
            duration: 6,
        });

        d.spell_book.push(Spell {
            id: SpellName::Poison,
            cost: 173,
            damage: 3,
            heal: 0,
            armor: 0,
            recharge: 0,
            duration: 6,
        });

        d.spell_book.push(Spell {
            id: SpellName::Recharge,
            cost: 229,
            damage: 0,
            heal: 0,
            armor: 0,
            recharge: 101,
            duration: 5,
        });

        return d;
    }

    fn sim(&self, hard: bool) -> isize {
        let mut least_mana = isize::MAX;

        let mut q: VecDeque<State> = VecDeque::new();
        q.push_front(State {
            player_hp: 50,
            player_mana: 500,
            total_consumed_mana: 0,
            boss_hp: self.boss_start_hp,
            shield_timer: 0,
            poison_timer: 0,
            rechrg_timer: 0,
        });

        while q.len() != 0 {
            let mut state = q.pop_front().unwrap();
            if hard {
                state.player_hp -= 1;
                if state.player_hp < 1 {
                    continue;
                }
            }

            for next_spell in &self.spell_book {
                if state.player_mana < next_spell.cost {
                    continue;
                }

                let mut next_state = state.clone();

                if next_spell.duration == 0 {
                    next_state.boss_hp -= next_spell.damage;
                    next_state.player_hp += next_spell.heal;
                } else {
                    if next_state.get_timer(next_spell.id) > 0 {
                        continue;
                    }
                    next_state.set_timer(next_spell.id, next_spell.duration);
                }

                next_state.player_mana -= next_spell.cost;
                next_state.total_consumed_mana += next_spell.cost;
                next_state.apply_effects();

                if next_state.boss_hp > 0 {
                    let mut dmg = self.boss_dmg - if next_state.shield_timer > 0 { 7 } else { 0 };
                    if dmg < 1 {
                        dmg = 1;
                    }
                    next_state.player_hp -= dmg;
                    next_state.apply_effects();
                }

                if next_state.total_consumed_mana >= least_mana || next_state.player_hp < 1 {
                    continue;
                }

                if next_state.boss_hp < 1 {
                    least_mana = next_state.total_consumed_mana;
                    continue;
                }

                q.push_back(next_state);
            }
        }
        return least_mana;
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(Self::sim(&self, false));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(Self::sim(&self, true));
    }
}

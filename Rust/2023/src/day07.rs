use itertools::Itertools;

use crate::Base;
use std::{cmp::Ordering, fmt::Display};

#[derive(Clone)]
enum HandType {
    High,
    PairOne,
    PairTwo,
    Three,
    House,
    Four,
    Five,
}

const CARD_ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

pub struct Day07 {
    pub hands: Vec<(u8, Vec<u8>, usize)>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { hands: Vec::new() };
    }
}

/*
Almost all of the work is done in ParseInput
Cards are translated to a u8 value 0-12, hand types are a u8 from 0-6 in order
of sorting value.

Then its just sort by hand type first, cards second, and count the scores.

*/
impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let (cards, value) = line.split_once(' ').unwrap();
            let card_chars = cards.chars().collect();
            self.hands.push((
                hand_type(&card_chars) as u8,
                card_chars
                    .iter()
                    .map(|x| CARD_ORDER.iter().position(|y| y == x).unwrap() as u8)
                    .collect(),
                value.trim().parse().unwrap(),
            ));
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut hands = self.hands.to_vec();

        hands.sort_by(|a, b| {
            if a.0 > b.0 {
                return Ordering::Greater;
            } else if a.0 < b.0 {
                return Ordering::Less;
            } else {
                return a.1.cmp(&b.1);
            }
        });
        let mut total = 0;
        for (i, (_, _, value)) in hands.iter().enumerate() {
            total += value * (i + 1);
        }
        return Box::new(total);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

fn hand_type(cards: &Vec<char>) -> HandType {
    let counts = cards.iter().counts();
    let l = counts.len();
    if l == 1 {
        return HandType::Five;
    } else if l == 2 {
        if counts.values().any(|x| *x == 1) {
            return HandType::Four;
        } else {
            return HandType::House;
        }
    } else if l == 3 {
        if counts.values().any(|x| *x == 3) {
            return HandType::Three;
        } else {
            return HandType::PairTwo;
        }
    } else if l == 4 {
        return HandType::PairOne;
    }
    return HandType::High;
}

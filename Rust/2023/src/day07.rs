use itertools::Itertools;

use crate::Base;
use std::{cmp::Ordering, fmt::Display};

#[derive(Clone, PartialEq, PartialOrd)]
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
const CARD_ORDER_2: [char; 13] = ['J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'Q', 'K', 'A'];

pub struct Day07 {
    // Hand type, cards, bid
    pub hands: Vec<(u8, Vec<char>, usize)>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { hands: Vec::new() };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let (cards, value) = line.split_once(' ').unwrap();
            let card_chars: Vec<char> = cards.chars().collect();
            self.hands
                .push((0, card_chars.iter().cloned().collect(), value.trim().parse().unwrap()));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut hands = self.hands.to_vec();
        for i in 0..hands.len() {
            hands[i].0 = hand_type(&hands[i].1) as u8;
        }

        hands.sort_by(|a, b| {
            if a.0 > b.0 {
                return Ordering::Greater;
            } else if a.0 < b.0 {
                return Ordering::Less;
            } else {
                return rank_hands(&a.1, &b.1);
            }
        });

        let mut total = 0;
        for (i, (_, _, value)) in hands.iter().enumerate() {
            total += value * (i + 1);
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut hands = self.hands.to_vec();
        for i in 0..hands.len() {
            hands[i].0 = hand_type_2(&hands[i].1) as u8;
        }

        // sort by hand type
        hands.sort_by(|a, b| {
            if a.0 > b.0 {
                return Ordering::Greater;
            } else if a.0 < b.0 {
                return Ordering::Less;
            } else {
                return rank_hands_2(&a.1, &b.1);
            }
        });

        let mut total = 0;
        for (i, (_, _, value)) in hands.iter().enumerate() {
            total += value * (i + 1);
        }
        return Box::new(total);
    }
}

fn rank_hands(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for i in 0..a.len() {
        // oddly enough, faster than a hashmap lookup table
        let vala = CARD_ORDER.iter().position(|x| *x == a[i]).unwrap();
        let valb = CARD_ORDER.iter().position(|x| *x == b[i]).unwrap();
        if vala == valb {
            continue;
        } else {
            return vala.cmp(&valb);
        }
    }
    return Ordering::Equal;
}

fn rank_hands_2(a: &Vec<char>, b: &Vec<char>) -> Ordering {
    for i in 0..a.len() {
        let vala = CARD_ORDER_2.iter().position(|x| *x == a[i]).unwrap();
        let valb = CARD_ORDER_2.iter().position(|x| *x == b[i]).unwrap();
        if vala == valb {
            continue;
        } else {
            return vala.cmp(&valb);
        }
    }
    return Ordering::Equal;
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

fn hand_type_2(cards: &Vec<char>) -> HandType {
    if !cards.contains(&'J') || cards.iter().all(|x| *x == 'J') {
        return hand_type(cards);
    }

    let mut best = HandType::High;
    let other_cards = cards.iter().filter(|x| **x != 'J');
    for c in other_cards.sorted() {
        let mut h = cards.clone();
        for i in 0..h.len() {
            if h[i] == 'J' {
                h[i] = *c;
            }
        }
        let wcht = hand_type(&h);
        if wcht > best {
            best = wcht;
        }
    }
    return best;
}

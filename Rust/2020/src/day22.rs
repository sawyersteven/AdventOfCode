use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    hash::{DefaultHasher, Hasher},
};

const P1WIN: bool = true;
const P2WIN: bool = false;

pub struct Day22 {
    queues: (VecDeque<usize>, VecDeque<usize>),
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 {
            queues: (VecDeque::new(), VecDeque::new()),
        };
    }

    fn tally(&self, q: &VecDeque<usize>) -> usize {
        let mut ans = 0;
        let mut m = q.len();
        for card in q {
            ans += *card * m;
            m -= 1;
        }
        return ans;
    }

    fn partial_copy_queue(&self, q: &VecDeque<usize>, count: usize) -> VecDeque<usize> {
        let mut nq = VecDeque::new();

        for (i, u) in q.iter().enumerate() {
            if i == count {
                break;
            }
            nq.push_back(*u);
        }
        return nq;
    }

    // true if p1 wins, false for p2
    fn play_2(&self, p1: &mut VecDeque<usize>, p2: &mut VecDeque<usize>) -> bool {
        let mut turn_history = HashSet::new();
        loop {
            let mut hasher = DefaultHasher::new();
            for u in &*p1 {
                hasher.write_usize(*u);
            }
            for u in &*p2 {
                hasher.write_usize(*u);
            }
            let uid = hasher.finish();

            if turn_history.contains(&uid) {
                return P1WIN;
            }
            turn_history.insert(uid);

            let p1card = p1.pop_front().unwrap();
            let p2card = p2.pop_front().unwrap();

            let round_winner: bool;

            if p1card <= p1.len() && p2card <= p2.len() {
                round_winner = self.play_2(
                    &mut self.partial_copy_queue(p1, p1card),
                    &mut self.partial_copy_queue(p2, p2card),
                );
            } else {
                round_winner = p1card > p2card;
            }

            if round_winner {
                p1.push_back(p1card);
                p1.push_back(p2card);
                if p2.len() == 0 {
                    return P1WIN;
                }
            } else {
                p2.push_back(p2card);
                p2.push_back(p1card);
                if p1.len() == 0 {
                    return P2WIN;
                }
            }
        }
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines().skip(1) {
            if line.is_empty() {
                break;
            }
            self.queues.0.push_back(line.parse().unwrap());
        }

        for line in raw_input.split("Player 2:\n").nth(1).unwrap().lines() {
            self.queues.1.push_back(line.parse().unwrap());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let (p1, p2) = &mut self.queues.clone();

        while p1.len() > 0 && p2.len() > 0 {
            let p1card = p1.pop_front().unwrap();
            let p2card = p2.pop_front().unwrap();

            if p1card > p2card {
                p1.push_back(p1card);
                p1.push_back(p2card);
            } else {
                p2.push_back(p2card);
                p2.push_back(p1card);
            }
        }

        return Box::new(self.tally(if p1.len() == 0 { &p2 } else { &p1 }));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let (p1, p2) = &mut self.queues.clone();
        self.play_2(p1, p2);
        return Box::new(self.tally(if p1.len() == 0 { &p2 } else { &p1 }));
    }
}

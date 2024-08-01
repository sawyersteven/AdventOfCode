use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day23 {
    input: Vec<u32>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: Vec::new() };
    }

    fn rotate_vdq_to(&self, q: &mut VecDeque<u32>, new_head: u32) {
        let mut head = q.front().unwrap();
        while *head != new_head {
            let v = q.pop_front().unwrap();
            q.push_back(v);
            head = q.front().unwrap();
        }
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.bytes().map(|x| (x - b'0') as u32).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const TURNS: usize = 1;
        let mut cups = VecDeque::from_iter(self.input.iter().cloned());

        for _ in 0..TURNS {
            let current_val = *cups.front().unwrap();
            cups.rotate_left(1);

            let mut picked_up = [0; 3];
            for i in 0..3 {
                picked_up[i] = cups.pop_front().unwrap();
            }

            let mut dest_val = current_val - 1;

            if dest_val == 0 {
                dest_val = 9;
            }

            while picked_up.contains(&dest_val) {
                dest_val -= 1;
                if dest_val == 0 {
                    dest_val = 9;
                }
            }

            self.rotate_vdq_to(&mut cups, dest_val);
            cups.rotate_left(1);

            for c in picked_up {
                cups.push_back(c);
            }

            self.rotate_vdq_to(&mut cups, current_val);
            cups.rotate_left(1);
        }

        self.rotate_vdq_to(&mut cups, 1);
        cups.pop_front();

        return Box::new(cups.iter().map(|x| (*x as u8 + b'0') as char).collect::<String>());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const MAX_VAL: u32 = 1000000;
        const TURNS: u32 = 10000000;

        // Map of cup#: next_cup#
        let mut cups = HashMap::new();
        for i in 0..(self.input.len() - 1) {
            cups.insert(self.input[i], self.input[i + 1]);
        }
        cups.insert(self.input[self.input.len() - 1], self.input[0]);

        cups.insert(self.input[self.input.len() - 1], 10);

        for i in 10..MAX_VAL {
            cups.insert(i, i + 1);
        }
        cups.insert(MAX_VAL, self.input[0]);

        let mut current = self.input[0];

        for _ in 0..TURNS {
            let abc = cups.pop_next_3(current);

            let mut dest = current - 1;
            if dest == 0 {
                dest = MAX_VAL;
            }
            while abc.contains(&dest) {
                dest -= 1;
                if dest == 0 {
                    dest = MAX_VAL;
                }
            }
            cups.insert_after(dest, abc);

            current = cups[&current];
        }
        let c1 = cups.pop_next(1);
        let c2 = cups.pop_next(1);

        return Box::new(c1 * c2); // 5403610688
    }
}

impl Circle for HashMap<u32, u32> {
    fn pop_next(&mut self, cup: u32) -> u32 {
        let n = self[&cup];
        self.insert(cup, self[&n]);
        return n;
    }

    fn pop_next_3(&mut self, cup: u32) -> [u32; 3] {
        let mut ns = [0; 3];
        let mut cur = cup;
        for i in 0..3 {
            let n = self.remove(&cur).unwrap();
            ns[i] = n;
            cur = n;
        }
        let cup_next = self.remove(&cur).unwrap();
        self.insert(cup, cup_next);

        return ns;
    }

    fn insert_after(&mut self, cup: u32, next: [u32; 3]) {
        self.insert(next[2], self[&cup]);
        self.insert(cup, next[0]);
        self.insert(next[0], next[1]);
        self.insert(next[1], next[2]);
    }
}

trait Circle {
    fn pop_next(&mut self, cup: u32) -> u32;
    fn pop_next_3(&mut self, cup: u32) -> [u32; 3];
    fn insert_after(&mut self, cup: u32, next: [u32; 3]);
}

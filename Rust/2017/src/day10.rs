use shared::circular_deque::CircularDeque;

use crate::Base;
use std::fmt::Display;

pub struct Day10 {
    pub input: String,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 {
            input: String::from(""),
        };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut circle = CircularDeque::<usize>::new();
        for i in 0..=u8::MAX {
            circle.add_last(i as usize);
        }

        let parts: Vec<&str> = self.input.split(',').collect();
        let mut lengths = Vec::<usize>::new();

        for i in 0..parts.len() {
            lengths.push(parts[i].parse().unwrap());
        }

        let mut total_skips = 0;
        let mut skip_size = 0;
        for len in lengths {
            circle.reverse_segment(len as isize);
            circle.move_head_right(skip_size + len);
            total_skips += skip_size + len;
            skip_size += 1;
        }

        circle.move_head_left(total_skips);

        return Box::new(circle.get(0) * circle.get(1));
    }

    fn part2(&self) -> Box<dyn Display> {
        let bytestring = self.input.as_bytes();
        let mut lengths = bytestring.clone().to_vec();
        for suffix in [17, 31, 73, 47, 23] {
            lengths.push(suffix);
        }

        let hash = KnotHash::new(&self.input).hash_code();
        return Box::new(hash);
    }
}

pub struct KnotHash {
    dense: Vec<u8>,
    hash: String,
}

#[allow(unused)]
impl KnotHash {
    pub fn new(input: &String) -> Self {
        let bytestring = input.as_bytes();
        let mut lengths = bytestring.clone().to_vec();
        for suffix in [17, 31, 73, 47, 23] {
            lengths.push(suffix);
        }

        let mut circle = CircularDeque::<u8>::new();
        for i in 0..=u8::MAX {
            circle.add_last(i);
        }

        Self::tie_knot(&mut circle, lengths, 64);

        let dense = Self::make_dense_hash(circle.to_vec());
        let hash = Self::make_hash_code(&dense);

        return KnotHash {
            dense: dense,
            hash: hash,
        };
    }

    pub fn hash_code(&self) -> String {
        return self.hash.clone();
    }

    pub fn dense_hash(&self) -> Vec<u8> {
        return self.dense.clone();
    }

    fn reverse_segment<T>(deque: &mut CircularDeque<T>, len: isize) {
        let mut start = 0;
        let mut end = len - 1;

        while start <= end {
            deque.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    fn make_hash_code(dense: &Vec<u8>) -> String {
        return String::from_iter(dense.iter().map(|x| format!("{:02x}", x)));
    }

    fn make_dense_hash(sparse: Vec<u8>) -> Vec<u8> {
        let mut dense = Vec::<u8>::new();
        for i in 0..16 {
            let mut xord = 0;

            let mut j = i * 16;
            while j < (i + 1) * 16 {
                xord ^= sparse[j];
                j += 1;
            }

            dense.push(xord as u8);
        }
        return dense;
    }

    fn tie_knot<T>(circle: &mut CircularDeque<T>, lengths: Vec<u8>, iters: usize) {
        let mut total_skips = 0;
        let mut skip_size = 0;

        for _ in 0..iters {
            for len in &lengths {
                circle.reverse_segment(*len as isize);
                circle.move_head_right(skip_size + *len as usize);
                total_skips += skip_size + *len as usize;
                skip_size += 1;
            }
        }

        circle.move_head_left(total_skips);
    }
}

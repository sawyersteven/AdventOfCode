use crate::Base;
use std::fmt::Display;

const DISKLEN1: usize = 272;
const DISKLEN2: usize = 35651584;

pub struct Day16 {
    pub buffer: Vec<bool>,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 { buffer: Vec::new() };
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        for b in raw_input.as_bytes() {
            self.buffer.push(b != &b'0');
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut src = self.buffer.clone();
        src = fill_disk(&src, DISKLEN1);
        src = reduce(&src);

        return Box::new(bin_string(&src));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut src = self.buffer.clone();
        src = fill_disk(&src, DISKLEN2);
        src = reduce(&src);

        return Box::new(bin_string(&src));
    }
}

fn fill_disk(src: &Vec<bool>, size: usize) -> Vec<bool> {
    let mut src = src.clone();
    while src.len() < size {
        let end: Vec<bool> = src.iter().rev().map(|x| !x).collect();

        src.push(false);
        for i in 0..end.len() {
            src.push(end[i]);
        }
    }
    src.truncate(size);
    return src;
}

fn reduce(src: &Vec<bool>) -> Vec<bool> {
    let mut src = src.clone();
    while src.len() % 2 == 0 {
        let mut r = Vec::<bool>::with_capacity(src.len() / 2);
        for ri in 0..r.capacity() {
            r.push(src[ri * 2] == src[ri * 2 + 1]);
        }
        if src.len() == 0 {
            panic!();
        }
        src = r;
    }
    return src;
}

fn bin_string(src: &Vec<bool>) -> String {
    return String::from_iter(src.iter().map(|x| if *x { '1' } else { '0' }));
}

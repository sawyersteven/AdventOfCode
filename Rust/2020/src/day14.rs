use crate::Base;
use std::{collections::HashMap, fmt::Display};

pub struct Day14 {
    blocks: Vec<Block>,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { blocks: Vec::new() };
    }

    fn mem_addrs(&self, mask: &[u8; 36], addr: u64) -> impl Iterator<Item = u64> {
        let mut addr = addr;
        let mut x_inds = Vec::new();
        for (i, b) in mask.iter().enumerate() {
            if *b == b'X' {
                x_inds.push(i);
            } else if *b == b'1' {
                addr |= 1 << (36 - 1 - i); // flip endian-ness
            }
        }

        let mut p = 0;
        let end = 2usize.pow(x_inds.len() as u32);
        std::iter::from_fn(move || loop {
            if p == end {
                return None;
            }
            for i in 0..x_inds.len() {
                if ((p >> i) & 1) == 1 {
                    addr |= 1 << (36 - 1 - x_inds[i]);
                } else {
                    addr &= !(1 << (36 - 1 - x_inds[i]));
                };
            }
            p += 1;
            return Some(addr);
        })
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        for blk in raw_input.split("mask = ").skip(1) {
            let mut lines = blk.lines();
            let mask_str = lines.next().unwrap();
            let mask_0 = bin_to_u64(&mask_str.replace('X', "1"));
            let mask_1 = bin_to_u64(&mask_str.replace('X', "0"));

            let mut mask_x = [0; 36];
            mask_x.clone_from_slice(mask_str.as_bytes());
            let mut block = Block {
                mask_x,
                mask_0,
                mask_1,
                mem_writes: Vec::new(),
            };

            for line in lines {
                let addr = line.split(']').nth(0).unwrap()[4..].parse().unwrap();
                let val = line.split(" = ").nth(1).unwrap().parse().unwrap();
                block.mem_writes.push((addr, val));
            }
            self.blocks.push(block);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut mem = HashMap::new();
        for block in &self.blocks {
            for (addr, value) in &block.mem_writes {
                let val = (value | block.mask_1) & block.mask_0;
                mem.insert(addr, val);
            }
        }

        return Box::new(mem.values().sum::<u64>());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut mem = HashMap::new();

        for block in &self.blocks {
            for (addr, val) in &block.mem_writes {
                for addr in self.mem_addrs(&block.mask_x, *addr) {
                    mem.insert(addr, *val);
                }
            }
        }

        return Box::new(mem.values().sum::<u64>());
    }
}

struct Block {
    mask_x: [u8; 36],
    mask_0: u64,
    mask_1: u64,
    mem_writes: Vec<(u64, u64)>,
}

fn bin_to_u64(value: &str) -> u64 {
    let mut num = 0;
    for b in value.as_bytes() {
        num <<= 1;
        if *b == b'1' {
            num += 1;
        }
    }
    return num;
}

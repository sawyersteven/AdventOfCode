use crate::Base;
use std::{cmp::Ordering, fmt::Display};

const OPEN: u8 = b'[';
const CLOSE: u8 = b']';

#[derive(Clone)]
enum Packet {
    Integer(u8),
    List(Vec<Packet>),
}

#[allow(unused)]
impl Packet {
    fn print(&self) {
        self._print_recurse(&self);
        println!();
    }

    fn _print_recurse(&self, current: &Packet) {
        match current {
            Packet::List(l) => {
                print!("{}", OPEN as char);
                for child in l {
                    self._print_recurse(&child);
                }
                print!("{}", CLOSE as char);
            }
            Packet::Integer(i) => {
                print!("{},", *i as char);
            }
        }
    }

    fn compare(&self, other: &Self) -> Ordering {
        match (self, other) {
            // both integer values
            (Packet::Integer(l), Packet::Integer(r)) => {
                return l.cmp(r);
            }
            // list, int
            (Packet::List(_l), Packet::Integer(_r)) => {
                return self.compare(&Packet::List(vec![other.clone()]));
            }
            // int, list
            (Packet::Integer(_l), Packet::List(_r)) => {
                return Packet::List(vec![self.clone()]).compare(other);
            }
            // list, list
            (Packet::List(l), Packet::List(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    let l = l_iter.next();
                    let r = r_iter.next();
                    if l.is_none() && r.is_none() {
                        return Ordering::Equal;
                    } else if l.is_none() && r.is_some() {
                        // left side empties first
                        return Ordering::Less;
                    } else if l.is_some() && r.is_none() {
                        // right side empties first
                        return Ordering::Greater;
                    } else {
                        let r = l.unwrap().compare(r.unwrap());
                        if r != Ordering::Equal {
                            return r;
                        }
                    }
                }
            }
        }
    }
}

pub struct Day13 {
    pairs: Vec<(Packet, Packet)>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 { pairs: Vec::new() };
    }

    fn build_packet_tree(&self, bytes: &[u8], parent: &mut Vec<Packet>) -> usize {
        let mut position = 0;
        while position < bytes.len() {
            match bytes[position] {
                OPEN => {
                    let mut child = Vec::new();
                    position += self.build_packet_tree(&bytes[(position + 1)..], &mut child);
                    parent.push(Packet::List(child));
                }
                CLOSE => {
                    return position + 1;
                }
                n => {
                    parent.push(Packet::Integer(n));
                }
            }
            position += 1;
        }
        return bytes.len();
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        let raw_input = raw_input.replace("10", ":");
        let lines: Vec<&str> = raw_input.split('\n').collect();
        for i in (0..lines.len()).step_by(3) {
            let l: Vec<u8> = lines[i].bytes().filter(|x| *x != b',').collect();
            let r: Vec<u8> = lines[i + 1].bytes().filter(|x| *x != b',').collect();

            let mut left = Vec::new();
            self.build_packet_tree(&l[1..(l.len() - 1)], &mut left);

            let mut right = Vec::new();
            self.build_packet_tree(&r[1..(r.len() - 1)], &mut right);

            self.pairs.push((Packet::List(left), Packet::List(right)));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0;

        for (i, (l, r)) in self.pairs.iter().enumerate() {
            if l.compare(r) == Ordering::Less {
                sum += 1 + i;
            }
        }
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut p2 = Vec::new();
        self.build_packet_tree("[[2]]".as_bytes(), &mut p2);
        let p2 = Packet::List(p2);
        let mut p2_ind = 1;
        for (l, r) in &self.pairs {
            if p2.compare(l) == Ordering::Greater {
                p2_ind += 1;
            }
            if p2.compare(r) == Ordering::Greater {
                p2_ind += 1;
            }
        }

        let mut p6 = Vec::new();
        self.build_packet_tree("[[6]]".as_bytes(), &mut p6);
        let p6 = Packet::List(p6);
        let mut p6_ind = 2;
        for (l, r) in &self.pairs {
            if p6.compare(l) == Ordering::Greater {
                p6_ind += 1;
            }
            if p6.compare(r) == Ordering::Greater {
                p6_ind += 1;
            }
        }
        return Box::new(p2_ind * p6_ind);
    }
}

/*
This WILL NOT WORK with test input where modules have single-character names
If using test data, modify any module names to two characters before running
*/

use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

const LOW: bool = false;
const HIGH: bool = true;
const RX: u16 = 0b01110010_01111000;
const BC: u16 = 0b01100010_01100011;

pub struct Day20 {
    modules: HashMap<u16, Box<dyn Module>>,
    broadcaster: Vec<u16>,
    raw_input: String,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 {
            modules: HashMap::new(),
            broadcaster: Vec::new(),
            raw_input: String::new(),
        };
    }

    fn push_button(&mut self) -> (usize, usize, bool) {
        // (sender, pulse, receiver)
        let mut call_q = VecDeque::new();

        let mut low_count = 1; // from initial button press
        let mut high_count = 0;
        let mut finished = false;

        for r in &self.broadcaster {
            call_q.push_front((BC, LOW, *r));
            low_count += 1;
        }

        while let Some((sender, rx_pulse, target)) = call_q.pop_back() {
            if target == RX {
                if rx_pulse == LOW {
                    finished = true;
                    break;
                }
                continue;
            }
            match self.modules.get_mut(&target).unwrap().rx(sender, rx_pulse) {
                None => {}
                Some(tx_pulse) => {
                    for o in self.modules[&target].outputs() {
                        call_q.push_front((target, tx_pulse, *o));
                        if tx_pulse == HIGH {
                            high_count += 1;
                        } else {
                            low_count += 1;
                        }
                    }
                }
            }
        }
        return (high_count, low_count, finished);
    }

    fn push_button_2(&mut self, tx_names: Vec<u16>) -> Vec<usize> {
        let mut tx_counts = Vec::with_capacity(tx_names.len());

        let mut call_q = VecDeque::new();

        for presses in 1..usize::MAX {
            for r in &self.broadcaster {
                call_q.push_front((BC, LOW, *r));
            }

            while let Some((sender, rx_pulse, target)) = call_q.pop_back() {
                if tx_names.contains(&sender) && rx_pulse == HIGH {
                    tx_counts.push(presses);
                }

                match self.modules.get_mut(&target).unwrap().rx(sender, rx_pulse) {
                    None => {}
                    Some(tx_pulse) => {
                        for o in self.modules[&target].outputs() {
                            call_q.push_front((target, tx_pulse, *o));
                        }
                    }
                }
            }
            if tx_counts.len() == tx_names.len() {
                break;
            }
        }
        return tx_counts;
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        let mut conjs = HashMap::new();
        let mut flips = HashMap::new();
        for line in raw_input.lines() {
            let outs = line
                .split("->")
                .nth(1)
                .unwrap()
                .split(',')
                .map(|x| name_to_u16(x.trim()))
                .collect();

            match line.as_bytes()[0] {
                b'%' => {
                    flips.insert(name_to_u16(&line[1..3]), FlipFlop::new(outs));
                }
                b'&' => {
                    conjs.insert(name_to_u16(&line[1..3]), Conjuction::new(outs));
                }
                b'b' => {
                    self.broadcaster = line
                        .split("->")
                        .nth(1)
                        .unwrap()
                        .split(',')
                        .map(|x| name_to_u16(x.trim()))
                        .collect();
                }
                _ => panic!(),
            }
        }
        // There must be a better way
        for (flipname, flip) in flips {
            let outs = flip.outputs();
            for (conjname, conj) in &mut conjs {
                if outs.contains(&conjname) {
                    conj.prev.insert(flipname, false);
                }
            }
            self.modules.insert(flipname, Box::new(flip) as Box<dyn Module>);
        }

        for (n, c) in conjs {
            self.modules.insert(n, Box::new(c) as Box<dyn Module>);
        }

        self.modules
            .insert(RX, Box::new(FlipFlop::new(Vec::new())) as Box<dyn Module>);
        self.raw_input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut h = 0;
        let mut l = 0;
        for _ in 0..1000 {
            let (hh, ll, _) = self.push_button();
            h += hh;
            l += ll;
        }
        return Box::new(l * h);
    }

    /*
      The parent of RX (the collector) is a conjunction that will only output
    LOW to RX if all of its input have sent HIGH as their last pulse.
      So I can measure how long it takes each of collector's input to send a
    HIGH and find their lcm. This makes some assumptions about how the loops
    for each input works, but AOC puzzles are usually not designed to be
    obtuse enough to worry about that.
     */
    fn part2(&mut self) -> Box<dyn Display> {
        for (_, m) in self.modules.iter_mut() {
            m.reset();
        }

        self.parse_input(self.raw_input.clone());

        let mut collector = 0;

        for m in &self.modules {
            if m.1.outputs().contains(&RX) {
                collector = *m.0;
                break;
            }
        }
        if collector == 0 {
            panic!();
        }

        let mut tx_names = Vec::new();
        for m in &self.modules {
            if m.1.outputs().contains(&collector) {
                tx_names.push(*m.0);
            }
        }

        let counts = self.push_button_2(tx_names);
        let prod = shared::utils::lcm_array(&counts);
        return Box::new(prod);
    }
}

fn name_to_u16(name: &str) -> u16 {
    if name.len() != 2 {
        panic!();
    }
    return ((name.as_bytes()[0] as u16) << 8) + name.as_bytes()[1] as u16;
}

trait Module {
    fn rx(&mut self, sender: u16, pulse: bool) -> Option<bool>;
    fn outputs(&self) -> &Vec<u16>;
    fn reset(&mut self);
}

struct FlipFlop {
    state: bool,
    outputs: Vec<u16>,
}

impl FlipFlop {
    fn new(outputs: Vec<u16>) -> Self {
        return FlipFlop {
            state: false,
            outputs: outputs,
        };
    }
}

impl Module for FlipFlop {
    fn rx(&mut self, _sender: u16, pulse: bool) -> Option<bool> {
        if pulse == HIGH {
            return None;
        }

        self.state = !self.state;
        return Some(self.state);
    }

    fn outputs(&self) -> &Vec<u16> {
        return &self.outputs;
    }

    fn reset(&mut self) {
        self.state = false;
    }
}

struct Conjuction {
    prev: HashMap<u16, bool>,
    outputs: Vec<u16>,
}

impl Conjuction {
    fn new(outputs: Vec<u16>) -> Self {
        return Conjuction {
            prev: HashMap::new(),
            outputs: outputs,
        };
    }
}

impl Module for Conjuction {
    fn rx(&mut self, sender: u16, pulse: bool) -> Option<bool> {
        self.prev.insert(sender, pulse);
        if self.prev.values().all(|x| *x) {
            return Some(LOW);
        }
        return Some(HIGH);
    }
    fn outputs(&self) -> &Vec<u16> {
        return &self.outputs;
    }

    fn reset(&mut self) {
        let keys: Vec<u16> = self.prev.keys().cloned().collect();
        for k in keys {
            self.prev.insert(k, false);
        }
    }
}

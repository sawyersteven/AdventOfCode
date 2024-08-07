use crate::{
    intcode::{self, Emulator, StatusCode},
    Base,
};
use std::fmt::Display;

const NET_SIZE: usize = 50;

pub struct Day23 {
    input: Vec<isize>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: Vec::new() };
    }

    fn make_network(&self) -> Vec<Emulator> {
        let mut network = Vec::new();

        for i in 0..NET_SIZE {
            let mut emu = intcode::Emulator::new(self.input.clone());
            emu.queue_input(&[i as isize]);
            network.push(emu);
        }
        return network;
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut network = self.make_network();
        loop {
            for i in 0..network.len() {
                let response = network[i].run();
                if response.0 == StatusCode::InputRequest {
                    network[i].queue_input(&[-1]);
                } else if response.0 == StatusCode::OutputDelivery {
                    let destination = response.1;
                    let x = network[i].run().1;
                    let y = network[i].run().1;
                    if destination == 255 {
                        return Box::new(y);
                    }
                    network[destination as usize].queue_input(&[x, y]);
                } else {
                    panic!();
                }
            }
        }
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut network = self.make_network();
        let mut nat_packet = (0, 0);
        let mut last_nat_y = -1;
        loop {
            let mut network_idle = true;

            for i in 0..network.len() {
                let response = network[i].run();
                if response.0 == StatusCode::InputRequest {
                    network[i].queue_input(&[-1]);
                } else if response.0 == StatusCode::OutputDelivery {
                    network_idle = false;
                    let destination = response.1;
                    if destination == 255 {
                        nat_packet = (network[i].run().1, network[i].run().1);
                    } else {
                        let a = network[i].run().1;
                        let b = network[i].run().1;
                        network[destination as usize].queue_input(&[a, b]);
                    }
                } else {
                    panic!();
                }
            }

            if network_idle {
                network[0].queue_input(&[nat_packet.1, nat_packet.1]);
                if last_nat_y == nat_packet.0 {
                    return Box::new(last_nat_y);
                }
                last_nat_y = nat_packet.0;
            }
        }
    }
}

use crate::{intcode, Base};
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

const FORBIDDEN: [&str; 5] = [
    "molten lava",
    "photons",
    "infinite loop",
    "giant electromagnet",
    "escape pod",
];

const DIRECTIONS: [&str; 4] = ["north", "east", "south", "west"];

const LINEFEED: isize = 10;

pub struct Day25 {
    input: Vec<isize>,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { input: Vec::new() };
    }

    fn send_string(&self, emu: &mut intcode::Emulator, instructions: &str) -> isize {
        let mut inst: Vec<isize> = instructions.bytes().map(|x| x as isize).collect();
        inst.push(LINEFEED);
        emu.queue_input(&inst);
        return emu.run().1;
    }

    fn explore_map(&self, mut emu: &mut intcode::Emulator) -> String {
        let mut out_buffer = VecDeque::new();

        let mut current_dir = 0;
        let mut sec_check = false;

        loop {
            let resp = emu.run();
            if resp.0 == intcode::StatusCode::InputRequest {
                let report: String = out_buffer.drain(0..).map(|x| x as char).collect();
                out_buffer.clear();
                let doors: Vec<&str> = report.split("lead:").nth(1).unwrap().split("\n\n").nth(0).unwrap()[3..]
                    .split("\n- ")
                    .collect();

                if report.contains(&"here:") {
                    let items: Vec<&str> = report.split("here:").nth(1).unwrap().split("\n\n").nth(0).unwrap()
                        [3..]
                        .split("\n- ")
                        .collect();
                    for item in items {
                        if FORBIDDEN.contains(&item) {
                            continue;
                        }

                        self.send_string(&mut emu, &*format!("take {}", item));
                    }
                }

                if report.contains("Security Checkpoint") {
                    if sec_check == false {
                        sec_check = true;
                        current_dir = (current_dir + 2) % 4;
                        self.send_string(&mut emu, DIRECTIONS[current_dir]);
                        continue;
                    } else {
                        return self.get_next_direction(current_dir, &doors).1;
                    }
                }

                let next_room: String;
                (current_dir, next_room) = self.get_next_direction(current_dir, &doors);

                self.send_string(&mut emu, &next_room);
                emu.run();
            }

            out_buffer.push_back(resp.1 as u8);
        }
    }

    fn get_next_direction(&self, mut current_dir: usize, doors: &[&str]) -> (usize, String) {
        loop {
            let left_hand_door = DIRECTIONS[(current_dir + 3) % 4];
            if !doors.contains(&left_hand_door) {
                current_dir = (current_dir + 1) % 4;
                continue;
            }
            return ((current_dir + 3) % 4, left_hand_door.to_string());
        }
    }

    fn run_until_done(&self, emu: &mut intcode::Emulator) -> (intcode::StatusCode, String) {
        let mut output_buffer = String::new();
        loop {
            let r = emu.run();
            if r.0 != intcode::StatusCode::OutputDelivery {
                return (r.0, output_buffer);
            }
            output_buffer.push(r.1 as u8 as char);
        }
    }

    fn permutate_inventory(&self, inventory: &HashSet<String>) -> impl Iterator<Item = HashSet<String>> {
        let inventory = inventory.clone();
        let mut mask = 2usize.pow(inventory.len() as u32);
        let inv: Vec<String> = inventory.iter().cloned().collect();

        std::iter::from_fn(move || {
            let mut permutation = HashSet::new();
            mask -= 1;
            while mask > 0 {
                let mut m = 1;
                for i in 0..inventory.len() {
                    if (mask & m) == m {
                        permutation.insert(inv[i].clone());
                    }
                    m <<= 1;
                }
                return Some(permutation);
            }
            return None;
        })
    }

    fn try_sec_door(&self, mut emu: &mut intcode::Emulator, door_direction: String) -> String {
        self.send_string(emu, "inv");
        let output = self.run_until_done(&mut emu).1;
        let mut inventory = HashSet::new();
        for line in output.split(10 as char) {
            if line.starts_with('-') {
                inventory.insert(line[2..].to_string());
            }
        }

        for i in self.permutate_inventory(&inventory) {
            let mut remove: HashSet<String> = HashSet::from_iter(inventory.iter().cloned());
            remove.retain(|x| !i.contains(x));
            for r in remove {
                self.send_string(&mut emu, &format!("drop {}", r));
                self.run_until_done(&mut emu);
            }

            let mut add = HashSet::from(i.clone());
            add.retain(|x| inventory.contains(x));
            for a in add {
                self.send_string(&mut emu, &format!("take {}", a));
                self.run_until_done(&mut emu);
            }

            inventory = HashSet::from(i);

            self.send_string(&mut emu, &door_direction);
            let resp = self.run_until_done(&mut emu);
            if resp.0 != intcode::StatusCode::InputRequest {
                return resp
                    .1
                    .split("typing ")
                    .nth(1)
                    .unwrap()
                    .split(' ')
                    .nth(0)
                    .unwrap()
                    .to_string();
            }
        }
        return String::new();
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emu = intcode::Emulator::new(self.input.clone());

        let dir = self.explore_map(&mut emu);

        let s = self.try_sec_door(&mut emu, dir);
        return Box::new(s);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

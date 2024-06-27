use itertools::Itertools;

use crate::{
    intcode::{self, Emulator, StatusCode},
    Base,
};
use std::fmt::Display;

pub struct Day07 {
    code: Vec<isize>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { code: Vec::new() };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        self.code = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.code.clone());
        let mut max_sig = 0;

        // itertools' permutations is very slow, but until stable rust
        // has generators/coroutines it will have to do
        for perm in [0, 1, 2, 3, 4].iter().permutations(5) {
            let mut resp = (StatusCode::Null, 0);
            for s in perm {
                emulator.reboot();
                emulator.queue_input(&[*s, resp.1]);
                resp = emulator.run();
            }
            max_sig = max_sig.max(resp.1);
        }

        return Box::new(max_sig);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut thrusters: [Emulator; 5] = [
            Emulator::new(self.code.clone()),
            Emulator::new(self.code.clone()),
            Emulator::new(self.code.clone()),
            Emulator::new(self.code.clone()),
            Emulator::new(self.code.clone()),
        ];

        let mut max_sig = 0;
        for perm in [5, 6, 7, 8, 9].iter().permutations(5) {
            let sig = run_feedback_loop(&mut thrusters, &perm);
            max_sig = max_sig.max(sig);
        }

        return Box::new(max_sig);
    }
}

fn run_feedback_loop(thrusters: &mut [Emulator; 5], settings: &Vec<&i32>) -> isize {
    let mut resp = (StatusCode::Null, 0);
    for i in 0..thrusters.len() {
        thrusters[i].reboot();
        thrusters[i].expand_mem(20_000_000); // not a typo, 20 mil
        thrusters[i].queue_input(&[*settings[i] as isize, resp.1]);
        resp = thrusters[i].run();
    }

    let mut current_emu_num = 0;
    loop {
        let emu = &mut thrusters[current_emu_num];
        emu.queue_input(&[resp.1]);
        resp = emu.run();

        if resp.0 == StatusCode::Complete && current_emu_num == (thrusters.len() - 1) {
            return resp.1;
        }
        current_emu_num += 1;
        current_emu_num %= thrusters.len();
    }
}

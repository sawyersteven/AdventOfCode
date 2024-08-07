use crate::{
    intcode::{self, Emulator},
    Base,
};
use std::fmt::Display;

const SCAFFOLD: bool = true;
const _EMPTY: bool = false;
const LINEFEED: isize = 10;

pub struct Day17 {
    input: Vec<isize>,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 { input: Vec::new() };
    }

    fn send_string(&self, emu: &mut intcode::Emulator, instructions: &str) -> isize {
        let mut inst: Vec<isize> = instructions.bytes().map(|x| x as isize).collect();
        inst.push(LINEFEED);
        emu.queue_input(&inst);
        return emu.run().1;
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());
        emulator.expand_mem(1800);
        let mut grid = Vec::new();
        let mut row = Vec::new();
        loop {
            let resp = emulator.run();
            if resp.1 == 10 {
                if row.len() == 0 {
                    break;
                }
                grid.push(row);
                row = Vec::new();
                continue;
            }
            row.push(resp.1 == 0x23);
        }

        let mut align_sum = 0;
        for r in 1..(grid.len() - 1) {
            for c in 1..(grid[0].len() - 1) {
                if grid[r][c] == SCAFFOLD
                    && grid[r + 1][c]
                    && grid[r - 1][c]
                    && grid[r][c + 1]
                    && grid[r][c - 1] == SCAFFOLD
                {
                    align_sum += c * r;
                }
            }
        }

        return Box::new(align_sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut emulator = Emulator::new(self.input.clone());
        emulator.expand_mem(1800);

        // Complete program:
        // L,10,L,12,R,6 | R,10,L,4,L,4,L,12 | L,10,L,12,R,6 | R,10,L,4,L,4,L,12 | L,10,L,12,R,6 | L,10,R,10,R,6,L,4 | R,10,L,4,L,4,L,12 | L,10,R,10,R,6,L,4 | L,10,L,12,R,6 | L,10,R,10,R,6,L,4
        // A                                   A                                   A                                                                           A
        // L,10,L,12,R,6
        //                 B                                   B                                                       B
        //                 R,10,L,4,L,4,L,12
        //                                                                                         C                                       C                                   C
        //                                                                                         L,10,R,10,R,6,L,4
        // A,B,A,B,A,C,B,C,A,C

        const MAIN: &str = "A,B,A,B,A,C,B,C,A,C";
        const A: &str = "L,10,L,12,R,6";
        const B: &str = "R,10,L,4,L,4,L,12";
        const C: &str = "L,10,R,10,R,6,L,4";

        emulator.run();
        self.send_string(&mut emulator, MAIN);
        emulator.run();
        self.send_string(&mut emulator, A);
        emulator.run();
        self.send_string(&mut emulator, B);
        emulator.run();
        self.send_string(&mut emulator, C);
        emulator.run();
        self.send_string(&mut emulator, "y");

        loop {
            let resp = emulator.run();
            if resp.0 == intcode::StatusCode::Complete {
                return Box::new(resp.1);
            }
        }
    }
}

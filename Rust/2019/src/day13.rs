use crate::{
    intcode::{self, Emulator, Response, StatusCode},
    Base,
};
use std::fmt::Display;

const DISP_W: usize = 40;
const DISP_H: usize = 24;
const TILES: [u8; 5] = [b' ', b'+', b'#', b'=', b'@'];

pub struct Day13 {
    display: [[u8; DISP_W]; DISP_H],
    code: Vec<isize>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 {
            display: [[b' '; DISP_W]; DISP_H],
            code: Vec::new(),
        };
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        self.code = intcode::parse_code(&raw_input);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut emu = Emulator::new(self.code.clone());
        let mut response: (StatusCode, isize);

        let mut block_tiles = 0;

        loop {
            response = emu.run();
            if response.0 == StatusCode::Complete {
                break;
            }
            let x = response.1;
            let y = emu.run().1;
            let tile = emu.run().1;
            self.display[y as usize][x as usize] = TILES[tile as usize];
            if tile == 2 {
                block_tiles += 1;
            }
        }
        return Box::new(block_tiles);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut code = self.code.clone();
        code[0] = 2;
        let mut emu = Emulator::new(code);
        let mut response: Response;

        let mut ball_x = 0;
        let mut paddle_x = 0;

        // make grid
        for _ in 0..(DISP_H * DISP_W) {
            let x = emu.run().1;
            let _y = emu.run().1;
            let tile = emu.run().1;
            if tile == 4 {
                ball_x = x;
            }
            if tile == 3 {
                paddle_x = x;
            }
        }

        // play game. Basically tracks position of ball and moves paddle
        // to be under it at all times
        loop {
            response = emu.run();
            if response.0 == StatusCode::Complete {
                return Box::new(response.1);
            }
            if response.0 == StatusCode::InputRequest {
                emu.queue_input(&[ball_x.cmp(&paddle_x) as isize]);
                emu.run();
            }
            let x = response.1;
            let _y = emu.run().1;
            let z = emu.run().1;
            if x == -1 {
                continue;
            }
            if z == 4 {
                ball_x = x;
            }
            if z == 3 {
                paddle_x = x
            };
        }
    }
}

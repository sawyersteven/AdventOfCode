use itertools::Itertools;

use crate::Base;
use std::fmt::Display;

pub struct Day25 {
    input: Vec<Vec<u8>>,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { input: Vec::new() };
    }

    fn snafu_to_dec(&self, value: &[u8]) -> isize {
        let mut dec = 0;
        for (place, num) in value.iter().rev().enumerate() {
            let val = match num {
                b'=' => -2,
                b'-' => -1,
                b'0' => 0,
                b'1' => 1,
                b'2' => 2,
                _ => unreachable!(),
            };
            dec += val * 5_isize.pow(place as u32)
        }
        return dec;
    }

    fn dec_to_snafu(&self, value: isize) -> String {
        let mut value = value;
        // Create unbalanced quinary where digits are 0..=4
        let mut balanced = Vec::new();
        while value > 0 {
            let rem = value % 5;
            balanced.push(rem);
            value = (value - rem) / 5;
        }
        //balanced.push(0);

        let mut dec_chars = vec![' '; balanced.len()];
        for i in 0..(balanced.len()) {
            if balanced[i] == 5 {
                balanced[i] = 0;
                balanced[i + 1] += 1;
                dec_chars[balanced.len() - 1 - i] = '0';
                continue;
            }
            dec_chars[balanced.len() - 1 - i] = match balanced[i] {
                4 => {
                    balanced[i + 1] += 1;
                    '-'
                }
                3 => {
                    balanced[i + 1] += 1;
                    '='
                }
                2 => '2',
                1 => '1',
                0 => '0',
                _ => unreachable!(),
            }
        }

        let snafu = dec_chars.iter().collect::<String>();
        return snafu.trim_start_matches(|x| x == ' ').to_string();
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            self.input.push(line.bytes().collect_vec());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let dec_fuel: isize = self.input.iter().map(|x| self.snafu_to_dec(x)).sum();

        return Box::new(self.dec_to_snafu(dec_fuel));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("🌟");
    }
}

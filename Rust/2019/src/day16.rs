use crate::Base;
use std::fmt::Display;

pub struct Day16 {
    signal: Vec<isize>,
    p2offset: isize,
}

const PATTERN: [isize; 4] = [0, 1, 0, -1];

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            signal: Vec::new(),
            p2offset: 0,
        };
    }

    pub fn decode_message(&self, signal: &[isize]) -> Vec<isize> {
        let mut result = vec![0; signal.len()];

        for row in 0..signal.len() as isize {
            let mut output = 0;

            let mut pattern_offset = 0;
            for col in 0..signal.len() as isize {
                if (col % (row + 1)) - row == 0 {
                    pattern_offset = (pattern_offset + 1) % PATTERN.len();
                }
                output += signal[col as usize] * PATTERN[pattern_offset];
            }
            result[row as usize] = output.abs() % 10;
        }
        return result;
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        self.signal = raw_input.as_bytes().iter().map(|x| (x - b'0') as isize).collect();
        self.p2offset = raw_input[0..7].parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut signal = self.signal.clone();
        for _ in 0..100 {
            for (i, r) in self.decode_message(&signal).iter().enumerate() {
                signal[i] = *r;
            }
        }

        let ans = signal
            .iter()
            .take(8)
            .map(|x| (*x as u8 + b'0') as char)
            .collect::<String>();

        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        /*
        This novella was copied from my c# project...

        Ok so this is a doozy. I don't usually write out the thought process
        for these since the code makes it obvious, but this was a real chore to
        completely understand and I want to document it.

        After the halfway point in the input signal, the matching pattern is
        N zeros and M ones where N is the row index and M is signal.Length - N.

        eg with a 10-digit signal length

        0    1 0-1 0 1 0-1 0 1 0
        1    0 1 1 0 0-1-1 0 0 1
        2    0 0 1 1 1 0 0 0-1-1
        3    0 0 0 1 1 1 1 0 0 0
        4    0 0 0 0 1 1 1 1 1 0
        * Halfway *
        5    0 0 0 0 0 1 1 1 1 1 (5 zeros)
        6    0 0 0 0 0 0 1 1 1 1 (6 zeros)
        7    0 0 0 0 0 0 0 1 1 1 (etc...)
        8    0 0 0 0 0 0 0 0 1 1
        9    0 0 0 0 0 0 0 0 0 1

        This means several things:

        The first half of the signal has no effect on the second half of the
        result. The second half numbers can be processed without even looking
        at the first half, assuming the offset for finding the answer is
        greater than signal.Length / 2;

        Since the only numbers contributing to the answer are multiple of 1,
        there is no need to multiply anything. We can simply add the signal
        numbers starting from the row index and take the ones digit.

        To make this easier we can pretend the first half of the signal
        doesn't even exist. Additionally, we can ignore all of the signal *before*
        the offset position. The patterns remains the same, starting with all
        ones, then [0, 1...] and so on.

        Working backward, we keep a running total of the sigal's values.
        Where i == signal.Length - 1, we can add signal[i] to the total, then
        replace signal[i] with the ones digit of the total. Continue backward
        until the signal is consumed to complete the phase.

        */

        const MULT: usize = 10_000;

        let base_signal = self.signal.clone();

        let total_len = base_signal.len() * MULT;
        let mut signal = vec![0; total_len - self.p2offset as usize];

        let mut i = signal.len() as isize - 1;
        let mut j = base_signal.len() as isize - 1;

        while i > -1 {
            if j == -1 {
                j = base_signal.len() as isize - 1;
            }
            signal[i as usize] = base_signal[j as usize];

            i -= 1;
            j -= 1;
        }

        let mut signal_total = 0;
        for _ in 0..100 {
            let mut i = signal.len() as isize - 1;
            while i > -1 {
                signal_total += signal[i as usize];
                signal[i as usize] = signal_total % 10;
                i -= 1;
            }
            signal_total = 0;
        }

        let ans = signal
            .iter()
            .take(8)
            .map(|x| (*x as u8 + b'0') as char)
            .collect::<String>();
        return Box::new(ans);
    }
}

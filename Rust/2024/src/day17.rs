use crate::Base;
use std::{fmt::Display, usize};

const A: usize = 0;
const B: usize = 1;
const C: usize = 2;

pub struct Day17 {
    input: String,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 { input: String::new() };
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut registers: [usize; 3] = [0, 0, 0];
        let mut prog = Vec::new();
        for (i, line) in self.input.lines().enumerate() {
            match i {
                0 => registers[A] = line[12..].parse().unwrap(),
                1 => registers[B] = line[12..].parse().unwrap(),
                2 => registers[C] = line[12..].parse().unwrap(),
                4 => {
                    prog = line[9..]
                        .as_bytes()
                        .iter()
                        .filter(|x| **x != b',')
                        .map(|x| x - b'0')
                        .collect()
                }
                _ => {}
            }
        }

        let out = run_code(&prog, &mut registers);
        let out: String = out.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(",");
        return Box::new(out);
    }

    /// This will only work with my input because I rewrote it in rust, but the
    ///  concept should work for any if you replace get_output()
    ///
    /// The only register that matters is A. B is always overwritten at the
    /// start of the loop, and C only holds a temporary value.
    ///
    /// Each output is determined by using a formula that takes as input the
    /// last 3 bits of A and 3 bits anywhere between 0 and 7 bits to the
    /// left of those 3. Then A is shifted right by 3 and it starts anew.
    ///
    /// Working backward from the end of the desired output, reg A *must* be
    /// all zero bits except the lowest 3. Eg: 00000111 or 00000101
    ///
    /// More than one input A value can give us the correct output value.
    ///
    /// Knowing this, the concept is this:
    ///
    /// Find the last output value but processing 0..=7 and collecting any
    /// that result in the correct output (the last digit of the program)
    ///
    /// For each of those, shift them left 3 places, then add 0..=7. Like
    /// above, collect the correct outputs (2nd to last digit).
    ///
    /// Continue this until all correct outputs are found. This may still
    /// result in several answers. We need the lowest.
    ///
    /// This is, in a certain sense, a type of pathfinding.
    ///
    fn part2(&mut self) -> Box<dyn Display> {
        let prog: Vec<u8> = self.input.lines().skip(4).next().unwrap()[9..]
            .as_bytes()
            .iter()
            .filter(|x| **x != b',')
            .map(|x| x - b'0')
            .collect();

        fn get_output(a: usize) -> u8 {
            let mut b = a.rem_euclid(8) ^ 7;
            b ^= a >> b;
            b ^= 7;
            return b.rem_euclid(8) as u8;
        }

        let mut a_vals = vec![0];
        let mut t_vals = Vec::new();
        for out_val in prog.iter().rev() {
            for a in &a_vals {
                for i in 0..=7 {
                    let test_a = (a << 3) + i;

                    if get_output(test_a) == *out_val {
                        t_vals.push(test_a);
                    }
                }
            }
            a_vals.clear();
            a_vals = t_vals.clone();
            t_vals.clear();
        }

        return Box::new(*a_vals.iter().min().unwrap());
    }
}

fn run_code(prog: &Vec<u8>, registers: &mut [usize; 3]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut inst = 0;
    while inst < prog.len() {
        let opcode = prog[inst];
        let operand = (prog[inst + 1]) as usize;
        match opcode {
            0 => registers[A] /= 2usize.pow(combo(operand, &registers) as u32), // adv
            1 => registers[B] ^= operand,                                       // bxl
            2 => registers[B] = combo(operand, &registers).rem_euclid(8),       // bst
            3 => {
                if registers[A] != 0 {
                    inst = operand;
                    inst -= 2;
                }
            } // jnz
            4 => registers[B] ^= registers[C],                                  // bxc
            5 => out.push(combo(operand, &registers).rem_euclid(8) as u8),      // out
            6 => registers[B] = registers[A] / 2usize.pow(combo(operand, &registers) as u32), // bdv
            7 => registers[C] = registers[A] / 2usize.pow(combo(operand, &registers) as u32), // cdv
            _ => unreachable!(),
        }
        inst += 2;
    }
    return out;
}

#[inline(always)]
fn combo(o: usize, registers: &[usize; 3]) -> usize {
    return match o {
        n if n <= 3 => n,
        n if n < 7 => registers[n - 4],
        _ => unreachable!(),
    };
}

/*
Program transcription for part 2. This may not have been entirely neccesary, but
I like these kinds of puzzles. This would likely run pt2 just as quickly using
the pt1 loop

IND| INST| Cmd| CVal| Code
  0|    2| bst|     | r[B] = OP_COMBO.rem_euclid(8)
  1|    4|    | r[A]|
  2|    1| bxl|     | r[B] ^= OP_LITERAL
  3|    7|    |   ! |
  4|    7| cdv|     | r[C] = r[A] / 2usize.pow(OP_COMBO)
  5|    5|    | r[B]|
  6|    0| adv|     | r[A] /= 2usize.pow(OP_COMBO)
  7|    3|    |   3 |
  8|    4| bxc|     | r[B] ^= r[C]
  9|    4|    | r[A]|
 10|    1| bxl|     | r[B] ^= OP_LITERAL
 11|    7|    |   ! |
 12|    5| out|     | out.push(OP_COMBO.rem_euclid(8))
 13|    5|    | r[B]|
 14|    3| jnz|     | ind = if r[A] == 0 {ind} else {next_literal - 2}
 15|    0|    |   0 |

// Clean up operands

IND| INST| Cmd| CVal| Code
  0|    2| bst|     | r[B] = r[A].rem_euclid(8)
  1|    4|    | r[A]|
  2|    1| bxl|     | r[B] ^= 7
  3|    7|    |   ! |
  4|    7| cdv|     | r[C] = r[A] / 2usize.pow(r[B])
  5|    5|    | r[B]|
  6|    0| adv|     | r[A] /= 2usize.pow(3)
  7|    3|    |   3 |
  8|    4| bxc|     | r[B] ^= r[C]
  9|    4|    | r[A]|
 10|    1| bxl|     | r[B] ^= 7
 11|    7|    |   ! |
 12|    5| out|     | out.push(r[B].rem_euclid(8))
 13|    5|    | r[B]|
 14|    3| jnz|     | ind = if r[A] == 0 {ind} else {next_literal - 2}
 15|    0|    |   0 |

// Make full code:
loop{
    r[B] = r[A].rem_euclid(8)
    r[B] ^= 7
    r[C] = r[A] / 2usize.pow(r[B])
    r[A] /= 8
    r[B] ^= r[C]
    r[B] ^= 7
    out.push(r[B].rem_euclid(8))
    if r[A] == 0{
        break;
    }
}
*/

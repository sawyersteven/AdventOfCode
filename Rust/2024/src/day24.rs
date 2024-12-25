use crate::Base;
use std::collections::HashSet;
use std::fmt::Display;
use std::{ops::*, u8};

pub struct Day24 {
    input: String,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 { input: String::new() };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut gates = Vec::new();
        let mut found = Vec::new();

        let mut lines = self.input.lines();
        while let Some(l) = lines.next() {
            if l == "" {
                lines.next();
                break;
            }
            found.push((&l[0..3], l.as_bytes()[5] - b'0'));
        }

        while let Some(l) = lines.next() {
            let mut words = l.split(' ');
            let o = Gate {
                a_wire: words.next().unwrap(),
                input_val: u8::MAX,
                op: match words.next().unwrap() {
                    "AND" => u8::bitand,
                    "OR" => u8::bitor,
                    "XOR" => u8::bitxor,
                    _ => unreachable!(),
                },
                b_wire: words.next().unwrap(),
                out: words.skip(1).next().unwrap(),
            };
            gates.push(o);
        }

        return Box::new(solve(&mut found, &mut gates));
    }

    /* To understand part 2, one must first understand how a ripple adder works
        Well, not entirely, you just need to understand what makes a ripple adder
        NOT work.

        A ripple carry adder looks like this in (pseudo)code, where a and b are the bits
        being added, and c is the carry-over from the previous bit (or zero if first)

    fn rca(a: bit, b: bit, c: bit) -> (sum: bit, carry: bit){
        sum = a ^ b ^ c;
        carry = (a & b) | (b & c) | (c & a);
        return (sum, carry)
    }

        Comments in the code below explain how this can be used to find faulty
        output names in the puzzle input.


        For this puzzle the pairs of bad outputs are irrelevant as they can be
        found individually by looking at their operation and bit index.
         */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut gates = Vec::new();

        let mut max_bit = "z00";

        let mut lines = self.input.lines();
        while let Some(l) = lines.next() {
            if l == "" {
                lines.next();
                break;
            }
        }

        while let Some(l) = lines.next() {
            let mut words = l.split(' ');
            let o = Gate {
                a_wire: words.next().unwrap(),
                input_val: u8::MAX,
                op: match words.next().unwrap() {
                    "AND" => u8::bitand,
                    "OR" => u8::bitor,
                    "XOR" => u8::bitxor,
                    _ => unreachable!(),
                },
                b_wire: words.next().unwrap(),
                out: words.skip(1).next().unwrap(),
            };

            if o.out.starts_with('z') {
                max_bit = max_bit.max(o.out);
            }
            gates.push(o);
        }

        let mut bad_outputs = HashSet::new();
        for gate in &gates {
            // The result of adding two bits is (a ^ b ^ c)
            // Therefore, AND and OR cannot write to an output unless it is
            // the highest bit in the output number.
            if gate.out.starts_with('z') && gate.op != u8::bitxor && gate.out != max_bit {
                bad_outputs.insert(gate.out);
                continue;
            }

            // OR cannot go to another OR.
            if gate.op == u8::bitxor {
                for gate2 in &gates {
                    if (gate.out == gate2.a_wire || gate.out == gate2.b_wire) && gate2.op == u8::bitor {
                        bad_outputs.insert(gate.out);
                        continue;
                    }
                }
                // XOR ops can only involve inputs (x, y) or outputs (z) as nothing
                // else must be calculated for their input
                if gate.out.as_bytes()[0] < b'x'
                    && gate.a_wire.as_bytes()[0] < b'x'
                    && gate.b_wire.as_bytes()[0] < b'x'
                {
                    bad_outputs.insert(gate.out);
                    continue;
                }
            }

            // The result of an AND must go directly to an OR unless it is the first input
            if gate.op == u8::bitand && gate.a_wire != "x00" && gate.b_wire != "x00" {
                for gate2 in &gates {
                    if (gate.out == gate2.a_wire || gate.out == gate2.b_wire) && gate2.op != u8::bitor {
                        bad_outputs.insert(gate.out);
                        continue;
                    }
                }
            }
        }

        let mut bad_outputs: Vec<&str> = bad_outputs.iter().cloned().collect();
        bad_outputs.sort();
        return Box::new(bad_outputs.join(","));
    }
}

fn solve<'a, F>(found: &mut Vec<(&'a str, u8)>, gates: &mut Vec<Gate<'a, F>>) -> u64
where
    F: Fn(u8, u8) -> u8,
{
    let mut output = 0u64;
    while let Some((name, value)) = found.pop() {
        if name.starts_with('z') {
            let mut ind = 0;
            ind += name[2..3].parse::<usize>().unwrap();
            ind += name[1..2].parse::<usize>().unwrap() * 10;
            output |= (value as u64) << ind;
            continue;
        }

        let mut i = 0;
        while i < gates.len() {
            let gate = &mut gates[i];

            if gate.a_wire == name || gate.b_wire == name {
                if gate.input_val == u8::MAX {
                    gate.input_val = value;
                } else {
                    let ans = (gate.op)(gate.input_val, value);
                    found.push((gate.out, ans));
                    gates.remove(i);
                    i -= 1;
                }
            }
            i += 1;
        }
    }
    return output;
}

// AND, OR, and XOR don't care about the order of arguments.
// a OP b will always equal b OP a, so which wire gets which input doesn't
// matter. We only need to care when we have two input values to use
#[derive(Clone)]
struct Gate<'a, F>
where
    F: Fn(u8, u8) -> u8,
{
    a_wire: &'a str,
    b_wire: &'a str,
    input_val: u8,
    op: F,
    out: &'a str,
}

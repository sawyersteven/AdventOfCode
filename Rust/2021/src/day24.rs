use crate::Base;
use std::{collections::VecDeque, fmt::Display};

const INST_COUNT: usize = 18;
const PEEK: isize = 1;

/*
Probably my least favorite reverse-engineering puzzle.

There are 14 instructions that may or may not push a value to a stack

There are 7 instruction that push values. These instructions will always
push an int into the stack. The values are the instruction's
input digit + an int specific to that instruction.

There are 7 instruction that pop a value from the stack. These may also push
depending on the popped value - an int specific to that instruction.

The check at the end of the procedure (z == 0) is effectively checking
that the stack is empty since zero cannot be pushed to the stack (pushes
are always digit+N; digits cannot be zero and N is never be negative).

This means that a pop instruction may not be allowed to push. Any more
than the mandatory 7 pushes will leave a value at the end.

Knowing this, each pop instruction in a valid input number will only ever
pop a specific push instruction. For example, in my input:

0 push
1 push
2 push
3 pop
4 push
5 pop
6 push
7 push
8 push
9 pop
0 pop
1 pop
2 pop
3 pop

The first pop is 3, which must match with 2. Instruction 5 must pop 4. 9 pops 8,
10 pops 7 (8 was already taken), 11 pops 6 (8 and 7 taken) and so on

With the pairs determined the relationship between the values remains.

A pushed value is the digit + the instructions 'b' parameter seen near the end of
the block as `add y N`

When that value is later popped, the parameter 'a' is subtracted from it `add x -N`
If this modified value does not equal the pop instruction's digit a new value is
pushed and the stack will become invalid.

The formula then to see if a digit is valid is:

push_inst_digit + Nb - Na = pop_inst_digit
or
low_ind_digit + D = high_ind_digit

In order to find the largest possible valid input number, the number on the
right side of the equation must be the largest digit that satisfies the
constraint, being 9 - max(D, 0) because D can be negative at this point.
This means the left hand side (the value of the lower-signifigcance digit)
is 9 - max(D, 0) + D

So to solve, parse the instructions for (is_pop, param_a, param_b)

Find pairs and deltas to get (low_ind, high_ind, delta)

Iter through pairs to find largest possible value for each high_ind
and build the biggest possible number.

So that's a novel, but its a convoluted logic used in the ALU code
and I wasn't having a good time.

*/

pub struct Day24 {
    //                  is_peek,     A,     B
    pub instructions: Vec<(bool, isize, isize)>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            instructions: Vec::with_capacity(INST_COUNT),
        };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        for i in (0..lines.len()).step_by(18) {
            let mut set = (false, 0, 0);
            set.0 = lines[i + 4][6..].trim_end().parse::<isize>().unwrap() == PEEK;
            set.1 = lines[i + 5][6..].trim_end().parse().unwrap();
            set.2 = lines[i + 15][6..].trim_end().parse().unwrap();
            self.instructions.push(set);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let pairs = self.find_pairs();

        let mut digits = [0; 14];
        for (l, h, d) in pairs {
            digits[l] = 9 - d.max(0);
            digits[h] = digits[l] + d;
        }
        let mut num = 0;
        for digit in digits {
            num *= 10;
            num += digit;
        }

        return Box::new(format!("{:?}", num));
    }

    fn part2(&self) -> Box<dyn Display> {
        let pairs = self.find_pairs();

        let mut digits = [0; 14];
        for (l, h, d) in pairs {
            digits[l] = 1 - d.min(0);
            digits[h] = digits[l] + d;
        }
        let mut num = 0;
        for digit in digits {
            num *= 10;
            num += digit;
        }

        return Box::new(format!("{:?}", num));
    }
}

impl Day24 {
    //                        low_ind, high_ind, delta
    fn find_pairs(&self) -> Vec<(usize, usize, isize)> {
        let mut pairs = Vec::<(usize, usize, isize)>::new();
        let mut stack = VecDeque::<usize>::new();
        for (line, inst) in self.instructions.iter().enumerate() {
            if inst.0 {
                // peek op
                stack.push_back(line);
            } else {
                // pop op
                let small = stack.pop_back().unwrap();
                pairs.push((small, line, inst.1 + self.instructions[small as usize].2));
            }
        }
        return pairs;
    }
}

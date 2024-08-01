use crate::Base;
use std::fmt::Display;
use std::ops::*;

pub struct Day18 {
    lines: Vec<String>,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { lines: Vec::new() };
    }

    fn solve_line(&self, line: &String, pt2: bool) -> usize {
        let mut line = line.clone();
        loop {
            if !line.contains('(') {
                return if pt2 {
                    self.solve_expr_2(&line)
                } else {
                    self.solve_expr(&line)
                };
            }
            let mut open = 0;
            let mut close = 0;
            for (i, c) in line.chars().enumerate() {
                if c == '(' {
                    open = i;
                } else if c == ')' {
                    close = i;
                    break;
                }
            }

            let result = if pt2 {
                self.solve_expr_2(&line[(open + 1)..close])
            } else {
                self.solve_expr(&line[(open + 1)..close])
            };
            line.replace_range(open..=close, &*result.to_string());
        }
    }

    fn solve_expr(&self, expr: &str) -> usize {
        let mut answer = 0;

        let mut next_op: fn(usize, usize) -> usize = usize::add;
        for c in expr.split(' ') {
            match c.parse::<usize>() {
                Ok(num) => answer = next_op(answer, num),
                Err(_) => {
                    next_op = match c.as_bytes()[0] {
                        b'*' => usize::mul,
                        b'+' => usize::add,
                        _ => unreachable!(),
                    }
                }
            }
        }
        return answer;
    }

    /* This is kind of janky
    In order to parse the line as a vec of usize I needed to use a usize for
    the operation characters. This will fail in strange ways if the value of
    any number in the sequence is ever near the usize max limit.
    */
    fn solve_expr_2(&self, expr: &str) -> usize {
        const ADD: usize = usize::MAX;
        const MUL: usize = ADD - 1;
        const NOP: usize = MUL - 1;

        let mut parts: Vec<usize> = expr
            .split(' ')
            .map(|x| {
                if x == "+" {
                    ADD
                } else if x == "*" {
                    MUL
                } else {
                    x.parse().unwrap()
                }
            })
            .collect();

        let pl = parts.len();
        let mut i = 0;

        while i < parts.len() {
            if parts[i] == ADD {
                let a = parts[i - 1];
                let b = parts[i + 1];

                parts[i - 1] = a + b;
                parts[i] = NOP;
                parts[i + 1] = NOP;

                for j in (i + 2)..parts.len() {
                    parts[j - 2] = parts[j].clone();
                }

                parts[pl - 1] = NOP;
                parts[pl - 2] = NOP;
                i -= 1;
            }
            i += 1;
        }

        let mut result: usize = parts[0];
        for i in parts.iter().skip(1).filter(|&x| *x != NOP && *x != MUL) {
            result *= i;
        }

        return result;
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        self.lines = raw_input.lines().map(|x| x.to_string()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut ans = 0;
        for line in &self.lines {
            ans += self.solve_line(line, false);
        }
        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut ans = 0;
        for line in &self.lines {
            ans += self.solve_line(line, true);
        }
        return Box::new(ans);
    }
}

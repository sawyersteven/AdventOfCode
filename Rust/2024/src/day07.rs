use crate::Base;
use std::{fmt::Display, ops::*};

/// is val a concat of a,b
///un_concat(12345, 345) == 12
///un_concat(12345, 2345) == 1
///un_concat(12345, 305) = 0
fn un_concat(mut val: isize, mut tail: isize) -> isize {
    // Compare the digits of the ending with the number from the back
    while tail > 0 {
        if val % 10 != tail % 10 {
            return 0;
        }
        val /= 10;
        tail /= 10;
    }
    return val;
}

struct Equation {
    nums: Vec<isize>,
    ans: isize,
}

impl Equation {
    fn solve_p1(&self) -> bool {
        return self._solve_p1(self.ans, self.nums.len() - 1);
    }

    fn _solve_p1(&self, val: isize, depth: usize) -> bool {
        if depth == 0 {
            return val == self.nums[0];
        }
        if val % self.nums[depth] == 0 {
            let nval = val.div(self.nums[depth]);
            if self._solve_p1(nval, depth - 1) {
                return true;
            }
        }

        let nval = val.sub(self.nums[depth]);
        if self._solve_p1(nval, depth - 1) {
            return true;
        }

        return false;
    }

    fn solve_p2(&self) -> bool {
        return self._solve_p2(self.ans, self.nums.len() - 1);
    }

    fn _solve_p2(&self, val: isize, depth: usize) -> bool {
        if depth == 0 {
            return val == self.nums[0];
        }

        // div
        if val % self.nums[depth] == 0 {
            let n = val.div(self.nums[depth]);
            if self._solve_p2(n, depth - 1) {
                return true;
            }
        }

        // sub
        let n = val.sub(self.nums[depth]);
        if n >= self.nums[0] && self._solve_p2(n, depth - 1) {
            return true;
        }

        // un-concat
        let n = un_concat(val, self.nums[depth]);
        if n != 0 && self._solve_p2(n, depth - 1) {
            return true;
        }

        return false;
    }
}

pub struct Day07 {
    equations: Vec<Equation>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 { equations: Vec::new() };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let parts = line.split_once(':').unwrap();
            let ans = parts.0.parse().unwrap();
            let nums = parts.1.trim().split(' ').map(|x| x.parse().unwrap()).collect();
            self.equations.push(Equation { ans, nums });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        for eq in &self.equations {
            if eq.solve_p1() {
                t += eq.ans;
            }
        }
        return Box::new(t);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut t = 0;
        for eq in &self.equations {
            if eq.solve_p2() {
                t += eq.ans;
            }
        }
        return Box::new(t);
    }
}

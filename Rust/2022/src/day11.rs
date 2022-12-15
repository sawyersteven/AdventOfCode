use crate::Base;
use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Add, Mul},
};

fn lcm(a: usize, b: usize) -> usize {
    let (mut x, mut y) = (a.min(b), a.max(b));
    let mut rem = x % y;
    while rem != 0 {
        x = y;
        y = rem;
        rem = x % y;
    }
    return (a * b) / y;
}

#[derive(Clone)]
struct Monkey {
    items: VecDeque<usize>,
    test: usize,
    operator: fn(usize, usize) -> usize,
    op_b: Option<usize>,
    destinations: (usize, usize), // true/false branches
}

impl Monkey {
    fn operation(&self, old: usize) -> usize {
        return (self.operator)(old, self.op_b.unwrap_or(old));
    }
}

pub struct Day11 {
    monkeys: Vec<Monkey>,
    test_lcm: usize,
}

impl Day11 {
    pub fn new() -> Day11 {
        return Day11 {
            monkeys: Vec::with_capacity(8),
            test_lcm: 1,
        };
    }

    /* Didn't expect to write an explanation this early in the month, but so it goes.
    self.lcm is the lowest-common-multiple of all of the monkey test contditions.
    To keep the item field from growing too large, we can take item %= lcm to
    lower the value to a number that will still pass/fail the division test
    for each monkey.
    */
    fn sim_round(&mut self, counts: &mut Vec<usize>, worry: usize) {
        for i in 0..self.monkeys.len() {
            counts[i] += self.monkeys[i].items.len();
            let items: Vec<usize> = self.monkeys[i].items.drain(0..).collect();
            for item in items {
                let mut item = self.monkeys[i].operation(item) / worry;
                item %= self.test_lcm;
                let dest = match item % self.monkeys[i].test {
                    0 => self.monkeys[i].destinations.0,
                    _ => self.monkeys[i].destinations.1,
                };
                self.monkeys[dest].items.push_back(item);
            }
        }
    }
}

impl Base for Day11 {
    fn parse_input(&mut self, raw_input: String) {
        for monk in raw_input.split("\n\n") {
            let lines: Vec<&str> = monk.split('\n').collect();
            let items = lines[1]
                .split(':')
                .nth(1)
                .unwrap()
                .split(',')
                .map(|x| x.trim().parse().unwrap())
                .collect();

            let test = lines[3].split(' ').last().unwrap().parse().unwrap();

            let dest_a = lines[4].split(' ').last().unwrap().parse().unwrap();
            let dest_b = lines[5].split(' ').last().unwrap().parse().unwrap();

            let operator_line: Vec<&str> = lines[2].split(' ').collect();

            self.monkeys.push(Monkey {
                items: items,
                test: test,
                destinations: (dest_a, dest_b),
                operator: if operator_line[6] == "+" { Add::add } else { Mul::mul },
                op_b: operator_line[7].parse().ok(),
            });
            self.test_lcm = lcm(self.test_lcm, test);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        // clone and replace later so part 2 has a fresh set of monkeys
        let m = self.monkeys.clone();
        let mut counts = vec![0; self.monkeys.len()];
        for _ in 0..20 {
            self.sim_round(&mut counts, 3);
        }
        counts.sort();
        counts.reverse();
        self.monkeys = m;
        return Box::new(counts[0] * counts[1]);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut counts = vec![0; self.monkeys.len()];
        for _ in 0..10000 {
            self.sim_round(&mut counts, 1);
        }
        counts.sort();
        counts.reverse();
        return Box::new(counts[0] * counts[1]);
    }
}

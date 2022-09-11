use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    pub input: Vec<(isize, isize)>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { input: Vec::new() };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts: Vec<&str> = line.split(' ').collect();
            let p: isize = parts[3].parse().unwrap();
            let c: isize = parts[parts.len() - 1].replace('.', "").parse().unwrap();
            self.input.push((p, c - p));
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        return Box::new(run(&self.input));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut input = self.input.clone();
        input.push((11, 0));
        return Box::new(run(&input));
    }
}

fn least_common_multiple(a: isize, b: isize) -> isize {
    let (mut big, mut small) = if a > b { (a, b) } else { (b, a) };

    let mut rem = big % small;
    while rem != 0 {
        big = small;
        small = rem;
        rem = big % small;
    }
    return (a * b) / small;
}

fn run(discs: &Vec<(isize, isize)>) -> isize {
    let mut answer = 0;
    let mut lcm = 1;

    for i in 0..discs.len() {
        while (answer + discs[i].1 + i as isize + 1) % discs[i].0 != 0 {
            answer += lcm;
        }

        lcm = least_common_multiple(lcm, discs[i].0);
    }
    return answer;
}

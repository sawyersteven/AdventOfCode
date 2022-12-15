use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day09 {
    input: Vec<(u8, u8)>,
    dirs: HashMap<u8, (isize, isize)>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 {
            input: Vec::new(),
            dirs: HashMap::from([(b'U', (0, 1)), (b'D', (0, -1)), (b'L', (-1, 0)), (b'R', (1, 0))]),
        };
    }

    fn sim_rope(&self, rope_len: usize) -> HashSet<(isize, isize)> {
        let mut visited = HashSet::<(isize, isize)>::new();
        let mut rope = vec![(0, 0); rope_len];

        for (dir, dist) in &self.input {
            let d = self.dirs[dir];
            for _ in 0..*dist {
                rope[0].0 += d.0;
                rope[0].1 += d.1;
                move_tails(&mut rope);
                visited.insert(rope[rope_len - 1]);
            }
        }
        return visited;
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input.push((line.as_bytes()[0], line[2..].parse().unwrap()));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.sim_rope(2).len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.sim_rope(10).len());
    }
}

fn move_tails(rope: &mut Vec<(isize, isize)>) {
    for i in 0..(rope.len() - 1) {
        let xdiff = rope[i].0 - rope[i + 1].0;
        let ydiff = rope[i].1 - rope[i + 1].1;
        if xdiff.abs() <= 1 && ydiff.abs() <= 1 {
            continue;
        }
        rope[i + 1].0 += 1 * xdiff.signum();
        rope[i + 1].1 += 1 * ydiff.signum();
    }
}

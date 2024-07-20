use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    fmt::Display,
};

pub struct Day17 {
    map: Grid2D<u8>,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 { map: Grid2D::new(0, 0) };
    }

    fn solve(&self, min_len: usize, max_len: usize) -> usize {
        let sz = self.map.size().to_vector2();
        let goal = sz - Vector2Int::ONE;

        let mut hist = HashMap::new();

        // Start at zero facing either interior direction
        let mut pq = BinaryHeap::new();
        pq.push(Reverse(State {
            pos: Vector2Int::ZERO,
            dir: Vector2Int::RIGHT,
            heat_loss: 0,
            line_len: 0,
        }));
        pq.push(Reverse(State {
            pos: Vector2Int::ZERO,
            dir: Vector2Int::UP,
            heat_loss: 0,
            line_len: 0,
        }));

        let mut best = usize::MAX;

        while let Some(c) = pq.pop() {
            let current = c.0;

            if current.pos == goal {
                best = best.min(current.heat_loss);
                continue;
            }

            if current.line_len < max_len && (current.pos + current.dir).in_range(&Vector2Int::ZERO, &goal) {
                self.add(&current, current.dir, &mut hist, &mut pq);
            }

            if current.line_len > min_len {
                let dir = Vector2Int::new(current.dir.y, -current.dir.x);
                if (current.pos + dir).in_range(&Vector2Int::ZERO, &goal) {
                    self.add(&current, dir, &mut hist, &mut pq);
                }

                let dir = dir * -1;
                if (current.pos - dir).in_range(&Vector2Int::ZERO, &goal) {
                    self.add(&current, dir, &mut hist, &mut pq);
                }
            }
        }
        return best;
    }

    fn add(
        &self,
        current: &State,
        dir: Vector2Int,
        hist: &mut HashMap<(Vector2Int, Vector2Int, usize), usize>,
        pq: &mut BinaryHeap<Reverse<State>>,
    ) {
        let next_pos = current.pos + dir;

        let next = State {
            pos: next_pos,
            dir: dir,
            heat_loss: current.heat_loss + (self.map[next_pos] as usize),
            line_len: if current.dir == dir { current.line_len + 1 } else { 1 },
        };

        if *hist.get(&(next_pos, dir, next.line_len)).unwrap_or(&usize::MAX) <= next.heat_loss {
            return;
        }

        hist.insert((next_pos, dir, next.line_len), next.heat_loss);

        pq.push(Reverse(next));
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
        let sz = self.map.size();
        for y in 0..sz.y {
            for x in 0..sz.x {
                let v = self.map.get(x, y);
                self.map.set(x, y, v - b'0');
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.solve(0, 3));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.solve(3, 10));
    }
}

#[derive(PartialEq, PartialOrd, Eq)]
struct State {
    pos: Vector2Int,
    dir: Vector2Int,
    heat_loss: usize,
    line_len: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        return self.heat_loss.cmp(&other.heat_loss);
    }
}

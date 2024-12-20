use shared::{bitset::Bitset2D, v2i::Vector2Int};

use crate::Base;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, VecDeque},
    fmt::Display,
    hash::Hash,
    usize,
};

pub struct Day16 {
    input: String,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 { input: String::new() };
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut end = Vector2Int::ZERO;
        let mut start = Vector2Int::ZERO;

        let sz = (self.input.len() as f64).sqrt() as usize;
        let mut bwalls = Bitset2D::new(sz, sz);

        for (y, line) in self.input.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => {
                        bwalls.set(x, y);
                    }
                    b'E' => end = Vector2Int::new(x as isize, y as isize),
                    b'S' => start = Vector2Int::new(x as isize, y as isize),
                    _ => {}
                }
            }
        }

        let mut heap = BinaryHeap::new();
        heap.push(Reverse((
            0,
            State {
                p: start,
                d: Vector2Int::RIGHT,
            },
        )));
        let mut costs: HashMap<State, usize> = HashMap::new();
        costs.insert(
            State {
                p: start,
                d: Vector2Int::RIGHT,
            },
            0,
        );

        while let Some(Reverse((cost, state))) = heap.pop() {
            if state.p == end {
                return Box::new(cost);
            }

            for next_dir in Vector2Int::CARDINALS.iter().filter(|x| **x != (state.d * -1)).cloned() {
                let next_pos = state.p + next_dir;
                if !bwalls.is_set(next_pos.x as usize, next_pos.y as usize) {
                    let new_cost = cost + if next_dir != state.d { 1001 } else { 1 };
                    let new_state = State {
                        p: next_pos,
                        d: next_dir,
                    };
                    if !costs.contains_key(&new_state) || costs[&new_state] > new_cost {
                        heap.push(Reverse((new_cost, new_state)));
                        costs.insert(new_state, new_cost);
                    }
                }
            }
        }
        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut end = (0, 0);
        let mut start = (0, 0);

        let sz = (self.input.len() as f64).sqrt() as usize;
        let mut walls = Bitset2D::new(sz, sz);

        for (y, line) in self.input.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => {
                        walls.set(x, y);
                    }
                    b'E' => end = (x, y),
                    b'S' => start = (x, y),
                    _ => {}
                }
            }
        }

        let (_, seats) = solve2(start, end, &walls);

        return Box::new(seats.count_set());
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, PartialOrd)]
struct State {
    p: Vector2Int,
    d: Vector2Int,
}

// have to impl for binary heap, but the only sorting that matters is the usize in the tuple.0
impl Ord for State {
    fn cmp(&self, _: &Self) -> std::cmp::Ordering {
        return std::cmp::Ordering::Equal;
    }
}

type P2 = (usize, usize);
fn solve2(start: P2, end: P2, walls: &Bitset2D) -> (usize, Bitset2D) {
    let mut cost_map = HashMap::new();
    let mut mincost = usize::MAX;

    let mut pq = BinaryHeap::new();
    pq.push(Reverse((0, start, (1isize, 0isize))));

    for y in 0..walls.rows() {
        for x in 0..walls.cols() {
            for d in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                cost_map.insert(((x, y), d), usize::MAX);
            }
        }
    }

    // build cost map
    while let Some(Reverse((cost, pos, dir))) = pq.pop() {
        if cost > mincost {
            break;
        }
        if pos == end {
            if cost < mincost {
                mincost = cost;
            }
        }

        let (dx, dy) = dir;
        let (px, py) = (pos.0, pos.1);

        let moves = [(px.saturating_add_signed(dx), py.saturating_add_signed(dy)), pos, pos];
        let dirs = [dir, (dy, dx), (-dy, -dx)];
        let costs = [cost + 1, cost + 1000, cost + 1000];

        for i in 0..3 {
            let pos = moves[i];
            let dir = dirs[i];
            let cost = costs[i];
            if !walls.is_set(pos.0, pos.1) && cost_map[&(pos, dir)] > cost {
                cost_map.insert((pos, dir), cost);
                pq.push(Reverse((cost, pos, dir)));
            }
        }
    }

    // travel backward from end, watching costs, to make paths
    let mut seats = Bitset2D::new(walls.cols(), walls.rows());
    seats.set(end.0, end.1);

    let mut dq = VecDeque::new();
    for dir in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        dq.push_back((end, dir, mincost));
    }
    while let Some((pos, dir, cost)) = dq.pop_back() {
        let (dx, dy) = dir;
        let (px, py) = (pos.0, pos.1);

        let moves = [(px.saturating_add_signed(-dx), py.saturating_add_signed(-dy)), pos, pos];
        let dirs = [dir, (dy, dx), (-dy, -dx)];
        let costs = [cost - 1, cost - 1000, cost - 1000];

        for i in 0..3 {
            let pos = moves[i];
            let dir = dirs[i];
            let cost = costs[i];

            if !walls.is_set(pos.0, pos.1) && cost_map[&(pos, dir)] == cost {
                seats.set(pos.0, pos.1);
                dq.push_back((pos, dir, cost));
            }
        }
    }

    return (mincost, seats);
}

use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

type GateName = u16;

const GATE_AA: GateName = (b'A' as u16) << 8 | (b'A' as u16);

fn make_name(a: u8, b: u8) -> u16 {
    return ((b as u16) << 8) | a as u16;
}

pub struct Day20 {
    map: Grid2D<u8>,
    gates: HashMap<Vector2Int, Gate>,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 {
            map: Grid2D::new(0, 0),
            gates: HashMap::new(),
        };
    }

    /// Traverses map from each gate and records other gates and their distance
    /// in the Gate struct
    fn fill_gate_branches(&mut self) {
        let keys: Vec<Vector2Int> = self.gates.keys().cloned().collect();
        for start in &keys {
            let mut visited: HashSet<Vector2Int> = HashSet::new();
            let mut queue: VecDeque<(Vector2Int, usize)> = VecDeque::new();

            queue.push_back((*start, 0));
            while !queue.is_empty() {
                let (current, dist) = queue.pop_front().unwrap();

                visited.insert(current);

                for v in Vector2Int::CARDINALS {
                    let next = current + v;
                    if visited.contains(&next) {
                        continue;
                    }

                    if self.gates.contains_key(&next) && self.gates[&next].name != GATE_AA {
                        self.gates.get_mut(start).unwrap().branches.push((next, dist + 1));
                        continue;
                    }
                    if self.map[&next] == b'.' {
                        queue.push_back((next, dist + 1));
                    }
                }
            }
        }

        for k1 in &keys {
            for k2 in &keys {
                if k1 == k2 {
                    continue;
                }
                if self.gates[k1].name == self.gates[k2].name {
                    self.gates.get_mut(k1).unwrap().counterpart = self.gates[k2].location;
                    self.gates.get_mut(k2).unwrap().counterpart = self.gates[k1].location;
                }
            }
        }
    }

    /// Finds gates and locations in input map and assigns to self.gates
    fn find_gates(&mut self) {
        let find_gate = |x, y| -> (GateName, Vector2Int) {
            let mut name = 0;
            let mut v = Vector2Int::ZERO;
            if self.map[(x, y + 1)] == b'.' {
                name = make_name(self.map[(x, y - 1)], self.map[(x, y)]);
                v = Vector2Int::new_from_usize(x, y + 1);
            } else if self.map[(x, y - 1)] == b'.' {
                name = make_name(self.map[(x, y)], self.map[(x, y + 1)]);
                v = Vector2Int::new_from_usize(x, y - 1);
            } else if self.map[(x + 1, y)] == b'.' {
                name = make_name(self.map[(x - 1, y)], self.map[(x, y)]);
                v = Vector2Int::new_from_usize(x + 1, y);
            } else if self.map[(x - 1, y)] == b'.' {
                name = make_name(self.map[(x, y)], self.map[(x + 1, y)]);
                v = Vector2Int::new_from_usize(x - 1, y);
            }
            return (name, v);
        };

        let mpsize = self.map.size();

        // inside gates
        for y in 2..(mpsize.y - 2) {
            for x in 2..(mpsize.x - 2) {
                if self.map[(x, y)] >= b'A' {
                    let (name, loc) = find_gate(x, y);
                    if name != 0 {
                        self.gates.insert(loc, Gate::new(loc, false, name));
                    }
                }
            }
        }

        // left/right outside
        for y in 2..(mpsize.y - 2) {
            if self.map[(1, y)] >= b'A' {
                let (name, loc) = find_gate(1, y);
                if name != 0 {
                    self.gates.insert(loc, Gate::new(loc, true, name));
                }
            }
            if self.map[(mpsize.x - 1, y)] >= b'A' {
                let (name, loc) = find_gate(mpsize.x - 2, y);
                if name != 0 {
                    self.gates.insert(loc, Gate::new(loc, true, name));
                }
            }
        }

        // top/bottom outside
        for x in 2..(mpsize.x - 2) {
            if self.map[(x, 1)] >= b'A' {
                let (name, loc) = find_gate(x, 1);
                if name != 0 {
                    self.gates.insert(loc, Gate::new(loc, true, name));
                }
            }
            if self.map[(x, mpsize.y - 2)] >= b'A' {
                let (name, loc) = find_gate(x, mpsize.y - 2);
                if name != 0 {
                    self.gates.insert(loc, Gate::new(loc, true, name));
                }
            }
        }
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
        self.find_gates();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.fill_gate_branches();

        let entry = self.gates.iter().find(|x| x.1.name == GATE_AA).unwrap().1.clone();

        let mut visited: HashSet<Vector2Int> = HashSet::new();
        let mut pq: Vec<(Option<Gate>, usize)> = Vec::new();
        pq.push((Some(entry), 0));

        while !pq.is_empty() {
            pq.sort_by(|a, b| b.1.cmp(&a.1));

            let (current, dist) = pq.pop().unwrap();

            let current = match current {
                Some(c) => c,
                None => return Box::new(dist - 1),
            };

            visited.insert(current.location);

            for (gate_loc, cost) in current.branches {
                if visited.contains(&gate_loc) {
                    continue;
                }

                match self.gates.get(&gate_loc) {
                    Some(g) => {
                        let exit = self.gates.get(&g.counterpart).cloned();

                        pq.push((exit, dist + cost + 1));
                    }
                    None => return Box::new(dist - 1),
                }
            }
        }
        unreachable!();
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut pq = Vec::new();

        let entry = self.gates.iter().find(|x| x.1.name == GATE_AA).unwrap().1.clone();

        pq.push(P2Node {
            location: Some(entry),
            dist: 0,
            depth: 0,
        });

        while pq.len() > 0 {
            pq.sort_by(|a, b| {
                let mut cmp = b.depth.cmp(&a.depth);
                if cmp == Ordering::Equal {
                    cmp = b.dist.cmp(&a.dist);
                }
                return cmp;
            });

            let current = pq.pop().unwrap();
            if current.location.is_none() && current.depth == -1 {
                return Box::new(current.dist - 1);
            }

            for (gate_loc, cost) in current.location.unwrap().branches {
                let enter_gate = &self.gates[&gate_loc].clone();
                let exit_gate_loc = self.gates.get(&gate_loc).map_or(Vector2Int::ZERO, |x| x.counterpart);
                if exit_gate_loc == Vector2Int::ZERO && current.depth != 0 {
                    continue;
                }

                let exit_gate = self.gates.get(&exit_gate_loc).cloned();

                if exit_gate.is_none() {
                    if current.depth != 0 {
                        continue;
                    }
                    let n = P2Node {
                        location: None,
                        dist: current.dist + cost + 1,
                        depth: current.depth - 1,
                    };
                    pq.push(n);
                    continue;
                }

                if current.depth == 0 && enter_gate.is_exterior {
                    continue;
                }

                let next = P2Node {
                    depth: current.depth + (if enter_gate.is_exterior { -1 } else { 1 }),
                    location: exit_gate,
                    dist: current.dist + cost + 1,
                };
                pq.push(next);
            }
        }

        return Box::new("-");
    }
}

#[derive(Clone)]
struct Gate {
    location: Vector2Int,
    name: GateName,
    counterpart: Vector2Int,
    is_exterior: bool,
    branches: Vec<(Vector2Int, usize)>,
}
impl Gate {
    fn new(location: Vector2Int, is_exterior: bool, name: GateName) -> Self {
        return Self {
            location,
            name,
            counterpart: Vector2Int::ZERO,
            is_exterior,
            branches: Vec::new(),
        };
    }
}

struct P2Node {
    location: Option<Gate>,
    dist: usize,
    depth: isize,
}

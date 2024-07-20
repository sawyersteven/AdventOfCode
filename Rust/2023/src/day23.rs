use shared::{grid2d::Grid2D, v2i::Vector2Int};

use crate::Base;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

pub struct Day23 {
    map: Grid2D<u8>,
    start: Vector2Int,
    end: Vector2Int,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 {
            map: Grid2D::new(0, 0),
            start: Vector2Int::ZERO,
            end: Vector2Int::ZERO,
        };
    }

    fn find_next_intersection(
        &self,
        start: Vector2Int,
        direction: Vector2Int,
        intersections: &[Vector2Int],
    ) -> Option<(Vector2Int, usize)> {
        let sz = self.map.size().to_vector2() - Vector2Int::ONE;
        let mut current = start;
        let mut dir = direction;
        let mut steps = 0;
        loop {
            if !(current + dir).in_range(&Vector2Int::ZERO, &sz) {
                return None;
            }
            if self.map[current + dir] != b'#' {
                current += dir;
                steps += 1;
            } else {
                let left = Vector2Int::new(-dir.y, dir.x);
                if self.map[current + left] != b'#' {
                    dir = left;
                    current += dir;
                    steps += 1;
                } else {
                    dir = left * -1;
                    current += dir;
                    steps += 1;
                }
            }

            if intersections.contains(&current) {
                return Some((current, steps));
            }
        }
    }

    fn long_walk_2(
        &self,
        current: Vector2Int,
        steps: usize,
        visited: &mut VecDeque<Vector2Int>,
        intersections: &HashMap<Vector2Int, HashMap<Vector2Int, usize>>,
    ) -> usize {
        let mut longest = 0;
        visited.push_back(current);
        for (next, cost) in &intersections[&current] {
            if *next == current || visited.contains(&next) {
                continue;
            }
            if *next == self.end {
                longest = longest.max(steps + cost);
                continue;
            }
            longest = longest.max(self.long_walk_2(*next, steps + cost, visited, &intersections));
        }
        visited.pop_back();

        return longest;
    }

    // Simple brute force pathfinding
    fn long_walk(&self, can_climb_slopes: bool) -> usize {
        let mut longest = 0;

        let sz = self.map.size().to_vector2() - Vector2Int::ONE;
        let mut q = VecDeque::new();

        let start = Path {
            visited: HashSet::new(),
            pos: self.start,
        };
        q.push_back(start);

        while let Some(current) = q.pop_back() {
            if current.pos == self.end {
                longest = longest.max(current.visited.len());
                continue;
            }

            if !can_climb_slopes {
                let mut slope_move = Vector2Int::ZERO;
                match self.map[current.pos] {
                    b'^' => {
                        slope_move = current.pos + Vector2Int::DOWN;
                    }
                    b'>' => {
                        slope_move = current.pos + Vector2Int::RIGHT;
                    }
                    b'v' => {
                        slope_move = current.pos + Vector2Int::UP;
                    }
                    b'<' => {
                        slope_move = current.pos + Vector2Int::LEFT;
                    }
                    b'#' => {
                        continue;
                    }
                    _ => {}
                }

                if slope_move != Vector2Int::ZERO {
                    if current.visited.contains(&slope_move) {
                        continue;
                    }
                    let mut c = current.clone();
                    c.visited.insert(c.pos);
                    c.pos = slope_move;
                    q.push_back(c);
                    continue;
                }
            }

            for next in Vector2Int::CARDINALS
                .iter()
                .map(|x| current.pos + *x)
                .filter(|x| x.in_range(&Vector2Int::ZERO, &sz) && !current.visited.contains(&x))
            {
                if *self.map.get(next.x as usize, next.y as usize) == b'#' {
                    continue;
                }
                let mut c = current.clone();
                c.visited.insert(c.pos);
                c.pos = next;
                q.push_back(c);
            }
        }
        return longest;
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.map = Grid2D::<u8>::from_string(&raw_input);
        let sz = self.map.size();
        for x in 0..sz.x {
            if *self.map.get(x, 0) == b'.' {
                self.start.x = x as isize;
                break;
            }
        }
        self.end.y = sz.y as isize - 1;
        for x in 0..sz.x {
            if *self.map.get(x, sz.y - 1) == b'.' {
                self.end.x = x as isize;
                break;
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.long_walk(false));
    }

    // This takes 3+ seconds in release build. Should be faster.
    fn part2(&mut self) -> Box<dyn Display> {
        let mut intersections: HashMap<Vector2Int, HashMap<Vector2Int, usize>> = HashMap::new();

        let map_sz = self.map.size();
        for y in 1..(map_sz.y - 1) {
            for x in 1..(map_sz.x - 1) {
                if *self.map.get(x, y) == b'#' {
                    continue;
                }
                let mut branch_count = 0;
                branch_count += if *self.map.get(x + 1, y) != b'#' { 1 } else { 0 };
                branch_count += if *self.map.get(x - 1, y) != b'#' { 1 } else { 0 };
                branch_count += if *self.map.get(x, y - 1) != b'#' { 1 } else { 0 };
                branch_count += if *self.map.get(x, y + 1) != b'#' { 1 } else { 0 };

                if branch_count >= 3 {
                    intersections.insert(Vector2Int::new(x as isize, y as isize), HashMap::new());
                }
            }
        }

        intersections.insert(self.start, HashMap::new());
        intersections.insert(self.end, HashMap::new());

        let ks = intersections.keys().cloned().collect::<Vec<Vector2Int>>();

        for i in 0..ks.len() {
            let k = ks[i];
            let start = k;
            for n in Vector2Int::CARDINALS {
                if start.y + n.y < 0 || start.y + n.y > (map_sz.y - 1) as isize {
                    continue;
                }
                if *self.map.get((start.x + n.x) as usize, (start.y + n.y) as usize) != b'#' {
                    match self.find_next_intersection(start, n, &ks) {
                        Some((end, steps)) => {
                            intersections.get_mut(&k).unwrap().insert(end, steps);
                        }
                        None => {}
                    }
                }
            }
        }

        return Box::new(self.long_walk_2(self.start, 0, &mut VecDeque::new(), &intersections));
    }
}

#[derive(Clone)]
struct Path {
    visited: HashSet<Vector2Int>,
    pos: Vector2Int,
}

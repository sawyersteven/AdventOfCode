use itertools::Itertools;
use shared::v2i::Vector2Int;

use crate::Base;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

pub struct Day24 {
    pub input: Vec<Vec<u8>>,
    dist_cache: HashMap<(Vector2Int, Vector2Int), usize>,
    targets: Vec<Vector2Int>,
    start: Vector2Int,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 {
            input: Vec::new(),
            dist_cache: HashMap::new(),
            targets: Vec::new(),
            start: Vector2Int::ZERO,
        };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.input.push(line.bytes().collect());
        }

        self.start = Vector2Int::ZERO;
        let mut walls: HashSet<Vector2Int> = HashSet::new();
        for y in 0..self.input.len() {
            for x in 0..self.input[0].len() {
                let v = Vector2Int::new(x as isize, y as isize);
                match self.input[y][x] {
                    b'#' => _ = walls.insert(v),
                    b'0' => self.start = v,
                    n if n > b'0' && n <= b'9' => self.targets.push(v),
                    _ => {}
                };
            }
        }

        self.targets.push(self.start);
        self.dist_cache = calc_distances(&self.targets, &walls);
        self.targets.retain(|x| *x != self.start);
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut min = usize::MAX;
        for path in self.targets.iter().permutations(self.targets.len()) {
            let mut dist = 0;
            dist += self.dist_cache[&(self.start, *path[0])];
            for i in 0..(path.len() - 1) {
                let mut k = (*path[i], *path[i + 1]);
                if !self.dist_cache.contains_key(&k) {
                    k = (k.1, k.0);
                }
                dist += self.dist_cache[&k];
                if dist > min {
                    break;
                }
            }
            min = min.min(dist);
        }

        return Box::new(min);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut min = usize::MAX;
        for path in self.targets.iter().permutations(self.targets.len()) {
            let mut dist = 0;
            dist += self.dist_cache[&(self.start, *path[0])];

            for i in 0..(path.len() - 1) {
                let mut k = (*path[i], *path[i + 1]);
                if !self.dist_cache.contains_key(&k) {
                    k = (k.1, k.0);
                }
                dist += self.dist_cache[&k];
                if dist > min {
                    break;
                }
            }
            dist += self.dist_cache[&(self.start, **path.last().unwrap())];
            min = min.min(dist);
        }

        return Box::new(min);
    }
}

fn calc_distances(
    points: &Vec<Vector2Int>,
    walls: &HashSet<Vector2Int>,
) -> HashMap<(Vector2Int, Vector2Int), usize> {
    let mut paths = HashMap::new();
    for a in points {
        for b in points {
            if a == b {
                continue;
            }
            paths.insert((a.clone(), b.clone()), find_path(a, b, walls));
        }
    }
    return paths;
}

fn find_path(a: &Vector2Int, b: &Vector2Int, walls: &HashSet<Vector2Int>) -> usize {
    let mut q: VecDeque<(Vector2Int, usize)> = VecDeque::new();
    let mut visited: HashSet<Vector2Int> = HashSet::new();
    q.push_back((*a, 0));

    while q.len() != 0 {
        let (current, dist) = q.pop_front().unwrap();
        if current == *b {
            return dist;
        }
        for v in Vector2Int::CARDINALS {
            let step = current + v;
            if visited.contains(&step) || walls.contains(&step) {
                continue;
            }
            visited.insert(step);
            q.push_back((step, dist + 1));
        }
    }
    println!("{} -> {}", a, b);
    panic!();
}

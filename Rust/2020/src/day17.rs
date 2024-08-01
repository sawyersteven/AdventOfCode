use shared::{grid3d::Grid3D, v3i::Vector3Int};

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const ACTIVE: bool = true;
const CUBE_SZ: usize = 25;

pub struct Day17 {
    cube: Grid3D<bool>,
    hypercube: HyperCube,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 {
            cube: Grid3D::new(CUBE_SZ, CUBE_SZ, CUBE_SZ),
            hypercube: HyperCube::new(),
        };
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        let idx_offset = CUBE_SZ / 2;
        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().map(|x| *x == b'#').enumerate() {
                self.cube.set(x + idx_offset, y + idx_offset, idx_offset, b);
                // w, z, y, x

                self.hypercube.set(
                    &Vector4 {
                        x: (x + idx_offset) as isize,
                        y: (y + idx_offset) as isize,
                        z: (idx_offset) as isize,
                        w: (idx_offset) as isize,
                    },
                    b,
                );
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut neighbor_cache = HashMap::new();

        let mut changes = HashSet::new();
        for _ in 0..6 {
            for v3 in nodes_3d() {
                let count = match neighbor_cache.get(&v3) {
                    Some(nbrs) => nbrs,
                    None => {
                        let nbrs = neighbors_3d(&v3);
                        neighbor_cache.insert(v3, nbrs);
                        &neighbor_cache[&v3]
                    }
                }
                .iter()
                .filter(|n| self.cube[n] == ACTIVE)
                .count();

                match self.cube[&v3] {
                    true => {
                        if count > 3 || count < 2 {
                            changes.insert(v3);
                        }
                    }
                    false => {
                        if count == 3 {
                            changes.insert(v3);
                        }
                    }
                }
            }

            for v3 in changes.drain() {
                self.cube[&v3] ^= true;
            }
        }

        return Box::new(self.cube.count(|x| *x));
    }

    /*
    This takes some time. A few thoughts for optimizing (that I won't bother to
    implement)...

    The active portion of the cube can only grow at a certain rate (1 unit in
    any direction per turn). So rather than checking every coord in each turn,
    only the original size + turn * 2 needs to be checked
     */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut neighbor_cache = HashMap::new();

        let mut changes = HashSet::new();
        for _ in 0..6 {
            for v4 in nodes_4d() {
                let count = match neighbor_cache.get(&v4) {
                    Some(nbrs) => nbrs,
                    None => {
                        let nbrs = neighbors_4d(&v4);
                        neighbor_cache.insert(v4, nbrs);
                        &neighbor_cache[&v4]
                    }
                }
                .iter()
                .filter(|n| self.hypercube.get(*n) == ACTIVE)
                .count();

                match self.hypercube.get(&v4) {
                    true => {
                        if count > 3 || count < 2 {
                            changes.insert(v4);
                        }
                    }
                    false => {
                        if count == 3 {
                            changes.insert(v4);
                        }
                    }
                }
            }

            for v4 in changes.drain() {
                let b = self.hypercube.get(&v4);
                self.hypercube.set(&v4, !b);
            }
        }

        let mut answer = 0;
        for n in nodes_4d() {
            if self.hypercube.get(&n) {
                answer += 1;
            }
        }
        return Box::new(answer);
    }
}

fn neighbors_3d(origin: &Vector3Int) -> Vec<Vector3Int> {
    let mut n = Vec::new();
    for x in (origin.x - 1)..=(origin.x + 1) {
        if x < 0 || x >= CUBE_SZ as isize {
            continue;
        }
        for y in (origin.y - 1)..=(origin.y + 1) {
            if y < 0 || y >= CUBE_SZ as isize {
                continue;
            }
            for z in (origin.z - 1)..=(origin.z + 1) {
                if z < 0 || z >= CUBE_SZ as isize {
                    continue;
                }
                let v = Vector3Int::new(x, y, z);
                if v == *origin {
                    continue;
                }
                n.push(v);
            }
        }
    }
    return n;
}

fn nodes_3d() -> impl Iterator<Item = Vector3Int> {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;

    std::iter::from_fn(move || loop {
        let v = Vector3Int::new(x, y, z);

        x += 1;
        if x == CUBE_SZ as isize {
            x = 0;
            y += 1;
            if y == CUBE_SZ as isize {
                y = 0;
                z += 1;
                if z == CUBE_SZ as isize {
                    return None;
                }
            }
        }

        return Some(v);
    })
}

struct HyperCube {
    inner: [bool; CUBE_SZ * CUBE_SZ * CUBE_SZ * CUBE_SZ],
}

impl HyperCube {
    fn new() -> Self {
        return HyperCube {
            inner: [false; CUBE_SZ * CUBE_SZ * CUBE_SZ * CUBE_SZ],
        };
    }
    fn set(&mut self, idx: &Vector4, val: bool) {
        self.inner[self.v4_to_ind(idx)] = val;
    }

    fn get(&self, idx: &Vector4) -> bool {
        return self.inner[self.v4_to_ind(idx)];
    }

    fn v4_to_ind(&self, ind: &Vector4) -> usize {
        let cs = CUBE_SZ as isize;
        return (ind.x + (ind.y * cs) + (ind.z * cs * cs) + (ind.w * cs * cs * cs)) as usize;
    }
}

fn neighbors_4d(origin: &Vector4) -> Vec<Vector4> {
    let mut n = Vec::new();
    for x in (origin.x - 1)..=(origin.x + 1) {
        if x < 0 || x >= CUBE_SZ as isize {
            continue;
        }
        for y in (origin.y - 1)..=(origin.y + 1) {
            if y < 0 || y >= CUBE_SZ as isize {
                continue;
            }
            for z in (origin.z - 1)..=(origin.z + 1) {
                if z < 0 || z >= CUBE_SZ as isize {
                    continue;
                }
                for w in (origin.w - 1)..=(origin.w + 1) {
                    if w < 0 || w >= CUBE_SZ as isize {
                        continue;
                    }
                    let v = Vector4 { x, y, z, w };
                    if v == *origin {
                        continue;
                    }
                    n.push(v);
                }
            }
        }
    }
    return n;
}

fn nodes_4d() -> impl Iterator<Item = Vector4> {
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut w = 0;

    std::iter::from_fn(move || loop {
        let v = Vector4 { x, y, z, w };

        x += 1;
        if x == CUBE_SZ as isize {
            x = 0;
            y += 1;
            if y == CUBE_SZ as isize {
                y = 0;
                z += 1;
                if z == CUBE_SZ as isize {
                    z = 0;
                    w += 1;
                    if w == CUBE_SZ as isize {
                        return None;
                    }
                }
            }
        }

        return Some(v);
    })
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Vector4 {
    x: isize,
    y: isize,
    z: isize,
    w: isize,
}

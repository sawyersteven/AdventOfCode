use shared::v3i::Vector3Int;

use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub struct Day18 {
    lava_cells: Vec<Vector3Int>,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { lava_cells: Vec::new() };
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let nums: Vec<isize> = line.split(',').map(|x| x.trim().parse().unwrap()).collect();
            self.lava_cells.push(Vector3Int::new(nums[0], nums[1], nums[2]));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;

        for cell in &self.lava_cells {
            for f in FACES {
                if !self.lava_cells.contains(&(f + *cell)) {
                    count += 1;
                }
            }
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let lava_cells: HashSet<&Vector3Int> = HashSet::from_iter(self.lava_cells.iter());
        let mut water_cells = HashSet::new();

        let mut area_min = Vector3Int::MAX;
        let mut area_max = Vector3Int::ZERO;

        for l in &lava_cells {
            area_min = area_min.min(l);
            area_max = area_max.max(l);
        }
        area_min -= Vector3Int::ONE;
        area_max += Vector3Int::ONE;

        let mut visited = HashSet::new();
        let mut q = VecDeque::new();

        q.push_back(Vector3Int::ZERO);

        while let Some(current) = q.pop_back() {
            visited.insert(current);
            for f in FACES {
                let neighbor = current + f;

                if visited.contains(&neighbor)
                    || water_cells.contains(&neighbor)
                    || lava_cells.contains(&neighbor)
                    || !neighbor.in_range(&area_min, &area_max)
                {
                    continue;
                }

                water_cells.insert(neighbor);
                q.push_back(neighbor);
            }
        }

        let mut count = 0;
        for water_cell in water_cells {
            for f in FACES {
                if lava_cells.contains(&(water_cell + f)) {
                    count += 1;
                }
            }
        }

        return Box::new(count);
    }
}

const FACES: [Vector3Int; 6] = [
    Vector3Int::new(-1, 0, 0),
    Vector3Int::new(1, 0, 0),
    Vector3Int::new(0, -1, 0),
    Vector3Int::new(0, 1, 0),
    Vector3Int::new(0, 0, -1),
    Vector3Int::new(0, 0, 1),
];

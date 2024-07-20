use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day22 {
    blocks: Vec<Block>,
    blocks_above: HashMap<u16, Vec<u16>>,
    blocks_below: HashMap<u16, Vec<u16>>,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 {
            blocks: Vec::new(),
            blocks_above: HashMap::new(),
            blocks_below: HashMap::new(),
        };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        let mut id = 1;
        for line in raw_input.lines() {
            let mut nums: Vec<u16> = Vec::with_capacity(6);
            for half in line.split('~') {
                for digit in half.split(',') {
                    nums.push(digit.parse().unwrap());
                }
            }
            self.blocks.push(Block {
                min: P3 {
                    x: nums[0],
                    y: nums[1],
                    z: nums[2],
                },
                max: P3 {
                    x: nums[3],
                    y: nums[4],
                    z: nums[5],
                },
                id: id,
            });
            id += 1;
        }

        for b in &self.blocks {
            self.blocks_above.insert(b.id, Vec::new());
            self.blocks_below.insert(b.id, Vec::new());
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        drop_blocks(&mut self.blocks, false);
        self.blocks.reverse(); // now sorted by min.z descending

        for i in 0..self.blocks.len() {
            let c = &self.blocks[i];
            let z = c.min.z - 1;

            for j in (i + 1)..self.blocks.len() {
                if self.blocks[i].overlaps_vertical(&self.blocks[j]) && self.blocks[j].max.z == z {
                    self.blocks_above.get_mut(&self.blocks[j].id).unwrap().push(c.id);
                    self.blocks_below.get_mut(&c.id).unwrap().push(self.blocks[j].id);
                }
            }
        }

        let mut can_remove = HashSet::new();
        for (id, _blocks_above) in &self.blocks_above {
            if _blocks_above.is_empty() {
                can_remove.insert(id);
                continue;
            }

            if _blocks_above
                .iter()
                .all(|top_block| self.blocks_below[&top_block].len() > 1)
            {
                can_remove.insert(id);
            }
        }
        return Box::new(can_remove.len());
    }

    /* Must run part 1 first to generate above/below maps
    I'm sure there is a faster way than this brute force, but it works
    */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        self.blocks.reverse();
        for start_id in self.blocks.iter().map(|x| x.id) {
            let mut blocks = self.blocks.iter().filter(|b| b.id != start_id).cloned().collect();
            total += drop_blocks(&mut blocks, true);
        }
        return Box::new(total);
    }
}

#[derive(PartialEq, Clone)]
struct P3 {
    x: u16,
    y: u16,
    z: u16,
}

#[derive(PartialEq, Clone)]
pub struct Block {
    min: P3,
    max: P3,
    id: u16,
}

impl Block {
    // if cubes overlap on x,y as if looking straight down at them
    fn overlaps_vertical(&self, other: &Block) -> bool {
        if self.min.x > other.max.x
            || other.min.x > self.max.x
            || self.min.y > other.max.y
            || other.min.y > self.max.y
        {
            return false;
        }
        return true;
    }
}

// Make blocks fall until settled. Blocks will be sorted low-to-high z
fn drop_blocks(blocks: &mut Vec<Block>, sorted: bool) -> usize {
    let mut count = 0;
    if !sorted {
        blocks.sort_by(|a, b| a.min.z.cmp(&b.min.z));
    }
    for i in 0..blocks.len() {
        let mut max_z = 0;
        for j in 0..i {
            if blocks[i].overlaps_vertical(&blocks[j]) {
                max_z = max_z.max(blocks[j].max.z + 1);
            }
        }
        let drop_by = blocks[i].min.z - max_z;
        if drop_by > 0 {
            blocks[i].min.z -= drop_by;
            blocks[i].max.z -= drop_by;
            count += 1;
        }
    }
    return count;
}

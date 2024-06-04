use crate::Base;
use std::fmt::Display;

const OPEN: u8 = b'.';
const TREE: u8 = b'|';
const LUMBER: u8 = b'#';
const MINUTES: usize = 10;

pub struct Day18 {
    forest: Vec<Vec<u8>>,
}

impl Day18 {
    pub fn new() -> Day18 {
        return Day18 { forest: Vec::new() };
    }
}

impl Base for Day18 {
    fn parse_input(&mut self, raw_input: String) {
        let w = raw_input.split('\n').nth(0).unwrap().len() + 2;
        // build forest grid with border of OPEN cells
        self.forest.push(vec![OPEN; w]);
        for line in raw_input.split('\n') {
            let mut row: Vec<u8> = line.bytes().collect();
            row.insert(0, OPEN);
            row.push(OPEN);
            self.forest.push(row);
        }
        self.forest.push(vec![OPEN; w]);
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut forest = self.forest.clone();
        for _ in 0..MINUTES {
            forest = self.run_iteration(&forest);
        }
        return Box::new(self.resource_value(&forest));
    }

    // Runs iters until a stable pattern emerges
    // then determines 1 millionth based on offset
    // The search offset and sample len are mostly
    // arbitrary and need to be only long enough
    // to find the pattern
    fn part2(&mut self) -> Box<dyn Display> {
        const MINUTES: usize = 1_000_000_000;
        const SEARCH_OFFSET: usize = 1000;
        const SAMPLE_LEN: usize = 75;

        let mut forest = self.forest.clone();

        // go long enough to find a stable pattern
        for _ in 0..SEARCH_OFFSET {
            forest = self.run_iteration(&forest);
        }

        let pattern_start = self.resource_value(&forest);

        let mut sample_data = [0; SAMPLE_LEN];
        for i in 0..SAMPLE_LEN {
            forest = self.run_iteration(&forest);
            sample_data[i] = self.resource_value(&forest);
        }

        let pattern_len = sample_data.iter().position(|x| *x == pattern_start).unwrap() + 1;
        if pattern_len == 0
            || *sample_data
                .iter()
                .rfind(|x| **x == pattern_start)
                .unwrap_or(&(pattern_len - 1))
                == pattern_len - 1
        {
            return Box::new("Pattern length not found -- increase sample size or sample offset");
        }

        let ans = sample_data[(MINUTES - SEARCH_OFFSET - 1) % pattern_len];

        return Box::new(ans);
    }
}

impl Day18 {
    fn run_iteration(&mut self, forest: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
        let mut next = forest.clone();
        for y in 1..(forest.len() - 1) {
            for x in 1..(forest[0].len() - 1) {
                let surrounding: [u8; 8] = [
                    forest[y - 1][x - 1],
                    forest[y - 1][x],
                    forest[y - 1][x + 1],
                    forest[y][x - 1],
                    forest[y][x + 1],
                    forest[y + 1][x - 1],
                    forest[y + 1][x],
                    forest[y + 1][x + 1],
                ];

                if forest[y][x] == OPEN {
                    let mut t = 0;
                    for c in surrounding {
                        if c == TREE {
                            t += 1;
                        }
                    }
                    next[y][x] = if t >= 3 { TREE } else { OPEN };
                } else if forest[y][x] == TREE {
                    let mut l = 0;
                    for c in surrounding {
                        if c == LUMBER {
                            l += 1;
                        }
                    }
                    next[y][x] = if l >= 3 { LUMBER } else { TREE };
                } else if forest[y][x] == LUMBER {
                    let mut l = 0;
                    let mut t = 0;
                    for c in surrounding {
                        if c == LUMBER {
                            l += 1;
                        } else if c == TREE {
                            t += 1;
                        }
                    }

                    next[y][x] = if l > 0 && t > 0 { LUMBER } else { OPEN };
                }
            }
        }
        return next;
    }

    fn resource_value(&self, forest: &Vec<Vec<u8>>) -> usize {
        let mut woods = 0;
        let mut yards = 0;
        for y in 1..(forest.len() - 1) {
            for x in 1..(forest[0].len() - 1) {
                match forest[y][x] {
                    TREE => woods += 1,
                    LUMBER => yards += 1,
                    _ => {}
                }
            }
        }
        return woods * yards;
    }
}

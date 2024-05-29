use crate::Base;
use std::{collections::HashSet, fmt::Display};

pub struct Day03 {
    claims: Vec<Claim>,
    collisions: HashSet<(isize, isize)>,
}

impl Day03 {
    pub fn new() -> Day03 {
        return Day03 {
            claims: Vec::new(),
            collisions: HashSet::new(),
        };
    }
}

impl Base for Day03 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        for i in 0..lines.len() {
            let line = lines[i];
            let parts: Vec<&str> = line.split(' ').collect();
            let xy: Vec<isize> = parts[2][0..(parts[2].len() - 1)]
                .split(',')
                .map(|x| x.parse().unwrap())
                .collect();

            let wh: Vec<isize> = parts[parts.len() - 1].split('x').map(|x| x.parse().unwrap()).collect();

            let c = Claim {
                id: parts[0][1..].parse().unwrap(),
                x: xy[0],
                y: xy[1],
                x2: xy[0] + wh[0],
                y2: xy[1] + wh[1],
            };
            self.claims.push(c);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sqins = HashSet::<(isize, isize)>::new();
        let mut sqin = (0, 0);
        for claim in &self.claims {
            for row in claim.y..claim.y2 {
                sqin.0 = row;
                for col in claim.x..claim.x2 {
                    sqin.1 = col;
                    if sqins.contains(&sqin) {
                        self.collisions.insert(sqin);
                    } else {
                        sqins.insert(sqin);
                    }
                }
            }
        }
        return Box::new(self.collisions.len());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut sqin = (0, 0);
        for c in &self.claims {
            let mut collision = false;
            for row in c.y..c.y2 {
                sqin.0 = row;
                for col in c.x..c.x2 {
                    sqin.1 = col;
                    if self.collisions.contains(&sqin) {
                        collision = true;
                        break;
                    }
                }
                if collision {
                    break;
                }
            }
            if !collision {
                return Box::new(c.id);
            }
        }
        return Box::new("-");
    }
}

struct Claim {
    id: isize,
    x: isize,
    y: isize,
    x2: isize,
    y2: isize,
}

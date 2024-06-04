use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashMap, fmt::Display, hash::Hash};

const WET: u8 = b'=';
const ROCKY: u8 = b'.';
const NARROW: u8 = b'|';

const GRID_PAD: usize = 75;

const TYPES: [u8; 3] = [ROCKY, WET, NARROW];

#[derive(Clone, Copy, PartialEq)]
enum Tool {
    None,
    Torch,
    Gear,
}

pub struct Day22 {
    depth: usize,
    target: Vector2Int,
    map: Vec<Vec<u8>>,
    input: String,
    compatible_tools: HashMap<u8, (Tool, Tool)>,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 {
            depth: 0,
            map: Vec::new(),
            target: Vector2Int::ZERO,
            input: String::from(""),
            compatible_tools: HashMap::from([
                (ROCKY, (Tool::Gear, Tool::Torch)),
                (NARROW, (Tool::None, Tool::Torch)),
                (WET, (Tool::None, Tool::Gear)),
            ]),
        };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        self.depth = raw_input
            .split('\n')
            .nth(0)
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let t: Vec<isize> = raw_input
            .split('\n')
            .nth(1)
            .unwrap()
            .split(' ')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        self.target = Vector2Int::new(t[0], t[1]);

        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.map = self.make_terrain_map(&self.make_erosion_map());

        let mut risk = 0;
        for y in 0..(self.target.y + 1) {
            for x in 0..(self.target.x + 1) {
                match self.map[y as usize][x as usize] {
                    WET => risk += 1,
                    NARROW => risk += 2,
                    _ => {}
                }
            }
        }
        return Box::new(risk);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let max_y = self.map.len() - 1;
        let max_x = self.map[0].len() - 1;

        let mut open = HashMap::new();
        let mut closed = HashMap::new();

        let start = Location {
            tool: Tool::Torch,
            point: Vector2Int::ZERO,
        };

        let target = Location {
            tool: Tool::Torch,
            point: self.target.clone(),
        };

        closed.insert(start, 0);
        open.insert(start, 0);

        while open.len() > 0 {
            let (coord, time) = min_score(&open);
            open.remove(&coord);

            if time > closed[&coord] {
                continue;
            }

            if coord == target {
                return Box::new(closed[&coord]);
            }

            // create location for this point with other tool
            let compat_tools = self.compatible_tools[&self.map[coord.point.y as usize][coord.point.x as usize]];
            let other_tool: Tool;

            other_tool = if compat_tools.0 == coord.tool {
                compat_tools.1
            } else {
                compat_tools.0
            };

            let alt_loc = Location {
                point: coord.point,
                tool: other_tool,
            };
            let alt_score = closed[&coord] + 7;

            if !closed.contains_key(&alt_loc) || closed[&alt_loc] > alt_score {
                closed.insert(alt_loc, alt_score);
                open.insert(alt_loc, alt_score);
            }

            for other in self.adjacent_moves(&coord.point, max_x as isize, max_y as isize) {
                let other_compat_tools = self.compatible_tools[&self.map[other.y as usize][other.x as usize]];
                if other_compat_tools.0 != coord.tool && other_compat_tools.1 != coord.tool {
                    continue;
                }

                let next_move = Location {
                    point: other,
                    tool: coord.tool,
                };

                let score = closed[&coord] + 1;
                if !closed.contains_key(&next_move) || closed[&next_move] > score {
                    closed.insert(next_move, score);
                    open.insert(next_move, score);
                }
            }
        }

        return Box::new("?");
    }
}

impl Day22 {
    fn adjacent_moves(&self, point: &Vector2Int, max_x: isize, max_y: isize) -> Vec<Vector2Int> {
        let mut adjacents = Vec::new();
        if point.y < max_y {
            adjacents.push(Vector2Int::new(point.x, point.y + 1));
        }
        if point.y > 0 {
            adjacents.push(Vector2Int::new(point.x, point.y - 1));
        }
        if point.x < max_x {
            adjacents.push(Vector2Int::new(point.x + 1, point.y));
        }
        if point.x > 0 {
            adjacents.push(Vector2Int::new(point.x - 1, point.y));
        }
        return adjacents;
    }

    fn make_erosion_map(&self) -> Vec<Vec<usize>> {
        const EROSION_MOD: usize = 20183;
        let h = self.target.y as usize + GRID_PAD;
        let w = self.target.x as usize + GRID_PAD;
        let mut map = vec![vec![0; w]; h];

        map[0][0] = self.depth % 20183;

        for y in 1..h {
            map[y][0] = ((y * 48271) + self.depth) % EROSION_MOD;
        }

        for x in 1..w {
            map[0][x] = ((x * 16807) + self.depth) % EROSION_MOD;
        }

        for y in 1..h {
            for x in 1..w {
                if x as isize == self.target.x && y as isize == self.target.y {
                    map[y][x] = map[0][0];
                    continue;
                }

                let k = ((map[y][x - 1] * map[y - 1][x]) + self.depth) % EROSION_MOD;
                map[y][x] = k;
            }
        }
        return map;
    }

    fn make_terrain_map(&self, geo_map: &Vec<Vec<usize>>) -> Vec<Vec<u8>> {
        let h = geo_map.len();
        let w = geo_map[0].len();
        let mut map = vec![vec![b' '; w]; h];

        for y in 0..h {
            for x in 0..w {
                map[y][x] = TYPES[(geo_map[y][x] % 3) as usize];
            }
        }
        return map;
    }
}

fn min_score(dict: &HashMap<Location, usize>) -> (Location, usize) {
    let mut low = (
        Location {
            tool: Tool::None,
            point: Vector2Int::ZERO,
        },
        usize::MAX,
    );

    for (loc, time) in dict {
        if *time < low.1 {
            low.0 = loc.clone();
            low.1 = *time;
        }
    }
    return low;
}

#[derive(Clone, Copy)]
struct Location {
    tool: Tool,
    point: Vector2Int,
}

impl Eq for Location {}

impl PartialEq for Location {
    fn eq(&self, other: &Self) -> bool {
        let h = (self.point.y * 10000) + (self.point.x * 10) + self.tool as isize;
        let other_h = (other.point.y * 10000) + (other.point.x * 10) + other.tool as isize;
        return h == other_h;
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.point, (self.tool as usize))
    }
}

impl Hash for Location {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let h = (self.point.y * 10000) + (self.point.x * 10) + self.tool as isize;
        h.hash(state);
    }
}

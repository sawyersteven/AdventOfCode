use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

type BitMask = u64;

struct PathNode {
    valves_state: BitMask, // 1 OPEN, 0 CLOSE
    loc_ind: usize,
    time_left: isize,
    pressure: isize,
}

struct Valve {
    dist_ind: usize,
    flow_rate: isize,
    tunnel_inds: Vec<usize>,
    uid: BitMask,
}

pub struct Day16 {
    valves: Vec<Valve>,
    dist_map: Vec<Vec<isize>>,
    aa_ind: usize,
}

/*
This one was a bit tricky. One of the more difficult path-finding puzzles so
far out of all the years.

To explain I left comments throughout, but a few general concepts are important:
    * There are 61 valves. All of them matter for the distance map, but only
    the valves that have a positive flow rate matter for pathfinding.

    * 61 valves means a u64 can be used as a bitmask to store the state of each
    valve as open (1) or closed (0).

    * Each valve is thus given a dist_ind (simply the line number it
    is parsed from) that corresponds to its index in the dist_map. It is also
    given a uid, which is 0b1 << dist_ind to use as a flag for the state bitmask


*/
impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            valves: Vec::new(),
            dist_map: Vec::new(),
            aa_ind: 0,
        };
    }

    // Generates distance map using Floyd Warshall algo as nested vecs
    // map[valve_a_index][valve_b_index] = distance
    fn make_dist_map(&mut self) {
        let valve_count = self.valves.len();
        let mut dist_map = vec![vec![isize::MAX / 4; valve_count]; valve_count];

        for k in 0..valve_count {
            for i in 0..valve_count {
                if self.valves[k].tunnel_inds.contains(&i) {
                    dist_map[k][i] = 1;
                }
            }
            dist_map[k][k] = 0;
        }

        for k in 0..valve_count {
            for i in 0..valve_count {
                for j in 0..valve_count {
                    let dist = dist_map[i][j].min(dist_map[i][k] + dist_map[k][j]);
                    dist_map[i][j] = dist;
                }
            }
        }

        self.dist_map = dist_map;
    }

    // Returns bitmask of valve state and total pressure released
    fn make_paths(&self, start: PathNode) -> HashMap<BitMask, isize> {
        let mut paths = HashMap::new();

        let mut q = VecDeque::new();
        q.push_back(start);

        while let Some(current) = q.pop_back() {
            // Iter only through valves that are closed and have flow > 0
            let mut eol = true;
            for next in self
                .valves
                .iter()
                .filter(|x| x.flow_rate > 0 && current.valves_state & x.uid == 0)
            {
                eol = false;
                let dist = self.dist_map[current.loc_ind][next.dist_ind];
                if dist > current.time_left {
                    let best = *paths.get(&current.valves_state).unwrap_or(&0);

                    if current.pressure > best {
                        paths.insert(current.valves_state, current.pressure);
                    }
                    continue;
                }

                q.push_back(PathNode {
                    valves_state: current.valves_state | next.uid,
                    loc_ind: next.dist_ind,
                    time_left: current.time_left - dist - 1,
                    pressure: current.pressure + next.flow_rate * (current.time_left - dist - 1),
                });
            }

            // if all paths exhausted but time remains. This only applies to
            // the test input in my case, but might for other users' inputs.
            if eol {
                let best = *paths.get(&current.valves_state).unwrap_or(&0);

                if current.pressure > best {
                    paths.insert(current.valves_state, current.pressure);
                }
            }
        }
        return paths;
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        let mut valve_indices = HashMap::new();
        for (i, line) in raw_input.lines().enumerate() {
            valve_indices.insert(&line[6..8], i);
        }

        for (i, line) in raw_input.lines().enumerate() {
            let parts: Vec<&str> = line.split(' ').collect();
            let name = parts[1].trim();
            let rate_s = parts[4].split("rate=").nth(1).unwrap();

            let tunnels = if line.contains("valves ") {
                line.split("valves ")
            } else {
                line.split("valve ")
            }
            .nth(1)
            .unwrap()
            .split(", ")
            .map(|x| valve_indices[&x.trim()])
            .collect();

            self.valves.push(Valve {
                dist_ind: i,
                flow_rate: rate_s[0..(rate_s.len() - 1)].parse().unwrap(),
                tunnel_inds: tunnels,
                uid: 0b1 << i,
            });

            if name == "AA" {
                self.aa_ind = i;
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        self.make_dist_map();

        let mut valve_state = 0;
        let _ = self
            .valves
            .iter()
            .filter(|x| x.flow_rate == 0)
            .map(|x| valve_state |= x.uid);

        let start = PathNode {
            valves_state: valve_state,
            loc_ind: self.aa_ind,
            time_left: 30,
            pressure: 0,
        };

        let paths = self.make_paths(start);
        return Box::new(*paths.values().max().unwrap());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut valve_state = 0;
        let _ = self
            .valves
            .iter()
            .filter(|x| x.flow_rate == 0)
            .map(|x| valve_state |= x.uid);

        let start = PathNode {
            valves_state: valve_state,
            loc_ind: self.aa_ind,
            time_left: 26,
            pressure: 0,
        };
        let paths = self.make_paths(start);

        let mut best = 0;

        let keys: Vec<BitMask> = paths.keys().map(|x| *x).collect();

        for h in 0..(paths.len() - 1) {
            let hm = keys[h];
            for e in h..paths.len() {
                let em = keys[e];
                if hm & em == 0 {
                    let t = paths[&hm] + paths[&keys[e]];
                    best = best.max(t);
                }
            }
        }

        return Box::new(best);
    }
}

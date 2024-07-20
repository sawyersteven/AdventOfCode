use crate::Base;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Display,
};

pub struct Day25 {
    wires: HashMap<u16, HashSet<u16>>,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 { wires: HashMap::new() };
    }

    // Checks if number of unique paths from start to end is under max
    fn count_paths(&self, start: u16, end: u16, max: u16) -> bool {
        let mut connections = 0;
        let mut used_nodes = HashSet::new();
        used_nodes.insert(start);

        for adjacent_node in &self.wires[&start] {
            if *adjacent_node == end {
                connections += 1;
                continue;
            }

            let mut visited = HashSet::new();
            let mut q = VecDeque::new();
            q.push_back((*adjacent_node, HashSet::from([*adjacent_node])));

            'outer: while q.len() > 0 {
                let (current, path) = q.pop_front().unwrap();
                for c in &self.wires[&current] {
                    if end == *c {
                        connections += 1;
                        if connections > max {
                            return false;
                        }
                        used_nodes.extend(path);
                        break 'outer;
                    } else if visited.contains(&c) || path.contains(&c) || used_nodes.contains(c) {
                        continue;
                    }

                    let mut p = path.clone();
                    p.insert(*c);
                    q.push_back((*c, p));
                    visited.insert(c);
                }
            }
        }
        return connections <= max;
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let parts: Vec<&str> = line.split(": ").collect();
            let out_ids: Vec<u16> = parts[1].split(' ').map(|x| str_to_id(x.trim())).collect();

            let in_id = str_to_id(parts[0]);

            if !self.wires.contains_key(&in_id) {
                self.wires.insert(in_id, HashSet::new());
            }
            self.wires.get_mut(&in_id).unwrap().extend(out_ids.clone());

            for o in &out_ids {
                if !self.wires.contains_key(o) {
                    self.wires.insert(o.clone(), HashSet::new());
                }
                self.wires.get_mut(o).unwrap().insert(in_id);
            }
        }
    }

    /*
    Split graphs into two halves based on how many unique paths are available
    from a randomly chosen 'seed' node:

    ##  v
    ###  ###
    #########
    #########
    #########
    ####   ##
    ##  ^

    Any path that crosses over the edges indicated above can have at most 3
    completely unique paths to the other side.

     */
    fn part1(&mut self) -> Box<dyn Display> {
        // The best seed is the most connected node
        let mut seed = *self.wires.keys().nth(0).unwrap();
        for (k, v) in &self.wires {
            if v.len() > self.wires[&seed].len() {
                seed = *k;
            }
        }

        let mut g1 = 1; // group 1 already contains the seed
        let mut g2 = 0;
        for end in self.wires.keys() {
            if *end == seed {
                continue;
            }

            if !self.count_paths(seed, *end, 3) {
                g1 += 1;
            } else {
                g2 += 1;
            }
        }

        return Box::new(g1 * g2); // 567606
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

/*
b'z' - b'a' = 0b00011001, so only the first 5 bits matter
*/
fn str_to_id(string: &str) -> u16 {
    let mut id = 0u16;
    for b in string.bytes() {
        id <<= 5;
        id |= (b - b'a') as u16;
    }
    return id;
}

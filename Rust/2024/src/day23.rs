use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    usize,
};

pub struct Day23 {
    input: String,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { input: String::new() };
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut network_map = HashMap::new();
        for line in self.input.lines() {
            let k = [line.as_bytes()[0], line.as_bytes()[1]];
            let v = [line.as_bytes()[3], line.as_bytes()[4]];

            network_map.entry(k).or_insert(HashSet::new()).insert(v);
            network_map.entry(v).or_insert(HashSet::new()).insert(k);
        }

        let mut res = Vec::new();
        let mut p = network_map.keys().cloned().collect();

        find_cliques_of_length(HashSet::new(), &mut p, &network_map, 3, &mut res);
        return Box::new(res.iter().filter(|sbg| sbg.iter().any(|pc| pc[0] == b't')).count());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut network_map = HashMap::new();
        for line in self.input.lines() {
            let k = [line.as_bytes()[0], line.as_bytes()[1]];
            let v = [line.as_bytes()[3], line.as_bytes()[4]];
            network_map.entry(k).or_insert(HashSet::new()).insert(v);
            network_map.entry(v).or_insert(HashSet::new()).insert(k);
        }

        let p = network_map.keys().cloned().collect();
        let largest = find_largest_clique(p, &network_map);
        let mut c = Vec::from_iter(largest.iter().cloned());

        let sorter = |a: &[u8; 2], b: &[u8; 2]| -> std::cmp::Ordering {
            return match a[0].cmp(&b[0]) {
                std::cmp::Ordering::Equal => a[1].cmp(&b[1]),
                n => n,
            };
        };

        c.sort_by(sorter);
        let mut ans = String::new();
        for name in &c {
            ans.push(name[0] as char);
            ans.push(name[1] as char);
            ans.push(',');
        }
        ans.pop();

        return Box::new(ans);
    }
}

// Streamlined Bron–Kerbosch to find all subgraphs of len K
fn find_cliques_of_length(
    r: HashSet<[u8; 2]>,
    p: &mut HashSet<[u8; 2]>,
    graph: &HashMap<[u8; 2], HashSet<[u8; 2]>>,
    k: usize,
    cliques: &mut Vec<HashSet<[u8; 2]>>,
) {
    for v in p.clone() {
        let mut r_next = r.clone();
        r_next.insert(v);

        if r_next.len() == k {
            cliques.push(r_next);
        } else {
            let mut p_next: HashSet<[u8; 2]> = match graph.get(&v) {
                Some(n) => p.intersection(n).copied().collect(),
                None => continue,
            };
            find_cliques_of_length(r_next, &mut p_next, graph, k, cliques);
        }

        p.remove(&v);
    }
}

// Bron-Kerbosch but using a stack instead of recursion because that is easier
// for me to optimize. Though "optimize" might be a generous description
fn find_largest_clique(p: HashSet<[u8; 2]>, graph: &HashMap<[u8; 2], HashSet<[u8; 2]>>) -> HashSet<[u8; 2]> {
    let mut stack = Vec::new();
    stack.push((HashSet::new(), p, HashSet::new()));

    let mut largest = HashSet::new();

    while let Some((r, p, mut x)) = stack.pop() {
        let mut original_p = p.clone();

        for n in p.iter() {
            let mut next_r = r.clone();
            next_r.insert(n.clone());

            let next_p: HashSet<[u8; 2]> = original_p.intersection(&graph[n]).copied().collect();
            let next_x: HashSet<[u8; 2]> = x.intersection(&graph[n]).copied().collect();

            if next_p.is_empty() && next_x.is_empty() {
                if next_r.len() > largest.len() {
                    largest = next_r;
                }
            } else {
                stack.push((next_r, next_p, next_x.clone()));
            }
            original_p.remove(n);
            x.insert(n.clone());
        }
    }
    return largest;
}

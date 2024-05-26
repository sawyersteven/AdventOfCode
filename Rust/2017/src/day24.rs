use crate::Base;
use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
};

pub struct Day24 {
    nodes: Vec<Node>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 { nodes: Vec::new() };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let ab: Vec<isize> = line.split('/').map(|x| x.parse().unwrap()).collect();
            self.nodes.push(Node::new(ab[0], ab[1]));
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut best_str = 0;
        let mut longest = 0;
        let mut strongest_longest_p2 = 0;

        let mut stack = VecDeque::<Node>::new();
        stack.push_back(Node::new(0, 0));

        while stack.len() > 0 {
            let current = stack.pop_back().unwrap();

            best_str = best_str.max(current.strength);
            if current.used_ids.len() > longest {
                longest = current.used_ids.len();
                strongest_longest_p2 = current.strength;
            } else if current.used_ids.len() == longest {
                strongest_longest_p2 = strongest_longest_p2.max(current.strength);
            }

            let connection_value = if current.reversed { current.a } else { current.b };

            for node in &self.nodes {
                if current.used_ids.contains(&node.id) || current.id == node.id {
                    continue;
                }

                if node.a == connection_value {
                    let mut next = Node::new(node.a, node.b);
                    next.used_ids = next.used_ids.union(&current.used_ids).map(|x| *x).collect();
                    next.used_ids.insert(current.id);
                    next.strength = current.strength + next.a + next.b;
                    stack.push_back(next);
                } else if node.b == connection_value {
                    let mut next = Node::new(node.a, node.b);
                    next.reversed = true;
                    next.used_ids = next.used_ids.union(&current.used_ids).map(|x| *x).collect();
                    next.used_ids.insert(current.id);
                    next.strength = current.strength + next.a + next.b;
                    stack.push_back(next);
                }
            }
        }

        println!("Part 2: {}", strongest_longest_p2);
        return Box::new(best_str);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new('^');
    }
}

struct Node {
    a: isize,
    b: isize,
    reversed: bool,
    used_ids: HashSet<isize>,
    strength: isize,
    id: isize,
}

impl Node {
    pub fn new(a: isize, b: isize) -> Self {
        return Node {
            a,
            b,
            reversed: false,
            used_ids: HashSet::new(),
            strength: 0,
            id: a + (b << 16),
        };
    }
}

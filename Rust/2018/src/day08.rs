use crate::Base;
use std::fmt::Display;

pub struct Day08 {
    input: Vec<String>,
    ints: Vec<isize>,
    root_node: Node,
}

impl Day08 {
    pub fn new() -> Day08 {
        return Day08 {
            input: Vec::new(),
            ints: Vec::new(),
            root_node: Node {
                offset: 0,
                length: 0,
                header: (0, 0),
                children_len: 0,
                children: Vec::with_capacity(0),
                metadata: Vec::with_capacity(0),
            },
        };
    }
}

impl Base for Day08 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
        self.ints = self.input[0].split(' ').map(|x| x.parse().unwrap()).collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut root = Node::new(&self.ints, 0);
        self.make_children(&mut root);
        let count = self.count_metadata_vals(&root);
        self.root_node = root;
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new(self.count_node_values(&self.root_node));
    }
}

impl Day08 {
    fn count_node_values(&self, root: &Node) -> isize {
        let mut total = 0;
        if root.children.len() == 0 {
            for i in &root.metadata {
                total += i;
            }
        } else {
            for i in &root.metadata {
                if (i - 1) < root.children.len() as isize {
                    total += self.count_node_values(&root.children[(i - 1) as usize]);
                }
            }
        }
        return total;
    }
    fn count_metadata_vals(&self, root: &Node) -> isize {
        let mut count = 0;
        for i in &root.metadata {
            count += i;
        }
        for child in &root.children {
            count += self.count_metadata_vals(&child);
        }
        return count;
    }

    fn make_children(&mut self, n: &mut Node) -> isize {
        let mut working_offset = n.offset + 2;

        for _ in 0..n.children_len {
            let mut child = Node::new(&self.ints, working_offset);
            let child_len = self.make_children(&mut child);
            n.children.push(child);
            n.length += child_len;
            working_offset += child_len;
        }

        let k = (n.offset + n.length) as usize - n.metadata.len();
        for i in 0..n.metadata.len() {
            let m = self.ints[k + i];
            n.metadata[i] = m;
        }
        return n.length;
    }
}

#[derive(Debug)]
#[allow(unused)]
struct Node {
    offset: isize,
    length: isize,
    header: (isize, isize),
    children_len: isize,
    children: Vec<Node>,
    metadata: Vec<isize>,
}

impl Node {
    pub fn new(ints: &Vec<isize>, offset: isize) -> Self {
        let header = (ints[offset as usize], ints[offset as usize + 1]);

        return Node {
            offset: offset,
            length: 2 + header.1,
            header: header,
            children_len: header.0,
            children: Vec::with_capacity(header.0 as usize),
            metadata: vec![0; header.1 as usize],
        };
    }
}

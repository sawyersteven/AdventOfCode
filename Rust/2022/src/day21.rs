use itertools::Itertools;

use crate::Base;
use std::{
    fmt::{Display, Error},
    str::FromStr,
};

#[derive(Clone, Debug)]
enum Half {
    Name(String),
    Value(isize),
}

impl FromStr for Half {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.parse() {
            Ok(v) => Ok(Half::Value(v)),
            Err(_) => Ok(Half::Name(s.to_string())),
        };
    }
}

#[derive(Clone, Debug)]
pub struct Node {
    name: String,
    value: Option<isize>,
    lhs: Half,
    rhs: Half,
    operation: char,
}

impl Node {
    fn calc(&self) -> Option<isize> {
        let a = match self.lhs {
            Half::Name(_) => return None,
            Half::Value(v) => v,
        };
        let b = match self.rhs {
            Half::Name(_) => return None,
            Half::Value(v) => v,
        };

        return match self.operation {
            '+' => Some(a + b),
            '-' => Some(a - b),
            '/' => Some(a / b),
            '*' => Some(a * b),
            _ => unreachable!(),
        };
    }

    /// Solves for RHS in the equation `LHS <self.op> RHS = ANSWER
    fn solve_for_rhs(&self, answer: isize) -> isize {
        let l = match self.lhs {
            Half::Name(_) => panic!("I'm sorry Dave, I'm afraid I can't do that."),
            Half::Value(v) => v,
        };
        return match self.operation {
            '+' => answer - l,
            '-' => l - answer,
            '/' => l / answer,
            '*' => answer / l,
            '=' => l,
            _ => unreachable!(),
        };
    }

    /// Solves for LHS in the equation `LHS <op> RHS = ANSWER
    fn solve_for_lhs(&self, answer: isize) -> isize {
        let r = match self.rhs {
            Half::Name(_) => panic!("I'm sorry Dave, I'm afraid I can't do that."),
            Half::Value(v) => v,
        };
        return match self.operation {
            '+' => answer - r,
            '-' => answer + r,
            '/' => answer * r,
            '*' => answer / r,
            '=' => r,
            _ => unreachable!(),
        };
    }
}

pub struct Day21 {
    pub input: Vec<Node>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 { input: Vec::new() };
    }

    fn reduce_nodes(&self, nodes: &mut Vec<Node>) {
        while nodes.len() > 1 {
            let og_len = nodes.len();
            let mut len = nodes.len();
            // for each node
            let mut i = 0;
            while i < len {
                // if that node has a known numeric value
                match nodes[i].value {
                    Some(v) => {
                        // look through the rest of the nodes
                        for j in 0..nodes.len() {
                            // skip those that already have a known value
                            if i == j || nodes[j].value.is_some() {
                                continue;
                            }
                            // for equation halves that can swap in the value
                            match &nodes[j].lhs {
                                Half::Name(n) => {
                                    if *n == nodes[i].name {
                                        if n == "humn" {
                                            println!("Replacing humn in {} with {}", nodes[j].name, v);
                                        }
                                        nodes[j].lhs = Half::Value(v);
                                    }
                                }
                                Half::Value(_) => {}
                            }
                            match &nodes[j].rhs {
                                Half::Name(n) => {
                                    if *n == nodes[i].name {
                                        if n == "humn" {
                                            println!("Replacing humn in {} with {}", nodes[j].name, v);
                                        }
                                        nodes[j].rhs = Half::Value(v);
                                    }
                                }
                                Half::Value(_) => {}
                            }
                            // try solving this node
                            nodes[j].value = nodes[j].calc();
                        }
                        // remove this node
                        nodes.remove(i);
                        len -= 1;
                    }
                    None => {}
                }
                i += 1;
            }

            if og_len == nodes.len() {
                break;
            }
        }
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts: Vec<&str> = line.split(|x| [':', ' '].contains(&x)).collect();

            if parts.len() == 3 {
                self.input.push(Node {
                    name: parts[0].to_string(),
                    value: Some(parts[2].parse().unwrap()),
                    lhs: Half::Value(0),
                    rhs: Half::Value(0),
                    operation: ' ',
                });
            } else {
                self.input.push(Node {
                    name: parts[0].to_string(),
                    value: None,
                    lhs: Half::from_str(parts[2]).unwrap(),
                    rhs: Half::from_str(parts[4]).unwrap(),
                    operation: parts[3].chars().nth(0).unwrap(),
                });
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut nodes = self.input.clone();
        self.reduce_nodes(&mut nodes);

        return Box::new(nodes[0].value.unwrap());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut nodes = self.input.clone();
        let mut root;
        for i in 0..nodes.len() {
            if nodes[i].name == String::from("humn") {
                nodes[i].lhs = Half::from_str("ANS").unwrap();
                nodes[i].value = None;
            }
            if nodes[i].name == String::from("root") {
                nodes[i].operation = '=';
            }
        }

        self.reduce_nodes(&mut nodes);

        root = nodes.iter().find_or_first(|x| x.name == "root").unwrap();

        let mut child_name = "".to_string();
        let mut required_answer = 0;
        match &root.lhs {
            Half::Name(n) => {
                child_name = n.clone();
                match &root.rhs {
                    Half::Name(n) => child_name = n.clone(),
                    Half::Value(v) => required_answer = *v,
                }
            }
            Half::Value(v) => required_answer = *v,
        };

        while root.name != "humn" {
            match &root.lhs {
                Half::Name(n) => child_name = n.clone(),
                Half::Value(_) => required_answer = root.solve_for_rhs(required_answer),
            }

            match &root.rhs {
                Half::Name(n) => child_name = n.clone(),
                Half::Value(_) => required_answer = root.solve_for_lhs(required_answer),
            }

            root = nodes.iter().find_or_first(|x| x.name == child_name).unwrap();
        }

        return Box::new(required_answer);
    }
}

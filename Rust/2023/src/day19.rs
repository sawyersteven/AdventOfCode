use crate::Base;
use std::{collections::HashMap, fmt::Display, ops::Range};

pub struct Day19 {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            workflows: HashMap::new(),
            parts: Vec::new(),
        };
    }

    // Split part into two parts -- one that passes the rule and one that fails
    fn split_part(&self, mut part: PartRanges, rule: &Rule) -> (PartRanges, PartRanges) {
        let mut part_pass = part.clone();

        let og_range = &part[&rule.field];

        if rule.op == OP::GT {
            part_pass.insert(rule.field.clone(), (rule.value + 1)..(og_range.end));
            part.insert(rule.field.clone(), (og_range.start)..(rule.value + 1));
        } else {
            part_pass.insert(rule.field.clone(), (og_range.start)..(rule.value));
            part.insert(rule.field.clone(), (rule.value)..(og_range.end));
        }

        return (part_pass, part);
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        // Parse workflows. This is ugly and takes longer than either puzzle
        for line in raw_input.split("\n\n").nth(0).unwrap().lines() {
            let name = line.split('{').nth(0).unwrap();
            let rules = &line[(name.len() + 1)..(line.len() - 1)]
                .split(',')
                .collect::<Vec<&str>>();

            let mut wf = Workflow {
                rules: Vec::new(),
                default: WFResult::from_string(rules[rules.len() - 1]),
            };

            for r in &rules[0..(rules.len() - 1)] {
                let field = match r.as_bytes()[0] {
                    b'x' => Field::x,
                    b'm' => Field::m,
                    b'a' => Field::a,
                    b's' => Field::s,
                    _ => panic!(),
                };

                let op = match r.as_bytes()[1] {
                    b'>' => OP::GT,
                    b'<' => OP::LT,
                    _ => panic!(),
                };

                let val = r
                    .split(|x| x == '<' || x == '>')
                    .nth(1)
                    .unwrap()
                    .split(':')
                    .nth(0)
                    .unwrap()
                    .parse()
                    .unwrap();

                let res = WFResult::from_string(r.split(':').nth(1).unwrap());

                wf.rules.push(Rule {
                    field: field,
                    op: op,
                    value: val,
                    result: res,
                });
            }
            self.workflows.insert(name.to_string(), wf);
        }

        // Parse parts
        for line in raw_input.split("\n\n").nth(1).unwrap().lines() {
            self.parts.push(Part::from_string(line));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut sum = 0;
        for part in &self.parts {
            let mut res = WFResult::Route("in".to_string());
            loop {
                match res {
                    WFResult::Accept() => {
                        sum += part.x;
                        sum += part.m;
                        sum += part.a;
                        sum += part.s;
                        break;
                    }
                    WFResult::Reject() => break,
                    WFResult::Route(r) => {
                        res = self.workflows[&r].process(part);
                    }
                }
            }
        }
        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let start = WFResult::Route("in".to_string());

        let mut q = Vec::new();
        q.push((PartRanges::base(), &start));

        let mut sm = 0;
        while let Some((mut part, wfr)) = q.pop() {
            match wfr {
                WFResult::Accept() => {
                    sm += (part[&Field::x].end - part[&Field::x].start)
                        * (part[&Field::m].end - part[&Field::m].start)
                        * (part[&Field::a].end - part[&Field::a].start)
                        * (part[&Field::s].end - part[&Field::s].start);
                }
                WFResult::Reject() => continue,
                WFResult::Route(rt) => {
                    for rule in &self.workflows[rt].rules {
                        let (part_true, part_false) = self.split_part(part, rule);
                        q.push((part_true, &rule.result));
                        part = part_false;
                    }
                    q.push((part, &self.workflows[rt].default));
                }
            }
        }
        return Box::new(sm);
    }
}
#[derive(PartialEq)]
enum OP {
    GT,
    LT,
}

type PartRanges = HashMap<Field, Range<isize>>;
trait PartRs {
    fn base() -> Self;
}

impl PartRs for PartRanges {
    fn base() -> Self {
        return PartRanges::from([
            (Field::x, 1..4001isize),
            (Field::m, 1..4001isize),
            (Field::a, 1..4001isize),
            (Field::s, 1..4001isize),
        ]);
    }
}

struct Workflow {
    rules: Vec<Rule>,
    default: WFResult,
}
impl Workflow {
    fn process(&self, part: &Part) -> WFResult {
        for r in &self.rules {
            match r.process(part) {
                Some(res) => return res,
                None => {}
            }
        }
        return self.default.clone();
    }
}

struct Rule {
    field: Field,
    op: OP,
    value: isize,
    result: WFResult,
}

impl Rule {
    fn process(&self, part: &Part) -> Option<WFResult> {
        let pv = match self.field {
            Field::x => part.x,
            Field::m => part.m,
            Field::a => part.a,
            Field::s => part.s,
        };

        if self.op(pv) {
            return Some(self.result.clone());
        } else {
            return None;
        }
    }

    fn op(&self, a: isize) -> bool {
        if self.op == OP::GT {
            return a > self.value;
        } else {
            return a < self.value;
        };
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
enum Field {
    x,
    m,
    a,
    s,
}

struct Part {
    x: isize,
    m: isize,
    a: isize,
    s: isize,
}

impl Part {
    fn from_string(input: &str) -> Part {
        let parts = input[1..(input.len() - 1)]
            .split(',')
            .map(|x| x.split('=').nth(1).unwrap().parse().unwrap())
            .collect::<Vec<isize>>();

        return Part {
            x: parts[0],
            m: parts[1],
            a: parts[2],
            s: parts[3],
        };
    }
}

#[derive(Clone, Debug, PartialEq)]
enum WFResult {
    Accept(),
    Reject(),
    Route(String),
}

impl WFResult {
    fn from_string(value: &str) -> WFResult {
        return match value {
            "A" => WFResult::Accept(),
            "R" => WFResult::Reject(),
            s => WFResult::Route(s.to_string()),
        };
    }
}

use crate::Base;
use std::{fmt::Display, ops::RangeInclusive};

pub struct Day16 {
    all_tickets: Vec<Vec<usize>>,
    valid_tickets: Vec<Vec<usize>>,
    my_ticket: Vec<usize>,
    rules: Vec<Rule>,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            all_tickets: Vec::new(),
            valid_tickets: Vec::new(),
            my_ticket: Vec::new(),
            rules: Vec::new(),
        };
    }

    fn find_invalid_field(&self, ticket: &Vec<usize>) -> Option<usize> {
        for val in ticket {
            if !self.rules.iter().any(|rule| rule.in_range(*val)) {
                return Some(*val);
            }
        }
        return None;
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            if line.is_empty() {
                break;
            }

            let mut parts = line.splitn(2, ": ");
            let name = parts.next().unwrap();

            let mut ab = parts.next().unwrap().split(" or ");
            let mut a = ab.next().unwrap().split('-');
            let ra = a.next().unwrap().parse().unwrap()..=a.next().unwrap().parse().unwrap();

            let mut b = ab.next().unwrap().split('-');
            let rb = b.next().unwrap().parse().unwrap()..=b.next().unwrap().parse().unwrap();

            self.rules.push(Rule {
                name: name.to_string(),
                range_a: ra,
                range_b: rb,
                possible_fields: Vec::new(),
            });
        }

        for t in raw_input.split("nearby tickets:\n").nth(1).unwrap().lines() {
            self.all_tickets
                .push(t.split(',').map(|x| x.parse().unwrap()).collect());
        }

        self.my_ticket = raw_input
            .split("your ticket:\n")
            .nth(1)
            .unwrap()
            .lines()
            .nth(0)
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut ans = 0;
        for ticket in &self.all_tickets {
            match self.find_invalid_field(&ticket) {
                Some(f) => {
                    ans += f;
                }
                None => self.valid_tickets.push(ticket.clone()),
            }
        }

        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        for rule in self.rules.iter_mut() {
            for field_num in 0..self.valid_tickets[0].len() {
                let mut valid = true;
                for t in &self.valid_tickets {
                    if !rule.in_range(t[field_num]) {
                        valid = false;
                        break;
                    }
                }
                if valid {
                    rule.possible_fields.push(field_num);
                }
            }
        }

        loop {
            for r1 in 0..self.rules.len() {
                if self.rules[r1].possible_fields.len() == 1 {
                    for r2 in (0..self.rules.len()).filter(|x| *x != r1) {
                        if let Some(ind) = self.rules[r2]
                            .possible_fields
                            .iter()
                            .position(|x| *x == self.rules[r1].field_position())
                        {
                            self.rules[r2].possible_fields.remove(ind);
                        }
                    }
                }
            }
            if !self.rules.iter().any(|x| x.possible_fields.len() > 1) {
                break;
            }
        }

        let mut ans = 1;
        for rule in &self.rules {
            if rule.name.starts_with("departure") {
                ans *= self.my_ticket[rule.field_position()];
            }
        }

        return Box::new(ans);
    }
}

struct Rule {
    name: String,
    range_a: RangeInclusive<usize>,
    range_b: RangeInclusive<usize>,
    possible_fields: Vec<usize>,
}

impl Rule {
    fn in_range(&self, num: usize) -> bool {
        return self.range_a.contains(&num) || self.range_b.contains(&num);
    }

    fn field_position(&self) -> usize {
        if self.possible_fields.len() != 0 {
            return self.possible_fields[0];
        }
        return usize::MAX;
    }
}

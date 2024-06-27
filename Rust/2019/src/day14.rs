use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day14 {
    // name: (amount made, requirements)
    recipes: HashMap<String, (usize, Vec<Ingredient>)>,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 {
            recipes: HashMap::new(),
        };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let io: Vec<&str> = line.split(" => ").collect();
            let out_parts: Vec<&str> = io[1].split(" ").collect();
            let mut l = Vec::new();
            for ing in io[0].split(", ") {
                l.push(Ingredient::new(ing));
            }
            self.recipes
                .insert(out_parts[1].to_string(), (out_parts[0].parse().unwrap(), l));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.make_fuel(1));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        const ORE_RESERVES: usize = 1000000000000;

        let mut lower: isize = 1;
        let mut upper: isize = -1;
        let mut amount: isize;

        // Binary search to see what `amt` value yield the most fuel without
        // going over ore reserve limit
        while lower + 1 != upper {
            if upper == -1 {
                amount = lower * 2;
            } else {
                amount = (upper + lower) / 2;
            }

            let req_ore = self.make_fuel(amount as usize);

            if req_ore > ORE_RESERVES {
                upper = amount;
            } else {
                lower = amount;
            }
        }
        return Box::new(lower);
    }
}

impl Day14 {
    fn make_fuel(&mut self, amt: usize) -> usize {
        let mut leftovers = HashMap::new();

        let mut required_ore = 0;
        let mut orders = VecDeque::new();

        orders.push_back(Ingredient {
            name: "FUEL".to_string(),
            amount: amt,
        });

        while let Some(order) = orders.pop_front() {
            let existing = *leftovers.get(&order.name).unwrap_or(&0);
            if order.name == "ORE" {
                required_ore += order.amount;
            } else if order.amount <= existing {
                leftovers.insert(order.name, existing - order.amount);
            } else {
                let req_amt = order.amount - existing;
                let recipe = &self.recipes[&order.name];
                let iters = div_round_up(req_amt as usize, recipe.0);
                for ing in &recipe.1 {
                    orders.push_back(Ingredient {
                        name: ing.name.clone(),
                        amount: ing.amount * iters,
                    });
                }
                leftovers.insert(order.name, (iters * recipe.0) - req_amt);
            }
        }
        return required_ore;
    }
}

fn div_round_up(a: usize, b: usize) -> usize {
    return ((a - 1) / b) + 1;
}

struct Ingredient {
    name: String,
    amount: usize,
}

impl Ingredient {
    fn new(raw: &str) -> Self {
        let parts: Vec<&str> = raw.split(' ').collect();
        return Ingredient {
            amount: parts[0].parse().unwrap(),
            name: parts[1].to_string(),
        };
    }
}

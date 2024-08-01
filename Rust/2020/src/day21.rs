use itertools::Itertools;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day21 {
    allergen_map: HashMap<String, Vec<String>>,
    ingredient_pile: Vec<String>,
    safe_ingredients: HashSet<String>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            allergen_map: HashMap::new(),
            ingredient_pile: Vec::new(),
            safe_ingredients: HashSet::new(),
        };
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        // make allergen map
        for line in raw_input.lines() {
            let mut parts = line.split(" (contains ");
            let recipe: Vec<String> = parts.next().unwrap().split(' ').map(|x| x.to_string()).collect();
            let a = parts.next().unwrap();
            let allergens = a[..(a.len() - 1)].split(", ");

            self.ingredient_pile.extend(recipe.clone().into_iter());

            for a in allergens {
                if !self.allergen_map.contains_key(a) {
                    self.allergen_map
                        .insert(a.to_string(), Vec::from_iter(recipe.clone().into_iter()));
                } else {
                    self.allergen_map.get_mut(a).unwrap().retain(|x| recipe.contains(x));
                }
            }

            for r in recipe {
                if !self.safe_ingredients.contains(&r) {
                    self.safe_ingredients.insert(r);
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut unsafe_ingredients = HashSet::new();
        let mut reduced = HashSet::new();

        while reduced.len() != self.allergen_map.len() {
            let keys: Vec<String> = self.allergen_map.keys().cloned().collect();
            for k in &keys {
                if !reduced.contains(k) && self.allergen_map[k].len() == 1 {
                    reduced.insert(k.clone());
                    let val = self.allergen_map[k][0].clone();
                    unsafe_ingredients.insert(val.clone());
                    for k2 in &keys {
                        if k == k2 {
                            continue;
                        }
                        self.allergen_map.get_mut(k2).unwrap().retain(|x| *x != val);
                    }
                }
            }
        }

        self.safe_ingredients.retain(|x| !unsafe_ingredients.contains(x));

        let ans = self
            .ingredient_pile
            .iter()
            .filter(|x| self.safe_ingredients.contains(*x))
            .count();

        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut ordered_ingredients = Vec::with_capacity(self.allergen_map.len());
        for an in self.allergen_map.keys().cloned().sorted() {
            ordered_ingredients.push(self.allergen_map.get_mut(&an).unwrap().pop().unwrap());
        }

        return Box::new(ordered_ingredients.join(","));
    }
}

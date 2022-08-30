use itertools::Itertools;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day19 {
    pub replacements: Vec<(String, String)>,
    pub base_molecule: String,
}

impl Day19 {
    pub fn new() -> Day19 {
        return Day19 {
            replacements: Vec::new(),
            base_molecule: String::new(),
        };
    }
}

impl Base for Day19 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            if line.is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split(" => ").collect();
            self.replacements.push((parts[0].to_string(), parts[1].to_string()));
        }

        self.base_molecule = raw_input.split('\n').last().unwrap().to_string();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut combos: HashSet<String> = HashSet::new();

        for (remove, add) in &self.replacements {
            for (ind, _) in self.base_molecule.match_indices(remove.as_str()) {
                let mut replaced = self.base_molecule.clone();
                replaced.replace_range(ind..(ind + remove.len()), &add);

                combos.insert(replaced);
            }
        }

        return Box::new(combos.len());
    }

    /// An interesting aspect of the input that makes this part much easier
    /// is that each output molecule Y of X => Y only exists once, so a
    /// map can be made of Y: X without addressing duplicate keys.
    ///
    /// I attempted to simply port a solution I had written in C#, but that
    /// must have been done with a different input that is slightly more
    /// forgiving than this input (or perhaps this only works on this)
    /// specific input.
    ///
    /// So this works by replacing as many of the longest output molecules
    /// as possible each iteration, starting the iteration again after a
    /// replacement occurs. This results in the molecule shrinking as much
    /// as possible every step. Luckily this didn't lead to a dead end.
    fn part2(&mut self) -> Box<dyn Display> {
        let mut map: HashMap<String, String> = HashMap::new();

        for (a, b) in &self.replacements {
            map.insert(b.to_owned(), a.to_owned());
        }

        let sorted_keys: Vec<String> = map
            .keys()
            .sorted_by(|x, y| Ord::cmp(&y.len(), &x.len()))
            .map(|x| x.to_owned())
            .collect();

        let mut current = self.base_molecule.clone();
        let mut count = 0;

        while current != "e" {
            for key in &sorted_keys {
                if current.contains(key.as_str()) {
                    count += current.matches(key).count();
                    current = current.replace(key.as_str(), map[key].as_str());
                    break;
                }
            }
        }

        return Box::new(count);
    }
}

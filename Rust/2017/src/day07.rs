use itertools::Itertools;

use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day07 {
    pub input: String,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            input: String::from(""),
        };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut parents = HashSet::<String>::new();
        let mut children = HashSet::<String>::new();

        for line in self.input.split('\n') {
            parents.insert(line.split(' ').nth(0).unwrap().to_string());
            if line.contains('>') {
                for child in line.split("> ").nth(1).unwrap().split(", ") {
                    children.insert(child.to_string());
                }
            }
        }

        return Box::new(parents.difference(&children).nth(0).unwrap().to_owned());
    }

    fn part2(&self) -> Box<dyn Display> {
        let root_cmd = "gynfwly";

        let mut weights = HashMap::<&str, isize>::new();
        let mut children = HashMap::<&str, Vec<&str>>::new();

        for line in self.input.split('\n') {
            let name = line.split(' ').nth(0).unwrap();
            let weight: isize = line
                .split('(')
                .nth(1)
                .unwrap()
                .split(')')
                .nth(0)
                .unwrap()
                .parse()
                .unwrap();

            weights.insert(name, weight);

            if !line.contains("> ") {
                continue;
            }

            let c: Vec<&str> = line.split("> ").nth(1).unwrap().split(", ").collect();
            children.insert(name, c);
        }

        let (w, _) = find_weight_correction(root_cmd, &weights, &children);

        return Box::new(w);
    }
}

/// Recursively finds odd child and returns corrected weight by
/// checking weights of each child until the incorrect one is found.
/// Then get the child's name from the mismatch index, and use the name
/// to find that child's individual weight to apply the difference to

fn find_weight_correction(
    current: &str,
    weights: &HashMap<&str, isize>,
    children: &HashMap<&str, Vec<&str>>,
) -> (isize, bool) {
    // weight, is_weight_correction
    if !children.contains_key(current) {
        return (weights[current], false);
    }

    let mut child_weights = Vec::new();
    for child_name in &children[current] {
        let (w, is_corrected) = find_weight_correction(child_name, weights, children);
        if is_corrected {
            return (w, is_corrected);
        }
        child_weights.push(w);
    }

    if !child_weights.iter().all_equal() {
        let (delta, index) = find_weight_delta(&child_weights);
        return (weights[children[current][index]] - delta, true);
    }

    let mut sum = 0;
    child_weights.iter().for_each(|x| sum += x);
    sum += weights[current];
    return (sum, false);
}

// Returns the difference and index of unique number
fn find_weight_delta(nums: &Vec<isize>) -> (isize, usize) {
    for i in 1..(nums.len() - 1) {
        if nums[i] != nums[i - 1] && nums[i] != nums[i + 1] {
            return (nums[i] - nums[i + 1], i);
        }
    }

    if nums[0] != nums[1] {
        return (nums[0] - nums[1], 0);
    }

    if nums[nums.len() - 1] != nums[nums.len() - 2] {
        return (nums[nums.len() - 1] - nums[nums.len() - 2], nums.len() - 1);
    }
    println!("{:#?}", nums);
    println!("{:#?}", nums.iter().all_equal());
    panic!();
}

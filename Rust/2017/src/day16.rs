use shared::circular_deque::CircularDeque;

use crate::Base;
use std::fmt::Display;

pub struct Day16 {
    pub instructions: Vec<String>,
}

impl Day16 {
    pub fn new() -> Day16 {
        return Day16 {
            instructions: Vec::new(),
        };
    }
}

impl Base for Day16 {
    fn parse_input(&mut self, raw_input: String) {
        self.instructions = raw_input.split(',').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut circle = CircularDeque::<char>::new();

        for i in 'a'..='p' {
            circle.add_last(i);
        }

        apply_dance(&self.instructions, &mut circle);

        return Box::new(String::from_iter(circle.to_vec()));
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut circle = CircularDeque::<char>::new();

        for i in 'a'..='p' {
            circle.add_last(i);
        }

        let mut history = Vec::<String>::new(); // rust hashsets don't preserve order :(
        let mut pattern_len = 0;
        for i in 0..usize::MAX {
            apply_dance(&self.instructions, &mut circle);
            let id = String::from_iter(circle.to_vec().iter().map(|x| x.to_string()));
            if history.contains(&id) {
                pattern_len = i;
                break;
            }
            history.push(id);
        }

        let hist: Vec<String> = history.iter().map(|x| x.to_string()).collect();

        return Box::new(hist[(1_000_000_000 - 1) % pattern_len].to_owned());
    }
}

fn apply_dance(instructions: &Vec<String>, circle: &mut CircularDeque<char>) {
    for instruction in instructions {
        match instruction.chars().nth(0).unwrap() {
            's' => {
                let l = instruction[1..].parse().unwrap();
                circle.move_head_left(l);
            }
            'x' => {
                let indicies: Vec<isize> = instruction[1..].split('/').map(|x| x.parse().unwrap()).collect();
                circle.swap(indicies[0], indicies[1]);
            }
            'p' => {
                let x = instruction.chars().nth(1).unwrap();
                let y = instruction.chars().nth(3).unwrap();

                let x_ind = circle.first_index_of(x).unwrap();
                let y_ind = circle.first_index_of(y).unwrap();

                circle[x_ind] = y;
                circle[y_ind] = x;
            }
            _ => panic!(),
        }
    }
}

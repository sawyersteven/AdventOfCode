use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    pub words: Vec<String>,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { words: Vec::new() };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.words = raw_input.split(',').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut total = 0;
        for word in &self.words {
            let mut value = 0;
            for c in word.chars() {
                value += c as usize;
                value *= 17;
                value %= 256;
            }
            total += value;
        }
        return Box::new(total);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut boxes = vec![Vec::<Lens>::new(); 256];

        for word in &self.words {
            let mut box_num = 0;
            let (lens, focal_len) = word.split_once(|x| x == '-' || x == '=').unwrap();

            for c in lens.chars() {
                box_num += c as usize;
                box_num *= 17;
                box_num %= 256;
            }

            if word.contains('-') {
                match boxes[box_num].iter().position(|x| x.label == lens) {
                    Some(i) => {
                        boxes[box_num].remove(i);
                    }
                    None => (),
                }
            } else {
                let l = Lens {
                    label: lens.to_string(),
                    len: focal_len.parse().unwrap(),
                };
                match boxes[box_num].iter().position(|x| x.label == lens) {
                    Some(i) => {
                        boxes[box_num].remove(i);
                        boxes[box_num].insert(i, l);
                    }
                    None => boxes[box_num].push(l),
                }
            }
        }

        let mut total = 0;
        for (i, bx) in boxes.iter().enumerate() {
            for (j, lns) in bx.iter().enumerate() {
                total += (i + 1) * (j + 1) * lns.len;
            }
        }

        return Box::new(total);
    }
}

#[derive(PartialEq, Debug, Clone)]
struct Lens {
    label: String,
    len: usize,
}

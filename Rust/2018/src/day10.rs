use regex::Regex;

use crate::Base;
use std::fmt::Display;

const LETTERHEIGHT: isize = 10;

pub struct Day10 {
    stars: Vec<Star>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { stars: Vec::new() };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        let re = Regex::new(r"[.*]?-?(\d+)").unwrap();

        for line in raw_input.split('\n') {
            let mut matches = Vec::<isize>::new();
            for m in re.find_iter(line) {
                matches.push(m.as_str().parse().unwrap());
            }
            self.stars.push(Star {
                pos_x: matches[0],
                pos_y: matches[1],
                vel_x: matches[2],
                vel_y: matches[3],
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        for seconds in 1..usize::MAX {
            for star in &mut self.stars {
                star.pos_x += star.vel_x;
                star.pos_y += star.vel_y;
            }

            let max_x = self.stars.iter().map(|s| s.pos_x).max().unwrap();
            let min_x = self.stars.iter().map(|s| s.pos_x).min().unwrap();
            let max_y = self.stars.iter().map(|s| s.pos_y).max().unwrap();
            let min_y = self.stars.iter().map(|s| s.pos_y).min().unwrap();

            if max_y - min_y + 1 == LETTERHEIGHT {
                let mut grid = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
                for star in &self.stars {
                    grid[(star.pos_y - min_y) as usize][(star.pos_x - min_x) as usize] = '#';
                }
                let mut output = String::from('\n');
                for line in grid {
                    for c in line {
                        output.push(c);
                    }
                    output.push('\n');
                }

                output.push_str(&*format!("\t\t\t  {}", seconds));
                return Box::new(output);
            }
        }

        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("^^^");
    }
}

#[derive(Debug)]
struct Star {
    pos_x: isize,
    pos_y: isize,
    vel_x: isize,
    vel_y: isize,
}

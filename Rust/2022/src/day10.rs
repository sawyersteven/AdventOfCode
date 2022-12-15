use crate::Base;
use std::fmt::Display;

pub struct Day10 {
    pub input: Vec<isize>,
}

impl Day10 {
    pub fn new() -> Day10 {
        return Day10 { input: Vec::new() };
    }
}

impl Base for Day10 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let val = line.split(' ').nth(1).unwrap_or("0").parse().unwrap();
            self.input.push(val);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut x = 1;
        let mut sum = 0;

        let mut cycle = 1;
        for line in &self.input {
            if line != &0 {
                cycle += 1;
                if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                    sum += x * cycle;
                }
                x += line;
            }
            cycle += 1;
            if [20, 60, 100, 140, 180, 220].contains(&cycle) {
                sum += x * cycle;
            }
        }

        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut x = 1;
        let mut screen_matrix = vec!['.'; 40 * 6];

        let mut cycle = 0;
        for line in &self.input {
            if (cycle % 40usize).abs_diff(x as usize) <= 1 {
                screen_matrix[cycle] = '#';
            }
            if line != &0 {
                cycle += 1;
                if (cycle % 40usize).abs_diff(x as usize) <= 1 {
                    screen_matrix[cycle] = '#';
                }
                x += line;
            }
            cycle += 1;
        }

        let mut displ = String::from_iter(screen_matrix.iter());
        for n in (0..displ.len()).step_by(40).rev() {
            displ.insert(n, '\n');
        }
        println!("{}", displ);
        return Box::new("-");
    }
}

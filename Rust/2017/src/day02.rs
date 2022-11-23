use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    pub input: Vec<Vec<usize>>,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: Vec::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let row = line.split('\t').map(|f| f.parse().unwrap()).collect();
            self.input.push(row);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for line in &self.input {
            let mut min = usize::MAX;
            let mut max = 0;

            for val in line {
                max = max.max(*val);
                min = min.min(*val);
            }
            sum += max - min;
        }
        return Box::new(sum);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for line in &self.input {
            let mut line_done = false;

            for i in 0..(line.len() - 1) {
                for j in (i + 1)..line.len() {
                    if line[i] % line[j] == 0 {
                        sum += line[i] / line[j];
                        line_done = true;
                        break;
                    } else if line[j] % line[i] == 0 {
                        sum += line[j] / line[i];
                        line_done = true;
                        break;
                    }
                }
                if line_done {
                    break;
                }
            }
        }
        return Box::new(sum);
    }
}

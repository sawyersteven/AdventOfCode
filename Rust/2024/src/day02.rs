use crate::Base;
use std::fmt::Display;

pub struct Day02 {
    input: String,
}

impl Day02 {
    pub fn new() -> Day02 {
        return Day02 { input: String::new() };
    }
}

impl Base for Day02 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for line in self.input.lines() {
            let rep: Vec<isize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            if is_report_good(&rep) {
                count += 1;
            };
        }
        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut count = 0;
        for line in self.input.lines() {
            let rep: Vec<isize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
            let l = rep.len();
            for i in 0..l {
                if is_report_good(&[&rep[0..i], &rep[(i + 1)..l]].concat()) {
                    count += 1;
                    break;
                }
            }
        }
        return Box::new(count);
    }
}

fn is_report_good(rep: &[isize]) -> bool {
    let ascending = rep[0] < rep[1];

    return (0..(rep.len() - 1))
        .all(|i| (rep[i] < rep[i + 1]) == ascending && (1..=3).contains(&rep[i].abs_diff(rep[i + 1])));
}

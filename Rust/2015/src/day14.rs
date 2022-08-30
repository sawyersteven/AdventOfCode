use crate::Base;
use std::{fmt::Display, vec};

pub struct Rate {
    pub speed: isize,
    pub run_time: isize,
    pub rest_time: isize,
}

pub struct Day14 {
    pub racers: Vec<Rate>,
}

const TIME: isize = 2503;

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { racers: Vec::new() };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        for line in lines {
            let parts: Vec<&str> = line.split(' ').collect();
            self.racers.push(Rate {
                speed: parts[3].parse().unwrap(),
                run_time: parts[6].parse().unwrap(),
                rest_time: parts[13].parse().unwrap(),
            });
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut best: isize = 0;

        for racer in &self.racers {
            let mut remaining = TIME;
            let mut dist: isize = 0;
            loop {
                if remaining > racer.run_time {
                    dist += racer.speed * racer.run_time;
                } else if remaining > 0 {
                    dist += racer.speed * remaining;
                    break;
                } else {
                    break;
                }
                remaining -= racer.run_time;
                remaining -= racer.rest_time;
            }

            best = best.max(dist);
        }

        return Box::new(best);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut locations = vec![0; self.racers.len()];
        let mut points = vec![0; self.racers.len()];

        let mut second: isize = 0;

        while second < TIME {
            for (i, racer) in self.racers.iter().enumerate() {
                if second % (racer.run_time + racer.rest_time) < racer.run_time {
                    locations[i] += racer.speed;
                }
            }

            let mut best_dist: isize = -1;
            for loc in &locations {
                best_dist = best_dist.max(*loc);
            }

            for (i, loc) in locations.iter().enumerate() {
                if *loc == best_dist {
                    points[i] += 1;
                }
            }
            second += 1;
        }

        let high_score = points.iter().max().unwrap();

        return Box::new(*high_score);
    }
}

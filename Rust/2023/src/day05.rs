use crate::Base;
use std::{fmt::Display, ops::Range};

pub struct Day05 {
    pub seeds: Vec<isize>,
    conversion_charts: Vec<Vec<RangeDelta>>,
}

impl Day05 {
    pub fn new() -> Day05 {
        return Day05 {
            seeds: Vec::new(),
            conversion_charts: Vec::new(),
        };
    }

    fn process_seed_num(&self, mut seed: isize) -> isize {
        for chart in &self.conversion_charts {
            for range_delta in chart {
                if range_delta.range.contains(&seed) {
                    seed -= range_delta.delta;
                    break;
                }
            }
        }
        return seed;
    }
}

impl Base for Day05 {
    fn parse_input(&mut self, raw_input: String) {
        self.seeds = raw_input
            .split_once('\n')
            .unwrap()
            .0
            .split_once(": ")
            .unwrap()
            .1
            .split(' ')
            .map(|x| x.trim().parse().unwrap())
            .collect();

        let k = raw_input.split_once("\n\n").unwrap().1;
        let charts: Vec<&str> = k.split("\n\n").collect();

        for chart in charts {
            let mut conversions = Vec::new();
            let lines: Vec<&str> = chart.split('\n').collect();

            for line in lines.iter().skip(1) {
                let nums: Vec<isize> = line.split(' ').map(|x| x.parse().unwrap()).collect();
                conversions.push(RangeDelta {
                    range: nums[1]..(nums[1] + nums[2] + 1),
                    delta: nums[1] - nums[0],
                });
            }
            self.conversion_charts.push(conversions);
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut low = isize::MAX;

        for seed_num in &self.seeds {
            low = low.min(self.process_seed_num(*seed_num));
        }

        return Box::new(low);
    }

    /* This takes ~97 seconds. Run at your own risk.
    An idea for optimization would be to record duplicate numbers at each
    stage. For example, two starting seed numbers could end up with the
    same Light number, so any calculations past that would be redundant.
    I might implement that eventually.
    */
    fn part2(&self) -> Box<dyn Display> {
        let mut low = isize::MAX;

        for (i, range_start) in self.seeds.iter().enumerate().step_by(2) {
            for num in *range_start..(*range_start + self.seeds[i + 1] + 1) {
                low = low.min(self.process_seed_num(num));
            }
        }
        return Box::new(low);
    }
}

struct RangeDelta {
    pub range: Range<isize>,
    pub delta: isize,
}

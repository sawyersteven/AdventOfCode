use shared::v3i::Vector3Int;

use crate::Base;
use std::fmt::Display;

/*
Cleaned up notes from my C# solution...

This is a bit easier than the puzzle initially sounds, so I wanted to keep
copious notes about the process...

We are tasked with finding a point in 3d space that overlaps the most nanobot
ranges. The solution is the manhattan distance from (0,0,0) to this point.

Nanobot ranges are effectively cubes because their range is defined as a
manhattan distance from their location.

The solution to this puzzle must lie on one of the edges of these range
cubes because those are the only locations at which the overlap count can
increase or decrease.

Bots are constructed with a min_dist_to_zero and a max_dist_to_zero that
describe in absolute values the distance of the closest and farthest edge
to (0,0,0)

Distances to the edges of nanobot ranges are stored with a +1 or -1 value
to indicate entering or exiting a nanobot's area. These are then sorted
by distance to zero.

Iterate through the edges adding/subtracting from an overlap counter as
you handle an enter/exit edge and find the edge location where the counter
is at its highest value.

To visualize in one dimension:
    1 2   3  4 3           4 3     2 3   45  4  3  21 0
    ================================================
      ==========
          ==========================
             =================
                           ============================
                                     ========
                                         ============
                                          ======
|----------|----------|----------|----------|----------|
0          10         20         30         40         50

The range 38-43 has the highest number of nanobots in range, but since we
only care about the smallest value, the edge of the last nanobot is the
answer.

*/
pub struct Day23 {
    nanobots: Vec<Nanobot>,
}

impl Day23 {
    pub fn new() -> Day23 {
        return Day23 { nanobots: Vec::new() };
    }
}

impl Base for Day23 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            self.nanobots.push(Nanobot::new(line));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut strongest = &self.nanobots[0];
        for bot in self.nanobots.iter().skip(1) {
            if bot.radius > strongest.radius {
                strongest = bot;
            }
        }

        let mut count = 0;

        for bot in &self.nanobots {
            if strongest.man_dist(&bot) <= strongest.radius {
                count += 1;
            }
        }

        return Box::new(count);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut edges = vec![(0, 0); self.nanobots.len() * 2];
        for i in 0..self.nanobots.len() {
            edges[i * 2] = (self.nanobots[i].min_dist_to_zero, 1);
            edges[(i * 2) + 1] = (self.nanobots[i].max_dist_to_zero, -1);
        }

        edges.sort_by(|a, b| a.0.cmp(&b.0));

        let mut current_overlap = 0;
        let mut best_overlap_count = 0;
        let mut best_overlap_distance = 0;

        for (dist, transition) in edges {
            current_overlap += transition;
            if current_overlap > best_overlap_count {
                best_overlap_count = current_overlap;
                best_overlap_distance = dist;
            }
        }
        return Box::new(best_overlap_distance);
    }
}

struct Nanobot {
    location: Vector3Int,
    radius: usize,
    min_dist_to_zero: usize,
    max_dist_to_zero: usize,
}

impl Nanobot {
    pub fn new(input_line: &str) -> Self {
        let parts: Vec<&str> = input_line.split(">, ").collect();
        let pos: Vec<isize> = parts[0]
            .split('<')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();

        let loc = Vector3Int::new(pos[0], pos[1], pos[2]);
        let rad: usize = parts[1].split('=').nth(1).unwrap().parse().unwrap();

        let man_dist = loc.x.abs() + loc.y.abs() + loc.z.abs();

        return Nanobot {
            location: loc,
            radius: rad,
            min_dist_to_zero: (man_dist - rad as isize).abs() as usize,
            max_dist_to_zero: man_dist as usize + rad,
        };
    }

    pub fn man_dist(&self, other: &Nanobot) -> usize {
        return ((self.location.x - other.location.x).abs()
            + (self.location.y - other.location.y).abs()
            + (self.location.z - other.location.z).abs()) as usize;
    }
}

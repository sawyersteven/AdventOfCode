use shared::v3i::Vector3Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display};

const POLL_RATE: usize = 200;

pub struct Day20 {
    pub input: String,
}

impl Day20 {
    pub fn new() -> Day20 {
        return Day20 {
            input: String::from(""),
        };
    }
}

impl Base for Day20 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut particles: Vec<Particle> = self
            .input
            .split('\n')
            .enumerate()
            .map(|(i, line)| Particle::new(line, i))
            .collect();

        let mut shortest_id = 0;
        for i in 0..usize::MAX {
            for p in &mut particles {
                p.tick();
            }

            if i % POLL_RATE == 0 {
                let mut shortest_dist = usize::MAX;
                let last_shortest_id = shortest_id;
                for p in &mut particles {
                    let d = p.position.manhattan(Vector3Int::zero());
                    if d < shortest_dist {
                        shortest_dist = d;
                        shortest_id = p.id;
                    }
                }
                if shortest_id == last_shortest_id {
                    break;
                }
            }
        }

        return Box::new(shortest_id);
    }

    fn part2(&self) -> Box<dyn Display> {
        let mut particles: Vec<Particle> = self
            .input
            .split('\n')
            .enumerate()
            .map(|(i, line)| Particle::new(line, i))
            .collect();

        let mut prev_particle_count = particles.len();
        for i in 0..usize::MAX {
            let mut positions = HashSet::<Vector3Int>::new();
            let mut duplicate_positions = HashSet::<Vector3Int>::new();

            for p in &particles {
                if positions.contains(&p.position) {
                    duplicate_positions.insert(p.position);
                }
                positions.insert(p.position);
            }

            particles.retain(|p| !duplicate_positions.contains(&p.position));

            if i % POLL_RATE == 0 {
                if prev_particle_count != 1000 && prev_particle_count == particles.len() {
                    break;
                }
                prev_particle_count = particles.len();
            }

            for p in &mut particles {
                p.tick();
            }
        }

        return Box::new(particles.len());
    }
}

struct Particle {
    position: Vector3Int,
    velocity: Vector3Int,
    accel: Vector3Int,
    id: usize,
}

impl Particle {
    pub fn new(line: &str, id: usize) -> Particle {
        let parts: Vec<&str> = line.split(">, ").collect();
        let nums: Vec<isize> = parts[0][3..].split(',').map(|x| x.parse().unwrap()).collect();
        let position = Vector3Int::new(nums[0], nums[1], nums[2]);

        let nums: Vec<isize> = parts[1][3..].split(',').map(|x| x.parse().unwrap()).collect();
        let velocity = Vector3Int::new(nums[0], nums[1], nums[2]);

        let nums: Vec<isize> = parts[2].replace('>', "")[3..]
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let accel = Vector3Int::new(nums[0], nums[1], nums[2]);

        return Particle {
            position,
            velocity,
            accel,
            id,
        };
    }

    pub fn tick(&mut self) {
        self.velocity += self.accel;
        self.position += self.velocity;
    }
}

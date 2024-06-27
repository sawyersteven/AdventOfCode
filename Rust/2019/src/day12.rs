use shared::v3i::Vector3Int;

use crate::{utils, Base};
use std::fmt::Display;

pub struct Day12 {
    moons: Vec<Moon>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 { moons: Vec::new() };
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n').collect::<Vec<&str>>() {
            let parts: Vec<&str> = line[1..(line.len() - 1)].split(", ").collect();
            let x = parts[0].split('=').nth(1).unwrap().parse().unwrap();
            let y = parts[1].split('=').nth(1).unwrap().parse().unwrap();
            let z = parts[2].split('=').nth(1).unwrap().parse().unwrap();
            self.moons.push(Moon::new(Vector3Int::new(x, y, z), Vector3Int::ZERO));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut moons = self.moons.clone();
        for _ in 0..1000 {
            {
                sim_step(&mut moons);
            }
        }
        let mut energy = 0;
        for i in 0..self.moons.len() {
            energy += nrg(&moons[i].position) * nrg(&moons[i].velocity);
        }
        return Box::new(energy);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut moons = self.moons.clone();
        let mut intervals = [0, 0, 0];
        let mut found = 0;

        let mut step = 1;
        while found < 3 {
            sim_step(&mut moons);
            let mut stopped = [true, true, true];

            for i in 0..moons.len() {
                stopped[0] &= moons[i].velocity.x == 0;
                stopped[1] &= moons[i].velocity.y == 0;
                stopped[2] &= moons[i].velocity.z == 0;
            }

            for i in 0..intervals.len() {
                if intervals[i] == 0 && stopped[i] {
                    intervals[i] = step * 2;
                    found += 1;
                }
            }
            step += 1;
        }
        return Box::new(utils::lcm(&intervals));
    }
}

fn nrg(v: &Vector3Int) -> isize {
    return v.x.abs() + v.y.abs() + v.z.abs();
}

fn sim_step(moons: &mut Vec<Moon>) {
    //Gravity
    for i in 0..(moons.len() - 1) {
        for j in (i + 1)..moons.len() {
            if moons[i].position.x > moons[j].position.x {
                moons[i].velocity.x -= 1;
                moons[j].velocity.x += 1;
            } else if moons[i].position.x < moons[j].position.x {
                moons[i].velocity.x += 1;
                moons[j].velocity.x -= 1;
            }

            if moons[i].position.y > moons[j].position.y {
                moons[i].velocity.y -= 1;
                moons[j].velocity.y += 1;
            } else if moons[i].position.y < moons[j].position.y {
                moons[i].velocity.y += 1;
                moons[j].velocity.y -= 1;
            }

            if moons[i].position.z > moons[j].position.z {
                moons[i].velocity.z -= 1;
                moons[j].velocity.z += 1;
            } else if moons[i].position.z < moons[j].position.z {
                moons[i].velocity.z += 1;
                moons[j].velocity.z -= 1;
            }
        }
    }

    // Velocity
    for i in 0..moons.len() {
        moons[i].position = moons[i].position + moons[i].velocity;
    }
}

#[derive(Clone, Copy)]
struct Moon {
    position: Vector3Int,
    velocity: Vector3Int,
}

impl Moon {
    pub fn new(pos: Vector3Int, vel: Vector3Int) -> Self {
        return Moon {
            position: pos,
            velocity: vel,
        };
    }
}

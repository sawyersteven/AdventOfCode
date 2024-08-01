use shared::v2i::Vector2Int;

use crate::Base;
use std::fmt::Display;

pub struct Day12 {
    nav: Vec<(u8, u16)>,
}

impl Day12 {
    pub fn new() -> Day12 {
        return Day12 { nav: Vec::new() };
    }
}

impl Base for Day12 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            let d = line.as_bytes()[0];
            let v = line[1..].parse().unwrap();
            self.nav.push((d, v));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut dir = Vector2Int::RIGHT;
        let mut pos = Vector2Int::ZERO;

        for (d, v) in &self.nav {
            let v = *v as isize;
            match d {
                b'N' => pos.y += v,
                b'S' => pos.y -= v,
                b'E' => pos.x += v,
                b'W' => pos.x -= v,
                b'L' => rot_left(&mut dir, v),
                b'R' => rot_right(&mut dir, v),
                b'F' => pos += dir * v,
                _ => unreachable!(),
            }
        }
        return Box::new(pos.manhattan(&Vector2Int::ZERO));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut pos = Vector2Int::ZERO;
        let mut waypoint = Vector2Int::new(10, 1);

        for (d, v) in &self.nav {
            let v = *v as isize;
            match d {
                b'N' => waypoint.y += v,
                b'S' => waypoint.y -= v,
                b'E' => waypoint.x += v,
                b'W' => waypoint.x -= v,
                b'L' => waypoint = rot_v2(&waypoint, -v),
                b'R' => waypoint = rot_v2(&waypoint, v),
                b'F' => pos += waypoint * v,
                _ => unreachable!(),
            }
        }
        return Box::new(pos.manhattan(&Vector2Int::ZERO));
    }
}

fn rot_right(v: &mut Vector2Int, degrees: isize) {
    for _ in 0..(degrees / 90) {
        let t = v.y;
        v.y = v.x * -1;
        v.x = t;
    }
}

fn rot_left(v: &mut Vector2Int, degrees: isize) {
    for _ in 0..(degrees / 90) {
        let t = v.x;
        v.x = v.y * -1;
        v.y = t;
    }
}

fn rot_v2(v: &Vector2Int, degrees: isize) -> Vector2Int {
    let steps = degrees / 90;
    let mut v = *v;
    if steps > 0 {
        for _ in 0..steps {
            let t = -v.x;
            v.x = v.y;
            v.y = t;
        }
    } else {
        for _ in steps..0 {
            let t = v.x;
            v.x = -v.y;
            v.y = t;
        }
    }
    return v;
}

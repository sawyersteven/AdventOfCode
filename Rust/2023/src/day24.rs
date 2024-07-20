use crate::Base;
use std::{f64::INFINITY, fmt::Display, ops::RangeInclusive};

struct F64_3D {
    x: f64,
    y: f64,
    z: f64,
}

impl F64_3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Self { x, y, z };
    }
}

pub struct Day24 {
    lines: Vec<Line>,
}

impl Day24 {
    pub fn new() -> Day24 {
        return Day24 { lines: Vec::new() };
    }
}

impl Base for Day24 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            self.lines.push(Line::from_string(line));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        const BOUNDS: RangeInclusive<f64> = 200000000000000.0..=400000000000000.0;

        let mut total = 0;
        for i in 0..(self.lines.len() - 1) {
            let la = &self.lines[i];
            for j in (i + 1)..self.lines.len() {
                let lb = &self.lines[j];

                let collision = la.intersect_2d(&lb);

                if collision.0 == INFINITY
                    || !la.point_in_future(&collision)
                    || !lb.point_in_future(&collision)
                    || !BOUNDS.contains(&collision.0)
                    || !BOUNDS.contains(&collision.1)
                {
                    continue;
                }
                total += 1;
            }
        }
        return Box::new(total);
    }

    /*
    Find origin and vel where:
    orig + (vel * tA) == lineA.orig + (lineA.vel * tA)
    orig + (vel * tB) == lineB.orig + (lineB.vel * tB)
    orig + (vel * tC) == lineC.orig + (lineC.vel * tC)

    Only need to solve for first 3 to establish line formula

    An interesting part of the input, which is probably not universal, is that
    a pair of hailstones has the same origin.y and vel.y. Because these two
    will only diverge on the y axis from this point forward, the stone we
    throw must share their origin.y and vel.y.

    Knowing that, take two other arbitrary hailstones (l_b and l_c) and calc
    the time at which their y axis collides with our stone's y axis.

    That time lets us determine l_b and l_c's complete location at the time
    of collision with the stone. Our velocity then must be the difference in
    location of these collision divided by the difference in time.

    Now we have our stone's velocity, the time of a collision, and that
    collision's location. Work back to time 0 and that's the stone's origin
    position.

    This may work on other inputs if modified to use whatever axis matches. I'm
    not going to implement that here.
    */
    fn part2(&mut self) -> Box<dyn Display> {
        for i in 0..(self.lines.len() - 1) {
            for j in (i + 1)..self.lines.len() {
                if self.lines[i].origin.y == self.lines[j].origin.y {
                    let l_a = &self.lines[j];
                    // vv not safe, but safe for my input vv
                    let l_b = &self.lines[j + 1];
                    let l_c = &self.lines[j + 2];

                    let t_b = (l_b.origin.y - l_a.origin.y) / (l_a.vel.y - l_b.vel.y);
                    let t_c = (l_c.origin.y - l_a.origin.y) / (l_a.vel.y - l_c.vel.y);

                    let collision_b = l_b.at_time(t_b);
                    let collision_c = l_c.at_time(t_c);

                    let v = F64_3D::new(
                        (collision_c.x - collision_b.x) / (t_c - t_b),
                        (collision_c.y - collision_b.y) / (t_c - t_b),
                        (collision_c.z - collision_b.z) / (t_c - t_b),
                    );

                    let p = F64_3D::new(
                        collision_b.x - v.x * t_b,
                        collision_b.y - v.y * t_b,
                        collision_b.z - v.z * t_b,
                    );

                    return Box::new(p.x + p.y + p.z);
                }
            }
        }

        return Box::new("-");
    }
}

struct Line {
    origin: F64_3D,
    vel: F64_3D,
    slope: f64,
}

impl Line {
    fn from_string(string: &str) -> Self {
        let digits = string
            .split(|x| x == ',' || x == '@')
            .map(|x| x.trim().parse().unwrap())
            .collect::<Vec<f64>>();

        return Self {
            origin: F64_3D::new(digits[0], digits[1], digits[2]),
            vel: F64_3D::new(digits[3], digits[4], digits[5]),
            slope: digits[4] / digits[3],
        };
    }

    fn at_time(&self, time: f64) -> F64_3D {
        return F64_3D::new(
            self.origin.x + (self.vel.x * time),
            self.origin.y + (self.vel.y * time),
            self.origin.z + (self.vel.z * time),
        );
    }

    fn point_in_future(&self, p: &(f64, f64)) -> bool {
        if self.vel.x.signum() == -1.0 {
            if p.0 > self.origin.x {
                return false;
            }
        } else if p.0 < self.origin.x {
            return false;
        }

        if self.vel.y.signum() == -1.0 {
            if p.1 > self.origin.y {
                return false;
            }
        } else if p.1 < self.origin.y {
            return false;
        }

        return true;
    }

    fn intersect_2d(&self, other: &Line) -> (f64, f64) {
        // m1(x - x1) + y1 = m2(x - x2) + y2
        let x = (other.origin.y - (other.slope * other.origin.x) + (self.slope * self.origin.x) - self.origin.y)
            / (self.slope - other.slope);

        let y = self.slope * (x - self.origin.x) + self.origin.y;

        return (x, y);
    }
}

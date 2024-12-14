use shared::{utils::read_user_input, v2i::Vector2Int};

use crate::Base;
use std::{fmt::Display, usize};

const W: isize = 101;
const H: isize = 103;
const SECONDS: isize = 100;

pub struct Day14 {
    input: String,
}

impl Day14 {
    pub fn new() -> Day14 {
        return Day14 { input: String::new() };
    }
}

impl Base for Day14 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mh = H / 2;
        let mw = W / 2;

        let mut quads = [0; 4];
        let quad_ranges = [
            (Vector2Int::new(0, 0), Vector2Int::new(mw - 1, mh - 1)), // nw
            (Vector2Int::new(mw + 1, 0), Vector2Int::new(W, mh - 1)), // ne
            (Vector2Int::new(0, mh + 1), Vector2Int::new(mw - 1, H)), // sw
            (Vector2Int::new(mw + 1, mh + 1), Vector2Int::new(W, H)), // se
        ];

        for line in self.input.lines() {
            let mut bot = Bot::from(line);
            bot.pos += bot.vel * SECONDS;
            bot.pos.x = bot.pos.x.rem_euclid(W);
            bot.pos.y = bot.pos.y.rem_euclid(H);
            if bot.pos.y == mh || bot.pos.x == mw {
                continue;
            }
            for i in 0..4 {
                if bot.pos.in_range(&quad_ranges[i].0, &quad_ranges[i].1) {
                    quads[i] += 1;
                    break;
                }
            }
        }

        return Box::new(quads.iter().product::<isize>());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let runtime = W * H;
        let mut bots: Vec<Bot> = self.input.lines().map(|l| Bot::from(l)).collect();

        let mut min_entropy = usize::MAX;
        let mut min_entropy_second = 0;
        for second in 1..=runtime {
            for i in 0..bots.len() {
                bots[i].pos.x = (bots[i].pos.x + bots[i].vel.x).rem_euclid(W);
                bots[i].pos.y = (bots[i].pos.y + bots[i].vel.y).rem_euclid(H);
            }

            let mut entropy = 0;
            for bot in &bots {
                if bot.pos.x > W / 2 {
                    continue;
                }
                let mut pos = bot.pos;
                pos.x = W - pos.x;
                if !bots.iter().any(|b| b.pos == pos) {
                    entropy += 1;
                }
            }

            if entropy < min_entropy {
                min_entropy = entropy;
                min_entropy_second = second;
            }
        }

        return Box::new(min_entropy_second);
    }
}

struct Bot {
    pos: Vector2Int,
    vel: Vector2Int,
}

impl From<&str> for Bot {
    fn from(value: &str) -> Self {
        let (p, v) = value.split_once(' ').unwrap();
        let (px, py) = p[2..].split_once(',').unwrap();
        let (vx, vy) = v[2..].split_once(',').unwrap();

        return Self {
            pos: Vector2Int::new(px.parse().unwrap(), py.parse().unwrap()),
            vel: Vector2Int::new(vx.parse().unwrap(), vy.parse().unwrap()),
        };
    }
}

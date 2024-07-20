use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashSet, fmt::Display, ops::RangeInclusive};

pub struct Day21 {
    rocks: HashSet<Vector2Int>,
    start: Vector2Int,
    input: String,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            rocks: HashSet::new(),
            start: Vector2Int::ZERO,
            input: String::new(),
        };
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        // Make map of rocks 5x the original size
        const EXPAND_BY: RangeInclusive<isize> = -2..=2 as isize;

        let tile_w = raw_input.lines().nth(0).unwrap().len() as isize;
        let tile_h = raw_input.lines().count() as isize;

        for (y, line) in raw_input.lines().enumerate() {
            for (x, b) in line.as_bytes().iter().enumerate() {
                match b {
                    b'#' => {
                        let og_rock = Vector2Int::new(x as isize, y as isize);
                        self.rocks.insert(og_rock);
                        for r in EXPAND_BY {
                            for c in EXPAND_BY {
                                let ex_rock = Vector2Int::new(og_rock.x + (tile_w * c), og_rock.y + (tile_h * r));
                                self.rocks.insert(ex_rock);
                            }
                        }
                    }
                    b'S' => {
                        self.start = Vector2Int::new(x as isize, y as isize);
                    }
                    _ => {}
                }
            }
        }
        self.input = raw_input.to_string();
    }

    // Simple brute force
    fn part1(&mut self) -> Box<dyn Display> {
        let mut ends = HashSet::new();
        ends.insert(self.start.clone());

        let mut moves = HashSet::new();

        for _ in 0..64 {
            for pos in &ends {
                for dir in Vector2Int::CARDINALS {
                    moves.insert(*pos + dir);
                }
            }
            ends = moves.drain().filter(|v| !self.rocks.contains(v)).collect();
        }
        return Box::new(ends.len());
    }

    /*
    This puzzle reminds me of the orcs/elves battle simulation from several years ago.
    I went through that for several days trying to get the right answer before I gave
    in and looked online for a solution. I tried a dozen solutions before I found one
    that actually gave the right answer. Apparently the solution for my input needed
    to be made specifically for that input.

    This took fewer attempts at other people's solutions to find one that worked, but
    it still seems like the solution must be made specifically for my input file.
    Which means a general-purpose solution may not exist. Which is irritating.

    So I did not devise this solution on my own, and I don't remember where/who I
    copied it from. But it has been heavily modified by me to fit my style and
    helper structs. I've left an original comment pointing to a 2nd solution origin
    for the main formula.

    The solution I copied took ~45 seconds in debug mode while this takes ~28.
    Or 2 seconds in release mode.

    So there's that.
    */
    fn part2(&mut self) -> Box<dyn Display> {
        let mut aa = HashSet::new();
        aa.insert(self.start);
        let mut bb = HashSet::new();

        // Get valid endpoints at:
        // 65 (end of first map tile)
        // 65 + 131 (end of +- second tile)
        // 65 + (131 * 2) (end of +- third tile)
        let mut b = (0, 0, 0);
        for i in 1..=(65 + (2 * 131)) {
            for pos in aa.iter().clone() {
                for n in Vector2Int::CARDINALS {
                    if !self.rocks.contains(&(*pos + n)) {
                        bb.insert(*pos + n);
                    }
                }
            }
            if i == 65 {
                b.0 = bb.len() as isize;
            } else if i == 65 + 131 {
                b.1 = bb.len() as isize;
            }
            (aa, bb) = (bb, aa);
        }
        b.2 = aa.len() as isize;

        // the following formula comes from inv(A) * B = X,
        // where A is Vandermonde matrix:
        // [ 0 0 1 ]
        // [ 1 1 1 ]
        // [ 4 2 1 ]
        // and B is a column vector from the above values b0, b1, b2
        // credit to: https://gist.github.com/dllu/0ca7bfbd10a199f69bcec92f067ec94c
        // below uses Cramer's Rule to solve for x0, x1, x2
        let n = 202300;
        let det_a = -2;
        let det_a0 = -b.0 + 2 * b.1 - b.2;
        let det_a1 = 3 * b.0 - 4 * b.1 + b.2;
        let det_a2 = -2 * b.0;
        let x0 = det_a0 / det_a;
        let x1 = det_a1 / det_a;
        let x2 = det_a2 / det_a;
        let ans = x0 * n * n + x1 * n + x2;
        assert_eq!(597102953699891, ans);
        return Box::new(ans);
    }
}

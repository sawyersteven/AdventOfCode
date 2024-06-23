use shared::grid2d::Grid2D;

use crate::Base;
use std::fmt::Display;

pub struct Day13 {
    maps: Vec<Grid2D<u8>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 { maps: Vec::new() };
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        for map in raw_input.split("\n\n") {
            self.maps.push(Grid2D::<u8>::from_string(&map.to_string()));
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for m in &self.maps {
            total += find_v_mirror_line(&m);
            total += find_h_mirror_line(&m) * 100;
        }
        return Box::new(total);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut total = 0;
        for m in &self.maps {
            total += find_one_off_v_mirror_line(&m);
            total += find_one_off_h_mirror_line(&m) * 100;
        }

        return Box::new(total);
    }
}

// Returns edge of mirror line (indexed from 1)
fn find_v_mirror_line(map: &Grid2D<u8>) -> usize {
    let sz = map.size();

    for col in 0..(sz.x - 1) {
        let mut a = col;
        let mut b = col + 1;
        loop {
            let is_mirrored = (0..sz.y).all(|y| map.get(a, y) == map.get(b, y));
            if !is_mirrored {
                break;
            }
            if a == 0 || b == sz.x - 1 {
                return col + 1;
            }
            a -= 1;
            b += 1;
        }
    }
    return 0;
}

// Returns edge of mirror line (indexed from 1), where only 1 cell is different
fn find_one_off_v_mirror_line(map: &Grid2D<u8>) -> usize {
    let sz = map.size();

    for col in 0..(sz.x - 1) {
        let mut a = col;
        let mut b = col + 1;
        let mut diffs = 0;
        loop {
            for y in 0..sz.y {
                if map.get(a, y) != map.get(b, y) {
                    diffs += 1;
                }
            }
            if diffs > 1 || a == 0 || b == sz.x - 1 {
                break;
            }

            a -= 1;
            b += 1;
        }
        if diffs == 1 {
            return col + 1;
        }
    }
    return 0;
}

// Returns edge of mirror line (indexed from 1), where only 1 cell is different
fn find_one_off_h_mirror_line(map: &Grid2D<u8>) -> usize {
    let sz = map.size();

    for row in 0..(sz.y - 1) {
        let mut a = row;
        let mut b = row + 1;
        let mut diffs = 0;
        loop {
            for x in 0..sz.x {
                if map.get(x, a) != map.get(x, b) {
                    diffs += 1;
                }
            }
            if diffs > 1 || a == 0 || b == sz.y - 1 {
                break;
            }

            a -= 1;
            b += 1;
        }

        if diffs == 1 {
            return row + 1;
        }
    }
    return 0;
}

// Returns edge of mirror line (indexed from 1)
fn find_h_mirror_line(map: &Grid2D<u8>) -> usize {
    let sz = map.size();

    for row in 0..(sz.y - 1) {
        let mut a = row;
        let mut b = row + 1;
        loop {
            let is_mirrored = (0..sz.x).all(|x| map.get(x, a) == map.get(x, b));
            if !is_mirrored {
                break;
            }
            if a == 0 || b == sz.y - 1 {
                return row + 1;
            }
            a -= 1;
            b += 1;
        }
    }
    return 0;
}

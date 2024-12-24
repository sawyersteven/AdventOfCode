use shared::{bitset::Bitset2D, v2i::Vector2Int};

use crate::Base;
use std::fmt::Display;

pub struct Day15 {
    input: String,
}

impl Day15 {
    pub fn new() -> Day15 {
        return Day15 { input: String::new() };
    }
}

impl Base for Day15 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let (map, path) = self.input.split_once("\n\n").unwrap();
        let sz = (map.len() as f64).sqrt() as isize;

        let mut walls = Bitset2D::new(sz as usize, sz as usize);
        let mut boxes = Bitset2D::new(sz as usize, sz as usize);
        let mut bot = Vector2Int::new(0, 0);

        for (y, line) in map.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => walls.set(x, y),
                    b'O' => boxes.set(x, y),
                    b'@' => bot = Vector2Int::new(x as isize, y as isize),
                    _ => {}
                }
            }
        }

        for m in path.bytes() {
            let dir = match m {
                b'^' => Vector2Int::new(0, -1),
                b'>' => Vector2Int::new(1, 0),
                b'v' => Vector2Int::new(0, 1),
                b'<' => Vector2Int::new(-1, 0),
                b'\n' => continue,
                _ => unreachable!(),
            };
            let move_to = bot + dir;
            if walls.is_set(move_to.x as usize, move_to.y as usize) {
                continue;
            }
            if !boxes.is_set(move_to.x as usize, move_to.y as usize) {
                bot = move_to;
                continue;
            }

            // find closest open space
            let mut gap_dist = 1;
            loop {
                let step = bot + (dir * gap_dist);
                if walls.is_set(step.x as usize, step.y as usize) {
                    gap_dist = 0;
                    break;
                }
                if !boxes.is_set(step.x as usize, step.y as usize) {
                    break;
                }
                gap_dist += 1;
            }
            if gap_dist == 0 {
                continue;
            }
            // swap first and last box
            let f = bot + (dir * gap_dist);
            boxes.set(f.x as usize, f.y as usize);
            boxes.unset(move_to.x as usize, move_to.y as usize);
            bot = move_to;
        }

        let mut sum = 0;
        for y in 1..(sz - 1) as usize {
            for x in 1..(sz - 1) as usize {
                if boxes.is_set(x, y) {
                    sum += (y * 100) + x;
                }
            }
        }

        return Box::new(sum);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let (map, path) = self.input.split_once("\n\n").unwrap();
        let sz = (map.len() as f64).sqrt() as usize;

        let mut box_lefts = Bitset2D::new(sz * 2, sz);
        let mut box_rights = Bitset2D::new(sz * 2, sz);
        let mut walls = Bitset2D::new(sz * 2, sz);
        let mut bot = Vector2Int::ZERO;

        for (y, line) in map.lines().enumerate() {
            for (x, b) in line.bytes().enumerate() {
                match b {
                    b'#' => {
                        walls.set(x * 2, y);
                        walls.set((x * 2) + 1, y)
                    }
                    b'O' => {
                        box_lefts.set(x * 2, y);
                        box_rights.set((x * 2) + 1, y);
                    }
                    b'@' => bot = Vector2Int::new(x as isize * 2, y as isize),
                    _ => continue,
                }
            }
        }

        for m in path.bytes() {
            let dir = match m {
                b'^' => Vector2Int::DOWN,
                b'>' => Vector2Int::RIGHT,
                b'v' => Vector2Int::UP,
                b'<' => Vector2Int::LEFT,
                b'\n' => continue,
                _ => unreachable!(),
            };
            let move_to = bot + dir;
            if walls.is_setv(move_to) {
                continue;
            }

            if dir == Vector2Int::RIGHT {
                if !box_lefts.is_setv(move_to) {
                    bot = move_to;
                } else {
                    if try_move_box_right(move_to, &mut box_lefts, &mut box_rights, &walls) {
                        bot = move_to;
                    }
                }
            } else if dir == Vector2Int::LEFT {
                if !box_rights.is_setv(move_to) {
                    bot = move_to;
                } else {
                    if try_move_box_left(move_to, &mut box_lefts, &mut box_rights, &walls) {
                        bot = move_to;
                    }
                }
            } else if dir == Vector2Int::DOWN {
                let under_left = box_lefts.is_setv(move_to);
                let under_right = box_rights.is_setv(move_to);
                if !under_left && !under_right {
                    bot = move_to;
                } else {
                    let bre = if under_left {
                        move_to + Vector2Int::RIGHT
                    } else {
                        move_to
                    };
                    // move up because up/down are inverted
                    if try_move_box_up(bre, &mut box_lefts, &mut box_rights, &walls, false) {
                        bot = move_to;
                    }
                }
            } else if dir == Vector2Int::UP {
                let above_left = box_lefts.is_setv(move_to);
                let above_right = box_rights.is_setv(move_to);
                if !above_left && !above_right {
                    bot = move_to;
                } else {
                    let tre = if above_left {
                        move_to + Vector2Int::RIGHT
                    } else {
                        move_to
                    };
                    // move up because up/down are inverted
                    if try_move_box_down(tre, &mut box_lefts, &mut box_rights, &walls, false) {
                        bot = move_to;
                    }
                }
            } else {
                unreachable!();
            }
        }

        let mut score = 0;
        for bl in box_lefts.into_iter() {
            score += (bl.1 * 100) + bl.0;
        }

        return Box::new(score);
    }
}

// tries to move a box to the right one unit. Returns bool indicating if move was successful
fn try_move_box_right(
    left_edge: Vector2Int,
    box_lefts: &mut Bitset2D,
    box_rights: &mut Bitset2D,
    walls: &Bitset2D,
) -> bool {
    let req_empty_space = left_edge + (Vector2Int::RIGHT * 2);
    let can_move: bool;
    if box_lefts.is_setv(req_empty_space) {
        can_move = try_move_box_right(req_empty_space, box_lefts, box_rights, walls);
    } else {
        can_move = !walls.is_setv(req_empty_space);
    }

    if can_move {
        box_lefts.unsetv(left_edge);
        box_lefts.setv(left_edge + Vector2Int::RIGHT);
        box_rights.unsetv(left_edge + Vector2Int::RIGHT);
        box_rights.setv(req_empty_space);
        return true;
    }

    return false;
}

fn try_move_box_left(
    right_edge: Vector2Int,
    box_lefts: &mut Bitset2D,
    box_rights: &mut Bitset2D,
    walls: &Bitset2D,
) -> bool {
    let req_empty_space = right_edge + (Vector2Int::LEFT * 2);
    let can_move: bool;
    if box_rights.is_setv(req_empty_space) {
        can_move = try_move_box_left(req_empty_space, box_lefts, box_rights, walls);
    } else {
        can_move = !walls.is_setv(req_empty_space);
    }

    if can_move {
        box_rights.unsetv(right_edge);
        box_rights.setv(right_edge + Vector2Int::LEFT);
        box_lefts.unsetv(right_edge + (Vector2Int::LEFT));
        box_lefts.setv(req_empty_space);
        return true;
    }

    return false;
}

/// probe only tests if move is possible without actually moving anything
fn try_move_box_up(
    right_edge: Vector2Int,
    box_lefts: &mut Bitset2D,
    box_rights: &mut Bitset2D,
    walls: &Bitset2D,
    probe: bool,
) -> bool {
    let req_empty_space_r = right_edge + Vector2Int::DOWN;
    let req_empty_space_l = right_edge + Vector2Int::DOWN + Vector2Int::LEFT;
    let can_move: bool;

    // directly under another box if R edge is under R edge eg:
    // ......
    // ..[]..
    // ..[]..
    // ..@...
    if walls.is_setv(req_empty_space_l) || walls.is_setv(req_empty_space_r) {
        return false;
    }
    if box_rights.is_setv(req_empty_space_r) {
        can_move = try_move_box_up(right_edge + Vector2Int::DOWN, box_lefts, box_rights, walls, probe);
    } else {
        // half under another box to the right if R edge is under L edge eg:
        // ......
        // ...[].
        // ..[]..
        // ..@...
        let half_under_right = box_lefts.is_setv(req_empty_space_r);

        // half under another box to the left if L edge is under R edge eg:
        // ......
        // .[]...
        // ..[]..
        // ..@...
        let half_under_left = box_rights.is_setv(req_empty_space_l);

        if !half_under_left && !half_under_right {
            // under nothing
            can_move = true;
        } else if half_under_right && !half_under_left {
            // if only under one box, try to move it
            can_move = try_move_box_up(
                right_edge + Vector2Int::DOWN + Vector2Int::RIGHT,
                box_lefts,
                box_rights,
                walls,
                probe,
            );
        } else if half_under_left && !half_under_right {
            // if only under one box, try to move it
            can_move = try_move_box_up(
                right_edge + Vector2Int::DOWN + Vector2Int::LEFT,
                box_lefts,
                box_rights,
                walls,
                probe,
            );
        } else {
            // if under both, we need to know if both can move before either can move, so change to probe
            let can_move_l = try_move_box_up(
                right_edge + Vector2Int::DOWN + Vector2Int::LEFT,
                box_lefts,
                box_rights,
                walls,
                true,
            );
            let can_move_r = try_move_box_up(
                right_edge + Vector2Int::DOWN + Vector2Int::RIGHT,
                box_lefts,
                box_rights,
                walls,
                true,
            );

            can_move = can_move_l && can_move_r;
            if can_move && !probe {
                // go back and actually do the move
                try_move_box_up(
                    right_edge + Vector2Int::DOWN + Vector2Int::LEFT,
                    box_lefts,
                    box_rights,
                    walls,
                    probe,
                );
                try_move_box_up(
                    right_edge + Vector2Int::DOWN + Vector2Int::RIGHT,
                    box_lefts,
                    box_rights,
                    walls,
                    probe,
                );
            }
        }
    }

    if !probe && can_move {
        box_rights.unsetv(right_edge);
        box_rights.setv(req_empty_space_r);
        box_lefts.unsetv(right_edge + Vector2Int::LEFT);
        box_lefts.setv(req_empty_space_l);
    }

    return can_move;
}

/// probe only tests if move is possible without actually moving anything
fn try_move_box_down(
    right_edge: Vector2Int,
    box_lefts: &mut Bitset2D,
    box_rights: &mut Bitset2D,
    walls: &Bitset2D,
    probe: bool,
) -> bool {
    let req_empty_space_r = right_edge + Vector2Int::UP;
    let req_empty_space_l = right_edge + Vector2Int::UP + Vector2Int::LEFT;
    let can_move: bool;

    // directly above another box if R edge is above R edge eg:
    // ..@...
    // ..[]..
    // ..[]..
    // ......
    if walls.is_setv(req_empty_space_l) || walls.is_setv(req_empty_space_r) {
        return false;
    }
    if box_rights.is_setv(req_empty_space_r) {
        can_move = try_move_box_down(right_edge + Vector2Int::UP, box_lefts, box_rights, walls, probe);
    } else {
        // half above another box to the right if R edge is above L edge eg:
        // ...@..
        // ..[]..
        // ...[].
        // ......
        let half_over_right = box_lefts.is_setv(req_empty_space_r);

        // half over another box to the left if L edge is over R edge eg:
        // ...@..
        // ..[]..
        // .[]...
        // ......
        let half_above_left = box_rights.is_setv(req_empty_space_l);

        if !half_above_left && !half_over_right {
            // over nothing
            can_move = true;
        } else if half_over_right && !half_above_left {
            // if only over one box, try to move it
            can_move = try_move_box_down(
                right_edge + Vector2Int::UP + Vector2Int::RIGHT,
                box_lefts,
                box_rights,
                walls,
                probe,
            );
        } else if half_above_left && !half_over_right {
            // if only over one box, try to move it
            can_move = try_move_box_down(
                right_edge + Vector2Int::UP + Vector2Int::LEFT,
                box_lefts,
                box_rights,
                walls,
                probe,
            );
        } else {
            // if under both, we need to know if both can move before either can move
            let can_move_l = try_move_box_down(
                right_edge + Vector2Int::UP + Vector2Int::LEFT,
                box_lefts,
                box_rights,
                walls,
                true,
            );
            let can_move_r = try_move_box_down(
                right_edge + Vector2Int::UP + Vector2Int::RIGHT,
                box_lefts,
                box_rights,
                walls,
                true,
            );

            can_move = can_move_l && can_move_r;

            if can_move && !probe {
                // go back and actually do the move
                try_move_box_down(
                    right_edge + Vector2Int::UP + Vector2Int::LEFT,
                    box_lefts,
                    box_rights,
                    walls,
                    probe,
                );
                try_move_box_down(
                    right_edge + Vector2Int::UP + Vector2Int::RIGHT,
                    box_lefts,
                    box_rights,
                    walls,
                    probe,
                );
            }
        }
    }

    if !probe && can_move {
        box_rights.unsetv(right_edge);
        box_rights.setv(req_empty_space_r);
        box_lefts.unsetv(right_edge + Vector2Int::LEFT);
        box_lefts.setv(req_empty_space_l);
    }

    return can_move;
}

trait B2DV {
    fn is_setv(&self, v: Vector2Int) -> bool;
    fn setv(&mut self, v: Vector2Int);
    fn unsetv(&mut self, v: Vector2Int);
}

impl B2DV for Bitset2D {
    fn is_setv(&self, v: Vector2Int) -> bool {
        return self.is_set(v.x as usize, v.y as usize);
    }

    fn setv(&mut self, v: Vector2Int) {
        self.set(v.x as usize, v.y as usize);
    }

    fn unsetv(&mut self, v: Vector2Int) {
        self.unset(v.x as usize, v.y as usize);
    }
}

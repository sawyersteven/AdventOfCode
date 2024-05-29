use shared::v2i::Vector2Int;

use crate::Base;
use std::{cmp::Ordering, collections::HashSet, fmt::Display};

const CART_CHARS: [u8; 4] = [b'^', b'>', b'v', b'<'];

pub struct Day13 {
    cart_positions: HashSet<Vector2Int>,
    carts: Vec<Cart>,
    grid: Vec<Vec<u8>>,
}

impl Day13 {
    pub fn new() -> Day13 {
        return Day13 {
            cart_positions: HashSet::new(),
            carts: Vec::new(),
            grid: Vec::new(),
        };
    }
}

impl Base for Day13 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();
        self.grid = vec![vec![b' '; lines[0].len()]; lines.len()];
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                let tile = lines[y].as_bytes()[x];
                self.grid[y][x] = tile;
                let cart_type = CART_CHARS.iter().position(|x| *x == tile);
                match cart_type {
                    Some(c) => {
                        self.grid[y][x] = if c % 2 == 1 { b'-' } else { b'|' };
                        let cart = Cart::new(
                            Vector2Int {
                                x: x as isize,
                                y: y as isize,
                            },
                            c as isize,
                        );
                        self.cart_positions.insert(cart.position);
                        self.carts.push(cart);
                    }
                    None => {}
                }
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut carts = self.carts.clone();
        for _ in 0..usize::MAX {
            carts.sort_by(cart_sort);
            for i in 0..carts.len() {
                carts[i].move_step(&self.grid);
                for j in 0..carts.len() {
                    if i == j {
                        continue;
                    }
                    if carts[j].position == carts[i].position {
                        return Box::new(carts[j].position);
                    }
                }
            }
        }

        return Box::new("-");
    }

    fn part2(&mut self) -> Box<dyn Display> {
        loop {
            self.carts.retain(|x| !x.has_crashed);
            if self.carts.len() == 1 {
                break;
            }

            self.carts.sort_by(cart_sort);
            for i in 0..self.carts.len() {
                if self.carts[i].has_crashed {
                    continue;
                }
                self.carts[i].move_step(&self.grid);
                for j in 0..self.carts.len() {
                    if i == j {
                        continue;
                    }
                    if self.carts[j].position == self.carts[i].position {
                        self.carts[j].has_crashed = true;
                        self.carts[i].has_crashed = true;
                    }
                }
            }
        }
        return Box::new(self.carts[0].position);
    }
}

fn cart_sort(a: &Cart, b: &Cart) -> Ordering {
    if a.position.y > b.position.y {
        return Ordering::Greater;
    } else if a.position.y < b.position.y {
        return Ordering::Less;
    } else {
        return if a.position.x > b.position.x {
            Ordering::Greater
        } else {
            Ordering::Less
        };
    }
}

#[derive(Clone, Copy)]
struct Cart {
    position: Vector2Int,
    direction: isize,
    next_turn: isize,
    has_crashed: bool,
}

impl Cart {
    const TURNS: [isize; 3] = [-1, 0, 1];
    const DIRECTIONS: [Vector2Int; 4] = [Vector2Int::DOWN, Vector2Int::RIGHT, Vector2Int::UP, Vector2Int::LEFT];

    pub fn new(position: Vector2Int, direction: isize) -> Self {
        return Cart {
            position: position,
            direction: direction,
            next_turn: 0,
            has_crashed: false,
        };
    }

    pub fn move_step(&mut self, grid: &Vec<Vec<u8>>) {
        self.position += Cart::DIRECTIONS[self.direction as usize];
        let track = grid[self.position.y as usize][self.position.x as usize];
        match track {
            b'/' => {
                self.rotate(if self.direction % 2 == 0 { 1 } else { 3 });
            }
            b'\\' => {
                self.rotate(if self.direction % 2 == 0 { 3 } else { 1 });
            }
            b'+' => {
                self.direction += Cart::TURNS[self.next_turn as usize];
                self.next_turn += 1;
                self.next_turn %= Cart::TURNS.len() as isize;
            }
            _ => {}
        }
        self.direction = (self.direction + 4) % 4;
    }

    fn rotate(&mut self, amount: isize) {
        self.direction += amount;
        self.direction %= 4;
    }
}

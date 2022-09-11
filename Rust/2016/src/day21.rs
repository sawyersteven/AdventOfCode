use crate::Base;
use std::fmt::Display;

pub struct Day21 {
    pub input: Vec<String>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 { input: Vec::new() };
    }

    fn scramble(&self, password: &str, reverse: bool) -> Vec<u8> {
        let mut password = password.as_bytes().to_vec();

        let input = match reverse {
            true => {
                let mut m = self.input.clone();
                m.reverse();
                m
            }
            false => self.input.clone(),
        };

        for line in input {
            let parts: Vec<&str> = line.split(' ').collect();

            match parts[0] {
                "swap" => {
                    let x: usize;
                    let y: usize;
                    if parts[1] == "position" {
                        x = parts[2].parse().unwrap();
                        y = parts[5].parse().unwrap();
                    } else {
                        x = password
                            .iter()
                            .position(|x| *x == parts[2].bytes().nth(0).unwrap())
                            .unwrap();
                        y = password
                            .iter()
                            .position(|x| *x == parts[5].bytes().nth(0).unwrap())
                            .unwrap();
                    }
                    password.swap(x, y);
                }
                "reverse" => {
                    let start: usize = parts[2].parse().unwrap();
                    let end: usize = parts[4].parse().unwrap();
                    password[start..=end].reverse();
                }
                "rotate" => {
                    let mut by: isize;
                    if parts[1] == "based" {
                        by = password
                            .iter()
                            .position(|x| *x == parts[6].bytes().nth(0).unwrap())
                            .unwrap() as isize;
                        if reverse {
                            if by % 2 == 1 {
                                by -= by / 2;
                            } else {
                                by = 5 + (by / 2);
                            }
                            if by == 5 {
                                by = 9;
                            }
                            by *= -1;
                        } else {
                            if by >= 4 {
                                by += 1;
                            }
                            by += 1;
                        }
                    } else {
                        by = parts[2].parse::<isize>().unwrap() * if parts[1] == "left" { -1 } else { 1 };
                        if reverse {
                            by *= -1;
                        }
                    }
                    by %= password.len() as isize;
                    if by > 0 {
                        password.rotate_right(by as usize);
                    } else {
                        password.rotate_left((by * -1) as usize);
                    }
                }
                "move" => {
                    let mut from: usize = parts[2].parse().unwrap();
                    let mut to: usize = parts[5].parse().unwrap();

                    if reverse {
                        (from, to) = (to, from);
                    }

                    if to > from {
                        password[from..=to].rotate_left(1);
                    } else {
                        password[to..=from].rotate_right(1);
                    }
                }

                _ => panic!(),
            }
        }
        return password;
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let pwd = Self::scramble(&self, "abcdefgh", false);
        return Box::new(String::from_iter(pwd.iter().map(|x| *x as char)));
    }

    fn part2(&self) -> Box<dyn Display> {
        let pwd = Self::scramble(&self, "fbgdceah", true);
        return Box::new(String::from_iter(pwd.iter().map(|x| *x as char)));
    }
}

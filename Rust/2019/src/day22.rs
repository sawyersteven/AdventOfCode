use num::{traits::Euclid, BigInt};

use crate::Base;
use std::fmt::Display;

const DECK_SIZE: usize = 10_007;
const DECK_SIZE2: usize = 119315717514047;

enum Technique {
    Deal,
    DealN,
    CutN,
}

pub struct Day22 {
    input: Vec<(Technique, isize)>,
}

impl Day22 {
    pub fn new() -> Day22 {
        return Day22 { input: Vec::new() };
    }
}

impl Base for Day22 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.lines() {
            if line.starts_with('c') {
                self.input
                    .push((Technique::CutN, line.split(' ').last().unwrap().parse().unwrap()));
            } else if *line.as_bytes().iter().last().unwrap() == b'k' {
                self.input.push((Technique::Deal, 0));
            } else {
                self.input
                    .push((Technique::DealN, line.split(' ').last().unwrap().parse().unwrap()));
            }
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut deck: Vec<usize> = (0..DECK_SIZE).collect();
        let mut scratch_deck = vec![0, DECK_SIZE];

        for (t, arg) in &self.input {
            match t {
                Technique::Deal => deck.reverse(),
                Technique::DealN => {
                    scratch_deck = deck.clone();
                    for i in 0..deck.len() {
                        deck[(i * *arg as usize) % DECK_SIZE] = scratch_deck[i];
                    }
                }
                Technique::CutN => {
                    if *arg > 0 {
                        let arg = *arg as usize;
                        scratch_deck = deck.clone();
                        for i in 0..(deck.len() - arg) {
                            deck[i] = deck[i + arg];
                        }
                        for i in 1..=arg {
                            deck[DECK_SIZE - i] = scratch_deck[arg - i];
                        }
                    } else {
                        let arg = arg.abs() as usize;

                        arr_cpy(&deck, deck.len() - arg, &mut scratch_deck, 0, arg);
                        for i in (arg..(deck.len() - 1)).rev() {
                            deck[i] = deck[i - arg];
                        }

                        for i in 0..arg {
                            deck[i] = scratch_deck[i];
                        }
                    }
                }
            }
        }

        let ind = deck.iter().position(|x| *x == 2019).unwrap();
        return Box::new(ind);
    }

    /// This is more of an advanced math problem than a programming problem.
    /// I did not create this solution, I merely copied it from a python repo
    /// that I can't remember now. The math is so far over my head that I'm
    /// not going to try and explain it.
    fn part2(&mut self) -> Box<dyn Display> {
        const ITERS: usize = 101741582076661;
        #[allow(non_snake_case)]
        let DECK_SZ_2 = BigInt::from(DECK_SIZE2);

        let card_index = BigInt::from(2020);

        let mut a = 1;
        let mut b = 0;

        // convert the procedures into linear algebra
        for (t, arg) in &self.input {
            let la: isize;
            let lb: isize;
            match t {
                Technique::Deal => {
                    la = -1;
                    lb = -1;
                }
                Technique::DealN => {
                    la = *arg;
                    lb = 0;
                }
                Technique::CutN => {
                    la = 1;
                    lb = -arg;
                }
            }
            a = (la * a).rem_euclid(DECK_SIZE2 as isize);
            b = (la * b + lb).rem_euclid(DECK_SIZE2 as isize);
        }

        let ma = BigInt::from(a).modpow(&BigInt::from(ITERS), &DECK_SZ_2);

        let mb = (b * (&ma - 1)) * BigInt::from(a - 1).modinv(&DECK_SZ_2).unwrap().rem_euclid(&DECK_SZ_2);

        let ans = BigInt::from((card_index - mb) * ma.modinv(&DECK_SZ_2).unwrap()).rem_euclid(&DECK_SZ_2);
        //return RealMod(((cardIndex - Mb) * InvMod(Ma, deckSize2)), deckSize2);
        return Box::new(ans);
    }
}

fn arr_cpy(src: &[usize], src_ind: usize, dst: &mut [usize], dst_ind: usize, len: usize) {
    for i in 0..len {
        dst[i + dst_ind] = src[i + src_ind];
    }
}

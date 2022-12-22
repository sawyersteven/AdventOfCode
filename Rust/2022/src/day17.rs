use crate::Base;
use std::{collections::HashSet, fmt::Display};

#[derive(Clone)]
struct State {
    rock_ind: usize,
    jet_ind: usize,
    peak: isize,
    occupied: HashSet<(isize, isize)>,
    iter_count: usize,
}

pub struct Day17 {
    jets: Vec<u8>,
    rocks: [Vec<(isize, isize)>; 5],
}

const WIDTH: isize = 7;

impl Day17 {
    pub fn new() -> Day17 {
        let rocks = [
            vec![(2, 0), (3, 0), (4, 0), (5, 0)],         // -
            vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)], // +
            vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)], // _|
            vec![(2, 0), (2, 1), (2, 2), (2, 3)],         // |
            vec![(2, 0), (3, 0), (2, 1), (3, 1)],         // []
        ];

        return Day17 {
            jets: Vec::new(),
            rocks: rocks,
        };
    }

    fn try_move_rock(
        &self,
        rock: &Vec<(isize, isize)>,
        dir: (isize, isize),
        occupied: &HashSet<(isize, isize)>,
    ) -> Option<Vec<(isize, isize)>> {
        let mut rc = rock.clone();
        for n in &mut rc {
            n.0 += dir.0;
            n.1 += dir.1;
        }
        if rc
            .iter()
            .any(|n| occupied.contains(n) || n.0 < 0 || n.0 >= WIDTH || n.1 < 0)
        {
            return None;
        }

        return Some(rc);
    }

    fn tetris(&self, blocks: usize) -> isize {
        let mut state = State {
            rock_ind: 0,
            jet_ind: 0,
            peak: 0,
            occupied: HashSet::new(),
            iter_count: 0,
        };
        self._sim_n_times(blocks, &mut state);
        return state.peak;
    }

    fn _sim_n_times(&self, iters: usize, state: &mut State) {
        state.iter_count += iters;
        let mut low_points = [0; 7];
        for _ in 0..iters {
            let mut rock = self
                .try_move_rock(
                    &self.rocks[state.rock_ind].clone(),
                    (0, state.peak + 3),
                    &state.occupied,
                )
                .unwrap();
            state.rock_ind = (state.rock_ind + 1) % 5;

            loop {
                let jet = if self.jets[state.jet_ind] == b'<' {
                    (-1, 0)
                } else {
                    (1, 0)
                };
                state.jet_ind = (state.jet_ind + 1) % self.jets.len();
                match self.try_move_rock(&rock, jet, &state.occupied) {
                    Some(r) => {
                        rock = r;
                    }
                    None => {}
                }

                match self.try_move_rock(&rock, (0, -1), &state.occupied) {
                    Some(r) => {
                        rock = r;
                    }
                    None => {
                        for n in &rock {
                            state.peak = state.peak.max(n.1 + 1);
                            low_points[n.0 as usize] = low_points[n.0 as usize].min(n.1);
                            state.occupied.insert(*n);
                        }
                        break;
                    }
                }

                // this is arbitary
                if state.jet_ind % 2022 == 0 {
                    let m = low_points.iter().min().unwrap();
                    state.occupied.retain(|(_, y)| y >= m);
                }
            }
        }
        let m = low_points.iter().min().unwrap();
        state.occupied.retain(|(_, y)| y >= m);
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.jets = raw_input.as_bytes().to_vec();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        return Box::new(self.tetris(2022));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

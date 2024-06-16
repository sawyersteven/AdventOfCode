use crate::Base;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

const WIDTH: isize = 7;
const MIN_PATTERN_LEN: usize = 10;

const ROCKS: &[&[(isize, isize)]] = &[
    &[(2, 0), (3, 0), (4, 0), (5, 0)],         // -
    &[(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)], // +
    &[(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)], // _|
    &[(2, 0), (2, 1), (2, 2), (2, 3)],         // |
    &[(2, 0), (3, 0), (2, 1), (3, 1)],         // []
];

pub struct Day17 {
    jets: Vec<u8>,
}

impl Day17 {
    pub fn new() -> Day17 {
        return Day17 { jets: Vec::new() };
    }
}

impl Base for Day17 {
    fn parse_input(&mut self, raw_input: String) {
        self.jets = raw_input.as_bytes().to_vec();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut game = Game::new(&self.jets);

        return Box::new(game.get_peak_after_drops(2022));
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut game = Game::new(&self.jets);

        return Box::new(game.get_peak_after_drops(1000000000000));
    }
}

struct Game<'a> {
    rock_ind: usize,
    jet_ind: usize,
    peak: isize,
    occupied: HashSet<(isize, isize)>,
    iter_count: usize,
    jets: &'a Vec<u8>,
    pattern_records: HashMap<usize, usize>,
    height_records: HashMap<usize, usize>,
    pattern_len: usize,
    last_pattern_drop_delta: usize,
}

impl<'a> Game<'a> {
    fn new(jets: &'a Vec<u8>) -> Self {
        return Game {
            rock_ind: 0,
            jet_ind: 0,
            peak: 0,
            occupied: HashSet::new(),
            iter_count: 0,
            jets: jets,
            pattern_records: HashMap::new(),
            height_records: HashMap::new(),
            pattern_len: 0,
            last_pattern_drop_delta: 0,
        };
    }

    // Attempts to set rock to new position. If collision returns false and
    // rock is unmodified
    fn try_move_rock(&self, rock: &mut Vec<(isize, isize)>, dir: (isize, isize)) -> bool {
        let mut rc = rock.clone();
        for n in &mut rc {
            n.0 += dir.0;
            n.1 += dir.1;
        }
        if rc
            .iter()
            .any(|n| !(0..WIDTH).contains(&n.0) || n.1 < 0 || self.occupied.contains(n))
        {
            return false;
        }
        for i in 0..rc.len() {
            rock[i] = rc[i];
        }
        return true;
    }

    fn get_peak_after_drops(&mut self, rock_drops: usize) -> usize {
        self.iter_count += rock_drops;

        for rock_num in 0..rock_drops {
            self.rock_ind = rock_num % 5;

            // spawn new rock
            let mut rock = ROCKS[self.rock_ind].to_vec();
            self.try_move_rock(&mut rock, (0, self.peak + 3));

            loop {
                let jet_x = if self.jets[self.jet_ind] == b'<' { -1 } else { 1 };
                self.jet_ind = (self.jet_ind + 1) % self.jets.len();

                self.try_move_rock(&mut rock, (jet_x, 0)); // don't care if jet move collides

                if !self.try_move_rock(&mut rock, (0, -1)) {
                    // rock has landed
                    for n in &rock {
                        self.peak = self.peak.max(n.1 + 1);
                        self.occupied.insert(*n);
                    }
                    break;
                }
            }
            match self.check_pattern(rock_num, rock_drops, self.rock_ind, self.jet_ind, self.peak as usize) {
                Some(f) => return self.peak as usize + f,
                None => {}
            }
        }
        return self.peak as usize;
    }

    fn check_pattern(
        &mut self,
        rock_num: usize,
        total_rocks: usize,
        rock_type_ind: usize,
        jet_ind: usize,
        peak_y: usize,
    ) -> Option<usize> {
        /* This is a bit hard to follow, so here are the main ideas:

        * Give each jet/rock_type index a uid. rock_type maxes out at 5, so as
        long as the jet pattern isn't over 2^61 in length this is ok.
        * Keep a record of this uid's peak and rock_number
        * If a state has been found before start looking for a pattern:
            * If a new uid is found the pattern resets and is discarded
            * For each next uid, check that the drop delta is the same as the
            previous uid
            * If the uids and drop deltas repeat MIN_PATTERN_LEN times *AND*
            the pattern fits neatly into the remaining drop counts it is done
         */
        let state_id = (jet_ind << 3) | rock_type_ind;

        if *self.pattern_records.get(&state_id).unwrap_or(&0) != 0 {
            let drop_delta = rock_num - self.pattern_records[&state_id];
            let height_delta = peak_y - *self.height_records.get(&state_id).unwrap_or(&0);

            if self.last_pattern_drop_delta == 0 {
                self.last_pattern_drop_delta = drop_delta;
            }

            if self.last_pattern_drop_delta == drop_delta {
                self.pattern_len += 1;
            } else {
                self.pattern_len = 0;
            }

            if self.pattern_len > MIN_PATTERN_LEN && (rock_num % drop_delta) == ((total_rocks - 1) % drop_delta) {
                let remaining_drop_num = total_rocks - rock_num;
                return Some(remaining_drop_num / drop_delta * height_delta);
            }
        } else {
            self.last_pattern_drop_delta = 0;
            self.pattern_len = 0;
        }
        self.height_records.insert(state_id, peak_y);
        self.pattern_records.insert(state_id, rock_num);

        return None;
    }
}

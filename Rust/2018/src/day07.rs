use crate::Base;
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Display,
};

pub struct Day07 {
    on_deck: HashSet<isize>,
    requirements: HashMap<isize, HashSet<isize>>,
}

impl Day07 {
    pub fn new() -> Day07 {
        return Day07 {
            on_deck: HashSet::new(),
            requirements: HashMap::new(),
        };
    }
}

impl Base for Day07 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let prereq = line.as_bytes()[36] as isize;
            let step = line.as_bytes()[5] as isize;
            if !self.requirements.contains_key(&prereq) {
                self.requirements.insert(prereq, HashSet::<isize>::new());
            }
            if !self.requirements.contains_key(&step) {
                self.requirements.insert(step, HashSet::<isize>::new());
            }
            self.requirements.get_mut(&prereq).unwrap().insert(step);
            self.on_deck.insert(step);
        }
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let mut on_deck = self.on_deck.clone();
        let mut reqs = self.requirements.clone();

        for c in self.requirements.keys() {
            on_deck.remove(c);
        }

        let mut complete = String::new();
        while on_deck.len() > 0 {
            let current = *on_deck.iter().min().unwrap();
            on_deck.remove(&current);
            for (k, v) in &mut reqs {
                v.remove(&current);
                if v.len() == 0 {
                    on_deck.insert(*k);
                }
            }
            reqs.retain(|_, v| v.len() != 0);
            complete.push(current as u8 as char);
        }

        return Box::new(complete);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let mut reqs = self.requirements.clone();
        let mut on_deck = HashSet::<isize>::new();

        for k in reqs.keys() {
            if reqs[k].len() == 0 {
                on_deck.insert(*k);
            }
        }

        let mut time = 0;
        const BASETIME: isize = 60;

        let mut workers = [Worker::new(0, 0); 5];

        loop {
            let mut time_reduction: isize;
            if on_deck.len() == 0 {
                time_reduction = isize::MAX;
                for w in workers {
                    if w.time <= 0 {
                        continue;
                    }
                    time_reduction = time_reduction.min(w.time);
                }
            } else {
                time_reduction = workers.iter().min().unwrap().time;
                time_reduction = time_reduction.max(0);
            }

            time += time_reduction;

            let mut still_working = false;
            for i in 0..workers.len() {
                workers[i].time -= time_reduction;
                if workers[i].time <= 0 {
                    for (k, v) in &mut reqs {
                        v.remove(&workers[i].task);
                        if v.len() == 0 {
                            on_deck.insert(*k);
                        }
                    }
                    reqs.retain(|_, v| v.len() != 0);
                    workers[i].task = *on_deck.iter().min().unwrap_or(&0);
                    workers[i].time = BASETIME + 1 + (workers[i].task - 0x41);
                    on_deck.remove(&workers[i].task);
                }
                if workers[i].time > 0 {
                    still_working = true;
                }
            }

            if on_deck.len() == 0 && !still_working {
                break;
            }
        }

        return Box::new(time);
    }
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Eq)]
struct Worker {
    pub task: isize,
    pub time: isize,
}

impl Ord for Worker {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if other.time == self.time {
            return Ordering::Equal;
        }
        return if other.time > self.time {
            Ordering::Less
        } else {
            Ordering::Greater
        };
    }

    fn max(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::max_by(self, other, Ord::cmp)
    }

    fn min(self, other: Self) -> Self
    where
        Self: Sized,
    {
        std::cmp::min_by(self, other, Ord::cmp)
    }
}

impl Worker {
    pub fn new(task: isize, time: isize) -> Self {
        return Worker { task: task, time: time };
    }
}

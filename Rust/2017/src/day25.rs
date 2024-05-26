use crate::Base;
use std::{
    collections::{HashMap, VecDeque},
    fmt::Display,
};

pub struct Day25 {
    starts: (isize, isize),
    rules: HashMap<isize, [[isize; 3]; 2]>,
}

impl Day25 {
    pub fn new() -> Day25 {
        return Day25 {
            starts: (0, 0),
            rules: HashMap::new(),
        };
    }
}

impl Base for Day25 {
    fn parse_input(&mut self, raw_input: String) {
        let lines: Vec<&str> = raw_input.split('\n').collect();

        self.starts.0 = (lines[0].bytes().nth_back(1).unwrap() - b'A') as isize;
        self.starts.1 = lines[1].split(' ').nth_back(1).unwrap().parse().unwrap();

        let mut i = 0;
        while i < lines.len() {
            if lines[i].is_empty() || lines[i].as_bytes()[0] != b'I' {
                i += 1;
                continue;
            }

            let state = (lines[i].bytes().nth_back(1).unwrap() - b'A') as isize;
            i += 1;
            let mut commands = [[0; 3]; 2];

            while i < lines.len() {
                if lines[i].is_empty() {
                    i += 1;
                    break;
                }

                let rule_num = lines[i].chars().nth_back(1).unwrap().to_digit(10).unwrap() as usize;
                let write_val = if lines[i + 1].chars().nth_back(1).unwrap() == '0' {
                    0
                } else {
                    1
                };

                let move_dir = if lines[i + 2].chars().nth_back(2).unwrap() == 'f' {
                    -1
                } else {
                    1
                };
                let end_state = (lines[i + 3].bytes().nth_back(1).unwrap() - b'A') as isize;

                commands[rule_num] = [write_val, move_dir, end_state];
                i += 4;
            }
            i += 0;
            self.rules.insert(state, commands);
        }
    }

    // The linkedlist::cursor feature is not available in stable rust as
    // of now, so this runs slower that it should because it relies
    // on a vecdequeue
    fn part1(&self) -> Box<dyn Display> {
        let mut state = self.starts.0;
        let steps = self.starts.1;

        let mut tape = VecDeque::<isize>::new();
        tape.push_front(0);
        let mut cursor_ind = 0;

        for _ in 0..steps {
            let commands = self.rules[&state][tape[cursor_ind] as usize];
            tape[cursor_ind] = commands[0];

            if commands[1] == 1 {
                cursor_ind += 1;
                if cursor_ind == tape.len() {
                    tape.push_back(0);
                }
            } else {
                if cursor_ind == 0 {
                    tape.push_front(0);
                } else {
                    cursor_ind -= 1;
                }
            }
            state = commands[2];
        }

        let mut checksum = 0;
        for i in tape {
            checksum += i;
        }

        return Box::new(checksum);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("*");
    }
}

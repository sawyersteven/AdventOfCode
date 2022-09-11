use crate::Base;
use std::fmt::Display;

pub struct Day04 {
    pub input: Vec<String>,
}

impl Day04 {
    pub fn new() -> Day04 {
        return Day04 { input: Vec::new() };
    }
}

impl Base for Day04 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.split('\n').map(|x| x.to_string()).collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut sum = 0;
        for line in &self.input {
            if is_valid(&line) {
                sum += line
                    .split('[')
                    .nth(0)
                    .unwrap()
                    .split('-')
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
        }
        return Box::new(sum);
    }

    fn part2(&self) -> Box<dyn Display> {
        for line in &self.input {
            let room_id: usize = line
                .split('[')
                .nth(0)
                .unwrap()
                .split('-')
                .last()
                .unwrap()
                .parse()
                .unwrap();

            let letters: Vec<char> = line.chars().collect();

            // translated line starts with "north"
            if transpose(letters[0], room_id) == 'n' && transpose(letters[1], room_id) == 'o' {
                return Box::new(room_id);
            }
        }

        return Box::new("-");
    }
}

fn transpose(c: char, r: usize) -> char {
    let rot = (r % 26) as u8;
    let mut n = c as u8;
    n += rot;
    if n > 'z' as u8 {
        n -= 26
    }
    return n as char;
}

fn is_valid(room: &String) -> bool {
    let split = room.rfind('-').unwrap();
    let mut letters: Vec<char> = room[0..split].replace('-', "").chars().collect();
    letters.sort();

    let mut counts: Vec<(char, usize)> = Vec::new();

    let mut counter = (letters[0], 1);
    for i in 1..letters.len() {
        if letters[i] != counter.0 {
            counts.push(counter);
            counter.0 = letters[i];
            counter.1 = 1;
        } else {
            counter.1 += 1;
        }
    }

    counts.push(counter);

    counts.sort_by(|a, b| {
        if a.1 == b.1 {
            return a.0.cmp(&b.0);
        }
        return b.1.cmp(&a.1);
    });

    let checksum: Vec<char> = room.split('[').last().unwrap().chars().collect();

    for i in 0..5 {
        if counts[i].0 != checksum[i] {
            return false;
        }
    }
    return true;
}

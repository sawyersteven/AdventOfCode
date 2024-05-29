use crate::Base;
use std::{collections::VecDeque, fmt::Display};

pub struct Day09 {
    pub game_data: (usize, usize),
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { game_data: (0, 0) };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        let mut parts = raw_input.split(' ');
        self.game_data.0 = parts.nth(0).unwrap().parse().unwrap();
        self.game_data.1 = parts.nth_back(1).unwrap().parse().unwrap();
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let (player_count, last_score) = self.game_data;
        return Box::new(*play_game(player_count, last_score).iter().max().unwrap());
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let (player_count, last_score) = self.game_data;
        return Box::new(*play_game(player_count, last_score * 100).iter().max().unwrap());
    }
}

// This is super slow due to vec inserts/removes.
// There is no cursor available in stable rust's linked list at this time
fn play_game(player_count: usize, last_marble: usize) -> Vec<usize> {
    let mut players = vec![0; player_count];

    let mut circle = VecDeque::new();
    circle.push_back(0);
    let mut current_index = 0;

    for i in 1..last_marble {
        let marble_num = i;
        if marble_num % 23 == 0 {
            let player = (marble_num - 1) % player_count;
            players[player] += marble_num;
            current_index = (current_index + circle.len() - 7) % circle.len();
            players[player] += circle[current_index];
            circle.remove(current_index);
        } else {
            let insert_at = (current_index + 2) % circle.len();
            circle.insert(insert_at, marble_num);
            current_index = insert_at;
        }
    }

    return players;
}

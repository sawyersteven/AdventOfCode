use shared::v2i::Vector2Int;

use crate::Base;
use std::{collections::HashMap, fmt::Display};

type PAD = HashMap<u8, Vector2Int>;

pub struct Day21 {
    input: String,
    robot_cache: HashMap<(Vec<u8>, usize), usize>,
    numpad: PAD,
    dirpad: PAD,
    dir_button_steps: [(Vector2Int, u8); 4],
}

// Breaking my habit of completely separating p1 and Vector2Int from each other
// This one is too brutal. There's also a lot of comments to keep me sane.
impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            input: String::new(),
            robot_cache: HashMap::new(),
            numpad: HashMap::from_iter(
                "789456123_0A"
                    .bytes()
                    .enumerate()
                    .map(|(i, b)| (b, Vector2Int::new(i as isize % 3, i as isize / 3))),
            ),
            dirpad: HashMap::from_iter(
                "_^A<v>"
                    .bytes()
                    .enumerate()
                    .map(|(i, b)| (b, Vector2Int::new(i as isize % 3, i as isize / 3))),
            ),
            dir_button_steps: [
                (Vector2Int::new(-1, 0), b'<'),
                (Vector2Int::new(1, 0), b'>'),
                (Vector2Int::new(0, -1), b'^'),
                (Vector2Int::new(0, 1), b'v'),
            ],
        };
    }
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input;
    }

    fn part1(&mut self) -> Box<dyn Display> {
        let input = self.input.clone();
        let codes: Vec<&str> = input.lines().collect();

        let ans = self.solve(codes, 4);
        self.robot_cache.clear();
        return Box::new(ans);
    }

    fn part2(&mut self) -> Box<dyn Display> {
        let input = self.input.clone();
        let codes: Vec<&str> = input.lines().collect();
        let ans = self.solve(codes, 27);
        return Box::new(ans == 264518225304496);
    }
}

impl Day21 {
    // Pad count is total number of keypads, numeric AND directional
    fn solve(&mut self, codes: Vec<&str>, pad_count: usize) -> usize {
        let mut total_steps = 0;
        for row in &codes {
            let mut code_steps = 0;

            let mut current = self.numpad[&b'A']; // all bots start at A
            for val in row.as_bytes() {
                // for step in code
                let next = self.numpad[&val];
                code_steps += self.get_keypad_steps(current, next, pad_count, self.numpad[&b'_']);
                current = next;
            }

            let numcode: usize = row[0..(row.len() - 1)].parse().unwrap();
            total_steps += code_steps * numcode;
        }
        return total_steps;
    }

    // Gets number of moves required for a robot to enter path by sending two
    // button coords to get_keypad_steps and totaling the step
    // Is memoized in self.robot_cache
    fn best_robot(&mut self, path: Vec<u8>, pad_count: usize) -> usize {
        if pad_count == 1 {
            return path.len();
        }

        // this is dumb but so is the borrow checker sometimes
        let r#tmp = (path, pad_count);
        if let Some(res) = self.robot_cache.get(&r#tmp) {
            return *res;
        }
        let (path, pad_count) = r#tmp;

        let mut steps = 0;
        let mut current = self.dirpad[&b'A']; // all robots start at A
        for btn in &path {
            let btn_coord = self.dirpad[btn];
            steps += self.get_keypad_steps(current, btn_coord, pad_count, self.dirpad[&b'_']);
            current = btn_coord;
        }
        self.robot_cache.insert((path, pad_count), steps);
        return steps;
    }

    // Gets number of steps in shortest route from start to end, avoiding
    // bad_coord, regardless of type of keypad, and pass it to a robot to
    // generate the button presses
    fn get_keypad_steps(
        &mut self,
        start: Vector2Int,
        end: Vector2Int,
        pad_count: usize,
        bad_coord: Vector2Int,
    ) -> usize {
        let mut best_count = usize::MAX;

        // all of these get consumed, so no benefit in a vecdeque
        let mut stack = Vec::new();
        stack.push((start, Vec::new()));

        while let Some((current_coord, path)) = stack.pop() {
            if current_coord == end {
                let mut p = path.clone();
                p.push(b'A');
                best_count = best_count.min(self.best_robot(p, pad_count - 1));
            } else if current_coord != bad_coord {
                for (step, button) in self
                    .dir_button_steps
                    .iter()
                    .filter(|(s, _)| is_good_move(current_coord, end, *s))
                {
                    let mut p = path.clone();
                    p.push(*button);
                    stack.push((current_coord + *step, p));
                }
            }
        }
        return best_count;
    }
}
// Does change move start closer to dest?
#[inline(always)]
fn is_good_move(start: Vector2Int, dest: Vector2Int, change: Vector2Int) -> bool {
    return (start + change).manhattan(&dest) < start.manhattan(&dest);
}

use crate::Base;
use std::fmt::Display;

pub struct Day09 {
    pub input: Vec<char>,
}

impl Day09 {
    pub fn new() -> Day09 {
        return Day09 { input: Vec::new() };
    }
}

impl Base for Day09 {
    fn parse_input(&mut self, raw_input: String) {
        self.input = raw_input.chars().collect();
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut total_len = self.input.len();

        let mut i = 0;
        while i < self.input.len() {
            if self.input[i] != '(' {
                i += 1;
                continue;
            }
            let marker_len = find_next(&self.input, ')', i + 1) - i + 1;
            let seg = String::from_iter(self.input[(i + 1)..(i + marker_len - 1)].into_iter());
            let marker_parts: Vec<usize> = seg
                .split('x')
                .map(|x| {
                    return x.parse().unwrap();
                })
                .collect();

            total_len += marker_parts[0] * (marker_parts[1] - 1) - marker_len;
            i += marker_parts[0];
            i += 1;
        }

        return Box::new(total_len);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new(recursive_decomp(&self.input));
    }
}

fn recursive_decomp(input: &[char]) -> usize {
    let mut len = input.len();

    let mut i = 0;
    while i < input.len() {
        if input[i] != '(' {
            i += 1;
            continue;
        }
        let marker_len = find_next(&input, ')', i + 1) - i + 1;
        let seg = String::from_iter(input[(i + 1)..(i + marker_len - 1)].into_iter());
        let marker_parts: Vec<usize> = seg
            .split('x')
            .map(|x| {
                return x.parse().unwrap();
            })
            .collect();

        let chunk_start = i + marker_len;
        let chunk_end = chunk_start + marker_parts[0];

        let chunk = &input[chunk_start..chunk_end];
        let decomp_len = recursive_decomp(chunk);

        len += decomp_len * marker_parts[1];
        len -= marker_parts[0] + marker_len;
        i += marker_len + marker_parts[0] - 1;
        i += 1;
    }
    return len;
}

fn find_next<T>(vec: &[T], needle: T, start_index: usize) -> usize
where
    T: Eq,
{
    for i in start_index..vec.len() {
        if vec[i] == needle {
            return i;
        }
    }
    return usize::MAX;
}

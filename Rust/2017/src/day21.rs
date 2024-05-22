use crate::{utils::vec2d, Base};
use std::{collections::HashMap, fmt::Display};

const INIT_STATE: [char; 9] = ['.', '#', '.', '.', '.', '#', '#', '#', '#'];

pub struct Day21 {
    pub lookup_table: HashMap<String, Vec<char>>,
}

impl Day21 {
    pub fn new() -> Day21 {
        return Day21 {
            lookup_table: HashMap::new(),
        };
    }

    fn enhance(&self, image: Vec<char>) -> Vec<char> {
        let mut cells = self.divide(image);
        for i in 0..cells.len() {
            cells[i] =
                self.lookup_table[&String::from_iter(cells[i].iter().map(|x| x.to_string() + "/"))].to_owned();
        }
        return self.join(cells);
    }

    fn join(&self, chunks: Vec<Vec<char>>) -> Vec<char> {
        let chunk_size = (chunks[0].len() as f32).sqrt().floor() as usize;
        let chunks_per_edge = (chunks.len() as f32).sqrt().floor() as usize;
        let mut result: Vec<char> = (0..(chunks.len() * chunks[0].len())).map(|_| ' ').collect();

        let mut start = 0;
        for i in 0..chunks.len() {
            let mut j = 0;
            for y in 0..chunk_size {
                for x in 0..chunk_size {
                    let ind = start + x + (y * chunk_size * chunks_per_edge);
                    result[ind] = chunks[i][j];
                    j += 1;
                }
            }
            start += chunk_size;
            if start % (chunk_size * chunks_per_edge) == 0 {
                start += chunk_size * chunks_per_edge * (chunk_size - 1);
            }
        }

        return result;
    }

    fn divide(&self, image: Vec<char>) -> Vec<Vec<char>> {
        let chunk_size = 2 + image.len() % 2;
        let chunks_per_edge = f32::sqrt(image.len() as f32) as usize / chunk_size;
        let mut chunks = vec2d(' ', chunks_per_edge * chunks_per_edge, chunk_size * chunk_size);

        let mut start = 0;
        for i in 0..chunks.len() {
            let mut j = 0;
            for y in 0..chunk_size {
                for x in 0..chunk_size {
                    let ind = start + x + (y * chunk_size * chunks_per_edge);
                    chunks[i][j] = image[ind];
                    j += 1;
                }
            }
            start += chunk_size;
            if start % (chunks_per_edge * chunk_size) == 0 {
                start += chunks_per_edge * chunk_size * (chunk_size - 1);
            }
        }
        return chunks;
    }

    fn line_to_grid(line: &str) -> Vec<Vec<char>> {
        let parts: Vec<Vec<char>> = line.split('/').map(|x| x.chars().collect()).collect();
        let mut grid = vec2d(' ', parts.len(), parts[0].len());

        for y in 0..parts.len() {
            for x in 0..parts[0].len() {
                grid[y][x] = parts[y][x];
            }
        }
        return grid;
    }
}

fn rotate_cw<T>(arr: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    let sz = arr.len();
    if sz != arr[0].len() {
        panic!("Must be square");
    }

    let mut tmp_map = Vec::<Vec<T>>::new();
    for row in 0..sz {
        let col = sz - 1;
        let mut ncol = 0;
        tmp_map.push(Vec::<T>::new());
        for col in col..0 {
            tmp_map[ncol][row] = arr[col][row];

            ncol += 1;
        }
    }
}

fn flip_horiz<T>(arr: &mut Vec<Vec<T>>)
where
    T: Copy,
{
    let h = arr.len();
    let w = arr[0].len();

    let mut tmp: T;

    for y in 0..h {
        for x in 0..(w / 2) {
            tmp = arr[y][x];
            arr[y][x] = arr[y][w - x - 1];
            arr[y][w - x - 1] = tmp;
        }
    }
}

fn join_2d<T>(arr: &Vec<Vec<T>>, sep: &str) -> String
where
    T: ToString,
{
    return String::from_iter(
        arr.iter()
            .map(|row| String::from_iter(row.iter().map(|c| c.to_string() + sep)) + sep),
    );
}

impl Base for Day21 {
    fn parse_input(&mut self, raw_input: String) {
        for line in raw_input.split('\n') {
            let parts: Vec<&str> = line.split(" => ").collect();

            let v: Vec<char> = parts[1].replace("/", "").chars().collect();
            let mut k = Self::line_to_grid(parts[0]);

            for _ in 0..4 {
                rotate_cw(&mut k);
                self.lookup_table.insert(join_2d(&k, "/"), v.clone());
            }
            flip_horiz(&mut k);
            for _ in 0..4 {
                rotate_cw(&mut k);
                self.lookup_table.insert(join_2d(&k, "/"), v.clone());
            }
        }
    }

    fn part1(&self) -> Box<dyn Display> {
        let mut image = INIT_STATE.clone().to_vec();
        for _turn in 0..5 {
            image = self.enhance(image);
        }

        let count = image.iter().filter(|&x| x == &'#').count();
        return Box::new(count);
    }

    fn part2(&self) -> Box<dyn Display> {
        return Box::new("-");
    }
}

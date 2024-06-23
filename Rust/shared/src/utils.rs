#![allow(unused)]

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
};

pub fn split_input(raw_input: String) -> Vec<String> {
    return raw_input.split("\n").map(|x| String::from(x)).collect();
}

pub fn read_user_input(prompt: &str) {
    println!("{prompt}");
    let mut s = String::new();
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
}

pub fn parse_to_char2d(raw_input: String, default_char: char, padding: usize) -> Vec<Vec<char>> {
    let lines: Vec<&[u8]> = raw_input.lines().map(|x| x.as_bytes()).collect();
    let h = lines.len();
    let w = lines.iter().map(|x| x.len()).max().unwrap();

    let mut grid = vec![vec![default_char; w + (padding * 2)]; h + (padding * 2)];

    for y in 0..h {
        for x in 0..w {
            grid[y + padding][x + padding] =
                *lines.get(y).and_then(|col| col.get(x)).unwrap_or(&(default_char as u8)) as char;
        }
    }

    return grid;
}

pub fn parse_to_u8_grid(raw_input: &String, default_val: u8, padding: usize) -> Vec<Vec<u8>> {
    let lines: Vec<&[u8]> = raw_input.lines().map(|x| x.as_bytes()).collect();
    let h = lines.len();
    let w = lines.iter().map(|x| x.len()).max().unwrap();

    let mut grid = vec![vec![default_val; w + (padding * 2)]; h + (padding * 2)];

    for y in 0..h {
        for x in 0..w {
            grid[y + padding][x + padding] = *lines.get(y).and_then(|col| col.get(x)).unwrap_or(&default_val);
        }
    }

    return grid;
}

pub fn print_grid<T>(grid: &Vec<Vec<T>>)
where
    T: Display,
{
    for line in grid {
        let mut out = String::new();
        for item in line {
            out += &*item.to_string();
        }
        println!("{}", out);
    }
}

/*
I'll be honest. I have no idea what kind of arcane black magic is happening
here. This is the first time I've used ChatGPT to figure an alorithm and this
is the result. It works great and I never would have figured it out on my own,
just don't ask me to explain it. All I did was wrap it in an iterator and clean
up a little.

This returns every possible permutation of bitcount bits set in a u32 until a
permutation exceeds max

Example:
    bit_permutations(2, 0b1111)
    Result:
        00000011
        00000101
        00000110
        00001001
        00001010
        00001100

*/
fn bit_permutations(bitcount: u32, max: u32) -> impl Iterator<Item = u32> {
    let mut permutation = 0;
    for i in 0..bitcount {
        permutation |= 1 << i;
    }

    std::iter::from_fn(move || loop {
        if permutation > max {
            return None;
        }
        let p = permutation;
        let x = permutation & (!permutation + 1);
        let y = permutation + x;
        permutation = (((permutation & !y) / x) >> 1) | y;

        return Some(p);
    })
}

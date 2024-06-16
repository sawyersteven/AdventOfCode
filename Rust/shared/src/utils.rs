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
    let lines: Vec<&[u8]> = raw_input.split("\n").map(|x| x.as_bytes()).collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut grid = vec![vec![default_char; w + (padding * 2)]; h + (padding * 2)];

    for y in 0..h {
        for x in 0..w {
            grid[y + padding][x + padding] = lines[y][x] as char;
        }
    }

    return grid;
}

pub fn parse_to_u8_grid(raw_input: String, default_val: u8, padding: usize) -> Vec<Vec<u8>> {
    let lines: Vec<&[u8]> = raw_input.split("\n").map(|x| x.as_bytes()).collect();
    let h = lines.len();
    let w = lines[0].len();

    let mut grid = vec![vec![default_val; w + (padding * 2)]; h + (padding * 2)];

    for y in 0..h {
        for x in 0..w {
            grid[y + padding][x + padding] = lines[y][x];
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

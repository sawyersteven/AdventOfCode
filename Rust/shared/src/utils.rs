#![allow(unused)]
use core::fmt::Debug;

use std::{
    fmt::Display,
    io::{stdin, stdout, Write},
    ops::{AddAssign, Mul},
    process::Output,
};

pub fn lcm_array(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let mut m = lcm(nums[0], nums[1]);
    for i in 2..nums.len() {
        m = lcm(m, nums[i]);
    }
    return m;
}

pub fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

pub fn gcd(mut a: usize, mut b: usize) -> usize {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
}

pub fn gcd_f64(mut a: f64, mut b: f64) -> f64 {
    if a == b {
        return a;
    }
    if b > a {
        let temp = a;
        a = b;
        b = temp;
    }
    while b > 0.0 {
        let temp = a;
        a = b;
        b = temp % b;
    }
    return a;
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

/*
Attempt to find repeating pattern in data generated by data_source function
Calls data_source at least search_offset times
A pattern is valid only if it repeats pattern_stability times without variation
If a pattern is found, a vec containing the pattern's contents, starting at
[search_offset + 1] is returned
A result of None does not mean there is no pattern, only that a pattern that
complies with the params has been found
*/
pub fn find_pattern<T>(
    search_offset: usize,
    min_pattern_len: usize,
    max_pattern_len: usize,
    pattern_stability: usize,
    data_source: &mut impl FnMut() -> T,
) -> Option<Vec<T>>
where
    T: PartialEq + Debug + Display + Clone,
{
    let pattern_stability = pattern_stability.min(2);
    let max_pattern_len = max_pattern_len.max(min_pattern_len);

    let mut sample = Vec::new();
    for _ in 0..search_offset {
        let _ = data_source();
    }

    let sample_size = max_pattern_len * pattern_stability;
    for _ in 0..sample_size {
        sample.push(data_source());
    }

    let first_pattern_item = &sample[0];

    let mut pattern_start_ind = 1;
    while pattern_start_ind < sample.len() {
        // Attempt to find length of pattern by finding repeats of data[0]
        let mut pattern_len;
        match sample
            .iter()
            .skip(pattern_start_ind)
            .position(|x| x == first_pattern_item)
        {
            None => return None,
            Some(i) => {
                pattern_len = pattern_start_ind + i;
            }
        }

        if pattern_len < min_pattern_len {
            pattern_start_ind += pattern_len;
            continue;
        }
        if pattern_len > max_pattern_len {
            return None;
        }

        let mut all_match = true;
        // check for data[0..len] is identical to data[repeat_ind..(reapeat_ind + len)]
        'outer: for ps in 1..pattern_stability {
            for pl in 0..pattern_len {
                let master = &sample[pl];
                let repeat = &sample[pl + (ps * pattern_len)];
                if repeat != master {
                    all_match = false;
                    break 'outer;
                }
            }
        }

        if all_match {
            return Some(sample[0..pattern_len].iter().cloned().collect());
        }

        pattern_start_ind += pattern_len;
    }
    return None;
}

pub fn count_bits<T>(val: &T) -> usize
where
    T: Copy
        + Mul
        + std::ops::BitAnd<i32, Output = T>
        + AddAssign<T>
        + std::cmp::PartialOrd<i32>
        + std::ops::ShrAssign<i32>
        + Into<usize>,
{
    let mut count = 0;
    let mut n = *val;
    while n > 0 {
        count += (n & 1).into();
        n >>= 1;
    }
    return count;
}

mod test {
    use super::find_pattern;

    #[test]
    fn test_find_pattern() {
        let mut d = 0;
        match find_pattern(100, 5, 15, 3, &mut || {
            d = (d + 7) % 13;
            return d;
        }) {
            None => {}
            Some(p) => {
                assert_eq!(p.len(), 13);
                assert_eq!(p[0], 5);
                assert_eq!(*p.last().unwrap(), 11);
            }
        }
    }
}

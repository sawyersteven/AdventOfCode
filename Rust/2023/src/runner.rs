use crate::Base;
use std::fs;

pub const YEAR: usize = 2023;

pub enum Part {
    One,
    Two,
    Both,
}

pub fn run_day(day: usize, part: Part) {
    let input = read_input_file(day);
    let mut problem = get_day(day).unwrap();

    println!("Day {}", day);

    let now = std::time::Instant::now();
    problem.parse_input(input);
    let elapsed = now.elapsed();
    println!("│ Parse Input [{}ms | {}us]", elapsed.as_millis(), elapsed.as_micros());

    match part {
        Part::One => {
            run_part1(&mut problem);
        }
        Part::Two => {
            run_part2(&mut problem);
        }
        Part::Both => {
            run_part1(&mut problem);
            run_part2(&mut problem);
        }
    }
}

fn run_part1(problem: &mut Box<dyn Base>) {
    let now = std::time::Instant::now();
    let answer = problem.part1();
    let elapsed = now.elapsed();
    println!(
        "│ Part 1 [{:03}ms | {:03}us]: {}",
        elapsed.as_millis(),
        elapsed.as_micros(),
        answer
    );
}

fn run_part2(problem: &mut Box<dyn Base>) {
    let now = std::time::Instant::now();
    let answer = problem.part2();
    let elapsed = now.elapsed();
    println!(
        "│ Part 2 [{:03}ms | {:03}us]: {}",
        elapsed.as_millis(),
        elapsed.as_micros(),
        answer
    );
}

fn get_day(day: usize) -> Option<Box<dyn Base>> {
    let d: Box<dyn Base> = match day {
        1 => Box::new(crate::day01::Day01::new()),
        2 => Box::new(crate::day02::Day02::new()),
        3 => Box::new(crate::day03::Day03::new()),
        4 => Box::new(crate::day04::Day04::new()),
        5 => Box::new(crate::day05::Day05::new()),
        6 => Box::new(crate::day06::Day06::new()),
        7 => Box::new(crate::day07::Day07::new()),
        8 => Box::new(crate::day08::Day08::new()),
        9 => Box::new(crate::day09::Day09::new()),
        10 => Box::new(crate::day10::Day10::new()),
        11 => Box::new(crate::day11::Day11::new()),
        12 => Box::new(crate::day12::Day12::new()),
        13 => Box::new(crate::day13::Day13::new()),
        14 => Box::new(crate::day14::Day14::new()),
        15 => Box::new(crate::day15::Day15::new()),
        16 => Box::new(crate::day16::Day16::new()),
        17 => Box::new(crate::day17::Day17::new()),
        18 => Box::new(crate::day18::Day18::new()),
        19 => Box::new(crate::day19::Day19::new()),
        20 => Box::new(crate::day20::Day20::new()),
        21 => Box::new(crate::day21::Day21::new()),
        22 => Box::new(crate::day22::Day22::new()),
        23 => Box::new(crate::day23::Day23::new()),
        24 => Box::new(crate::day24::Day24::new()),
        25 => Box::new(crate::day25::Day25::new()),
        _ => {
            println!("Invalid day {}", day);
            return None;
        }
    };
    return Some(d);
}

fn read_input_file(day: usize) -> String {
    let file_loc = format!("../../Input/{}/{:02}.txt", YEAR, day);
    let path = std::path::Path::new(&file_loc);
    println!("{}", path.display());
    return fs::read_to_string(path).unwrap();
}

use crate::{Args, Base, Part};
use std::fs;

pub const YEAR: usize = 2024;

pub fn run_day(args: Args) {
    let input = read_input_file(args.day);
    let mut problem = get_day(args.day).unwrap();

    println!("Day {}", args.day);

    let now = std::time::Instant::now();
    problem.parse_input(input);
    let elapsed = now.elapsed();
    println!("- Parse Input [{}ms | {}us]", elapsed.as_millis(), elapsed.as_micros());

    match args.part {
        Part::One => {
            run_part1(&mut problem, args.benchmark_iters);
        }
        Part::Two => {
            run_part2(&mut problem, args.benchmark_iters);
        }
        Part::Both => {
            run_part1(&mut problem, args.benchmark_iters);
            run_part2(&mut problem, args.benchmark_iters);
        }
    }
}

fn run_part1(problem: &mut Box<dyn Base>, count: usize) {
    let mut answer: Box<(dyn std::fmt::Display + 'static)> = Box::new("");

    let mut times = Vec::new();

    for _ in 0..count {
        let now = std::time::Instant::now();
        answer = problem.part1();
        times.push(now.elapsed());
    }

    if count == 1 {
        println!(
            "- Part 1 [{:03}ms | {:03}us]: {}",
            times[0].as_millis(),
            times[0].as_micros(),
            answer
        );
    } else {
        let average_m = times.iter().map(|x| x.as_millis() as usize).sum::<usize>() / times.len();
        let average_u = times.iter().map(|x| x.as_micros() as usize).sum::<usize>() / times.len();
        let max = times.iter().max().unwrap();
        let min = times.iter().min().unwrap();
        println!(
            "┌ Part 1 ({} iterations)\n├ Avg [{:03}ms | {:03}us] \n├ Min [{:03}ms | {:03}us] \n├ Max [{:03}ms | {:03}us] \n└ Answer: {}",
            count, average_m, average_u, min.as_millis(), min.as_micros(), max.as_millis(), max.as_micros(),
            answer
        );
    }
}

fn run_part2(problem: &mut Box<dyn Base>, count: usize) {
    let mut answer: Box<(dyn std::fmt::Display + 'static)> = Box::new("");

    let mut times = Vec::new();

    for _ in 0..count {
        let now = std::time::Instant::now();
        answer = problem.part2();
        times.push(now.elapsed());
    }

    if count == 1 {
        println!(
            "- Part 2 [{:03}ms | {:03}us]: {}",
            times[0].as_millis(),
            times[0].as_micros(),
            answer
        );
    } else {
        let average_m = times.iter().map(|x| x.as_millis() as usize).sum::<usize>() / times.len();
        let average_u = times.iter().map(|x| x.as_micros() as usize).sum::<usize>() / times.len();
        let max = times.iter().max().unwrap();
        let min = times.iter().min().unwrap();
        println!(
            "┌ Part 2 ({} iterations)\n├ Avg [{:03}ms | {:03}us] \n├ Min [{:03}ms | {:03}us] \n├ Max [{:03}ms | {:03}us] \n└ Answer: {}",
            count, average_m, average_u, min.as_millis(), min.as_micros(), max.as_millis(), max.as_micros(),
            answer
        );
    }
}

fn get_day(day: usize) -> Option<Box<dyn Base>> {
    let d: Box<dyn Base>;
    match day {
        1 => d = Box::new(crate::day01::Day01::new()),
        2 => d = Box::new(crate::day02::Day02::new()),
        3 => d = Box::new(crate::day03::Day03::new()),
        4 => d = Box::new(crate::day04::Day04::new()),
        5 => d = Box::new(crate::day05::Day05::new()),
        6 => d = Box::new(crate::day06::Day06::new()),
        7 => d = Box::new(crate::day07::Day07::new()),
        8 => d = Box::new(crate::day08::Day08::new()),
        9 => d = Box::new(crate::day09::Day09::new()),
        10 => d = Box::new(crate::day10::Day10::new()),
        11 => d = Box::new(crate::day11::Day11::new()),
        12 => d = Box::new(crate::day12::Day12::new()),
        13 => d = Box::new(crate::day13::Day13::new()),
        14 => d = Box::new(crate::day14::Day14::new()),
        15 => d = Box::new(crate::day15::Day15::new()),
        16 => d = Box::new(crate::day16::Day16::new()),
        17 => d = Box::new(crate::day17::Day17::new()),
        18 => d = Box::new(crate::day18::Day18::new()),
        19 => d = Box::new(crate::day19::Day19::new()),
        20 => d = Box::new(crate::day20::Day20::new()),
        21 => d = Box::new(crate::day21::Day21::new()),
        22 => d = Box::new(crate::day22::Day22::new()),
        23 => d = Box::new(crate::day23::Day23::new()),
        24 => d = Box::new(crate::day24::Day24::new()),
        25 => d = Box::new(crate::day25::Day25::new()),
        _ => {
            println!("Invalid day {}", day);
            return None;
        }
    }
    return Some(d);
}

fn read_input_file(day: usize) -> String {
    let file_loc = format!("../../Input/{}/{:02}.txt", YEAR, day);
    let path = std::path::Path::new(&file_loc);
    println!("{}", path.display());
    return fs::read_to_string(path).unwrap();
}

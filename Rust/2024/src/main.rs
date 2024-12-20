use std::fmt::Display;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;
mod runner;
use clap::{arg, Parser, ValueEnum};

#[derive(Parser)]
struct Args {
    day: usize,

    #[arg(short, value_enum, default_value_t = Part::Both )]
    part: Part,

    #[arg(short, default_value_t = 1)]
    benchmark_iters: usize,
}

#[derive(Clone, ValueEnum)]
enum Part {
    One,
    Two,
    Both,
}

fn main() {
    let args = Args::parse();

    runner::run_day(args);
}

pub trait Base {
    fn parse_input(&mut self, raw_input: String);
    fn part1(&mut self) -> Box<dyn Display>;
    fn part2(&mut self) -> Box<dyn Display>;
}

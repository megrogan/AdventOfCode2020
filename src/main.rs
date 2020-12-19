#[macro_use] extern crate lazy_static;
extern crate regex;

use core::fmt::Debug;
use std::env;

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

fn main() {
    env_logger::init();

    let mut args = env::args();
    args.next();

    let day_or_all = &args.next().unwrap_or("all".to_string());

    run_day("day1", &day01::run, day_or_all);
    run_day("day2", &day02::run, day_or_all);
    run_day("day3", &day03::run, day_or_all);
    run_day("day4", &day04::run, day_or_all);
    run_day("day5", &day05::run, day_or_all);
    run_day("day6", &day06::run, day_or_all);
    run_day("day7", &day07::run, day_or_all);
    run_day("day8", &day08::run, day_or_all);
    run_day("day9", &day09::run, day_or_all);
    run_day("day10", &day10::run, day_or_all);
    run_day("day11", &day11::run, day_or_all);
    run_day("day12", &day12::run, day_or_all);
    run_day("day13", &day13::run, day_or_all);
    run_day("day14", &day14::run, day_or_all);
    run_day("day15", &day15::run, day_or_all);
    run_day("day16", &day16::run, day_or_all);
    run_day("day17", &day17::run, day_or_all);
    run_day("day18", &day18::run, day_or_all);
    run_day("day19", &day19::run, day_or_all);
}

fn run_day<T>(name: &str, f: & dyn Fn() -> T, day_or_all: &str) where T: Debug {
    if day_or_all == "all" || day_or_all == name {
        println!("{} = {:?}", name, f());
    }
}

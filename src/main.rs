#[macro_use] extern crate lazy_static;
extern crate regex;

use core::fmt::Debug;
use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main() {
    env_logger::init();

    let mut args = env::args();
    args.next();

    let day_or_all = &args.next().unwrap_or("all".to_string());

    run_day("day1", &day1::run, day_or_all);
    run_day("day2", &day2::run, day_or_all);
    run_day("day3", &day3::run, day_or_all);
    run_day("day4", &day4::run, day_or_all);
    run_day("day5", &day5::run, day_or_all);
    run_day("day6", &day6::run, day_or_all);
    run_day("day7", &day7::run, day_or_all);
    run_day("day8", &day8::run, day_or_all);
    run_day("day9", &day9::run, day_or_all);
}

fn run_day<T>(name: &str, f: & dyn Fn() -> T, day_or_all: &str) where T: Debug {
    if day_or_all == "all" || day_or_all == name {
        println!("{} = {:?}", name, f());
    }
}

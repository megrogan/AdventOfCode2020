use std::collections::HashSet;
use std::fs;

pub fn run() -> (i64, i64) {
    let contents = fs::read_to_string("input/day1.txt").expect("cannot open input file");

    const TARGET: i64 = 2020;

    let set: HashSet<i64> = contents
        .lines()
        .map(to_i64)
        .collect();

    let pair = find_pair_that_sums_to_target(&set, TARGET).unwrap();
    let triple = find_triple_that_sums_to_target(&set, TARGET).unwrap();

    (
        pair.0 * pair.1,
        triple.0 * triple.1 * triple.2
    )
}

fn find_pair_that_sums_to_target(set: &HashSet<i64>, target: i64) -> Option<(i64, i64)> {
    set
        .iter()
        .map(|x| (*x, target - *x))
        .filter(|p| set.contains(&p.1))
        .next()
}

fn find_triple_that_sums_to_target(set: &HashSet<i64>, target: i64) -> Option<(i64, i64, i64)> {
    set
        .iter()
        .map(|x| (*x, find_pair_that_sums_to_target(set, target - *x)))
        .filter(|t| t.1.is_some())
        .map(|t| (t.0, t.1.unwrap().0, t.1.unwrap().1))
        .next()
}

fn to_i64(s: &str) -> i64 {
    s.trim().parse().expect("expecting number")
}
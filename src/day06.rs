use std::hash::Hash;
use std::collections::HashSet;
use std::fs;

pub fn run() -> (usize, usize) {
    
    let input = fs::read_to_string("input/day6.txt").unwrap();

    let groups: Vec<&str> = input
        .split("\n\n")
        .collect();

    let part1 = groups
        .iter()
        .map(|group| count_questions_where_anyone_answered_yes(*group))
        .sum();

    let part2 = groups
        .iter()
        .map(|group| count_questions_where_everyone_answered_yes(*group))
        .sum();

    (
        part1, 
        part2
    ) 
}

fn count_questions_where_anyone_answered_yes(group: &str) -> usize {
    get_questions(group).len()
}

fn count_questions_where_everyone_answered_yes(group: &str) -> usize {
    let qs = group
        .lines()
        .fold(get_questions(group), |all, line| intersection(all, &get_questions(line)));

    qs.len()
}

fn get_questions(line: &str) -> HashSet<char> {
    line
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}

fn intersection<T: Eq + Hash>(a: HashSet<T>, b: &HashSet<T>) -> HashSet<T> {
    a.into_iter().filter(|e| b.contains(e)).collect()
}
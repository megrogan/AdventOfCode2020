use std::collections::HashMap;

pub fn run() -> (usize, usize) {

    let input = "8,11,0,19,1,2";
    let starting_numbers: Vec<usize> = input.split(",").map(|n| n.parse().unwrap()).collect();

    (
        solve(&starting_numbers, 2020),
        0//solve(&starting_numbers, 30000000),
    )
}

fn solve(starting_numbers: &Vec<usize>, n: usize) -> usize {

    let mut last_seen: HashMap<usize, usize> = starting_numbers
        .iter()
        .enumerate()
        .map(|(turn, number)| (*number, turn))
        .collect();

    let mut last = *starting_numbers.last().unwrap();
    let mut last_turn: Option<&usize> = None;

    for i in starting_numbers.len()..n {
        let new = match last_turn {
            None => 0,
            Some(lt) => (i - 1) - *lt
        };

        last_seen.insert(last, i - 1);
        last_turn = last_seen.get(&new);
        last = new;
    }

    last
}
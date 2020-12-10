use std::fs;

#[cfg(test)]
mod tests {
    use crate::day03::*;

    #[test]
    fn is_tree_at_position_expected() {
        let map_row = "..#..#......###.#...#......#..# ";

        let map_row = parse_input_line(map_row);

        assert!(!is_tree_at_position(map_row, 0));
        assert!(is_tree_at_position(map_row, 2));
    }
}

pub fn run() -> (u64, u64) {
    let contents = fs::read_to_string("input/day3.txt").expect("cannot open input file");

    let part1 = calculate_collisions(&contents, 3, 1);

    let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];    

    let part2 = slopes
        .iter()
        .map(|s| {
            let c = calculate_collisions(&contents, s.0, s.1);
            // println!("{}", c);
            c
        })
        .product();
    
    (part1, part2)
}

fn calculate_collisions(map: &str, right: u32, down: u32) -> u64 {
    map
        .lines()
        .map(parse_input_line)
        .enumerate()
        .filter(|(i, _)| ((*i as u32) % down) == 0)
        .filter(|(i, map_row)| is_tree_at_position(*map_row, ((*i as u32) * right) / down))
        .count() as u64
}

fn parse_input_line(line: &str) -> u32 {

    let bin_str: String = line
        .trim()
        .chars()
        .rev()
        .map(|c| if c == '#' { '1' } else { '0' })
        .collect();

    u32::from_str_radix(&bin_str, 2).unwrap()
}

fn is_tree_at_position(map_row: u32, pos: u32) -> bool {
    ((1_u32 << pos % 31) & map_row) != 0
}


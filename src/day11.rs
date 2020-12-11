use std::fs;

pub fn run() -> (usize, usize) {

    let input = fs::read_to_string("input/day11.txt").unwrap();

    let mut floor_plan = load_floor_plan(&input);

    (
        count_occupied_in_steady_state(&mut floor_plan),
        0
    )
}

fn count_occupied_in_steady_state(floor_plan: &mut Vec<Vec<Space>>) -> usize {

    for i in 0..99999 {
        let mut new_floor_plan = floor_plan.clone();

        if !play_round(&floor_plan, &mut new_floor_plan) {
            return count_occupied(&new_floor_plan);
        }

        *floor_plan = new_floor_plan;

        if i % 1000 == 0 {
            println!("{} iterations", i)
        }
    }

    panic!("Unstable floor plan!");
}

fn play_round(floor_plan: &Vec<Vec<Space>>, new_floor_plan: &mut Vec<Vec<Space>>) -> bool {

    fn adjacent_spaces(floor_plan: &Vec<Vec<Space>>, x: usize, y:usize) -> Vec<Space> {
        let mut seats = Vec::new();
        seats.push(floor_plan[y-1][x-1]);
        seats.push(floor_plan[y-1][x]);
        seats.push(floor_plan[y-1][x+1]);
        seats.push(floor_plan[y][x-1]);
        seats.push(floor_plan[y][x+1]);
        seats.push(floor_plan[y+1][x-1]);
        seats.push(floor_plan[y+1][x]);
        seats.push(floor_plan[y+1][x+1]);
        seats
    }

    fn adjacent_seat_is_occupied(floor_plan: &Vec<Vec<Space>>, x: usize, y:usize) -> bool {
        adjacent_spaces(floor_plan, x, y)
            .iter()
            .any(|space| *space == Space::Occupied)
    }

    fn more_than_3_adjacent_seats_occupied(floor_plan: &Vec<Vec<Space>>, x: usize, y:usize) -> bool {
        adjacent_spaces(floor_plan, x, y)
            .iter()
            .filter(|space| **space == Space::Occupied)
            .count() > 3
    }

    let mut has_changed = false;

    for x in 1..floor_plan[0].len()-1 {
        for y in 1..floor_plan.len()-1 {

            let space = floor_plan[y][x];

            let space = match space {
                Space::Floor => Space::Floor,
                Space::Empty => if adjacent_seat_is_occupied(&floor_plan, x, y) { Space::Empty } else { has_changed = true; Space::Occupied },
                Space::Occupied => if more_than_3_adjacent_seats_occupied(&floor_plan, x, y) { has_changed = true; Space::Empty } else { Space::Occupied }
            };

            new_floor_plan[y][x] = space;
        }
    }

    has_changed
}

fn count_occupied(floor_plan: &Vec<Vec<Space>>) -> usize {
    floor_plan
        .iter()
        .flatten()
        .filter(|s| **s == Space::Occupied)
        .count()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Space {
    Floor,
    Empty,
    Occupied
}

// Load the floor plan and include an outer rim of floor space
fn load_floor_plan(input: &str) -> Vec<Vec<Space>> {

    fn parse_line(line: &str) -> Vec<Space> {
        let mut row: Vec<Space> = line
            .trim()
            .chars()
            .map(|c| match c {
                'L' => Space::Empty,
                _ => Space::Floor
            })
            .collect();

        // Add extra floor at start and finish
        row.insert(0, Space::Floor);
        row.push(Space::Floor);

        row
    }

    let mut floor_plan: Vec<Vec<Space>> = input
        .lines()
        .map(parse_line)
        .collect();

    // Add extra floor at start and finish
    let width = floor_plan[0].len();
    floor_plan.insert(0, vec![Space::Floor; width]);
    floor_plan.push(vec![Space::Floor; width]);

    floor_plan
}

#[cfg(test)]
mod tests {
    use crate::day11::*;

    #[test]
    fn test_floor_plan() {
        let input = 
          r"L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL";

        let floor_plan = load_floor_plan(&input);

        assert_eq!(Space::Floor, floor_plan[1][2]);
        assert_eq!(Space::Empty, floor_plan[2][1]);
    }

    #[test]
    fn test_count_occupied_in_steady_state() {
        let input = 
          r"L.LL.LL.LL
            LLLLLLL.LL
            L.L.L..L..
            LLLL.LL.LL
            L.LL.LL.LL
            L.LLLLL.LL
            ..L.L.....
            LLLLLLLLLL
            L.LLLLLL.L
            L.LLLLL.LL";

        let mut floor_plan = load_floor_plan(&input);
        
        let result = count_occupied_in_steady_state(&mut floor_plan);

        assert_eq!(37, result);
    }
}
use std::fs;

pub fn run() -> (usize, usize) {

    let input = fs::read_to_string("input/day11.txt").unwrap();

    let mut floor_plan = load_floor_plan(&input);

    (
        count_occupied_in_steady_state(&mut floor_plan, &AdjacentPolicy{}),
        count_occupied_in_steady_state(&mut floor_plan, &VisiblePolicy{}),
    )
}

fn count_occupied_in_steady_state(floor_plan: &mut Vec<Vec<Space>>, seating_policy: &dyn SeatingPolicy) -> usize {

    for i in 0..99999 {
        let mut new_floor_plan = floor_plan.clone();

        if !play_round(&floor_plan, &mut new_floor_plan, seating_policy) {
            return count_occupied(&new_floor_plan);
        }

        *floor_plan = new_floor_plan;

        if i % 1000 == 0 {
            log::debug!("{} iterations", i);
        }
    }

    panic!("Unstable floor plan!");
}

fn play_round(
    floor_plan: &Vec<Vec<Space>>, 
    new_floor_plan: &mut Vec<Vec<Space>>, 
    seating_policy: &dyn SeatingPolicy) -> bool {

    let mut has_changed = false;

    for x in 1..floor_plan[0].len()-1 {
        for y in 1..floor_plan.len()-1 {

            let space = floor_plan[y][x];

            let space = match space {
                Space::Floor => Space::Floor,
                Space::Empty => 
                    if seating_policy.should_become_occupied(&floor_plan, x, y) { 
                        has_changed = true; 
                        Space::Occupied 
                    } else { 
                        Space::Empty 
                    },
                Space::Occupied => 
                    if seating_policy.should_become_empty(&floor_plan, x, y) { 
                        has_changed = true; 
                        Space::Empty 
                    } else { 
                        Space::Occupied 
                    }
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

// fn compare_floor_plans(fp1: &Vec<Vec<Space>>, fp2: &Vec<Vec<Space>>) -> bool {
//     let fp1 = fp1.iter().flatten();
//     let fp2 = fp2.iter().flatten();
//     fp1.zip(fp2).all(|(s1,s2)| *s1 == *s2)
// }

trait SeatingPolicy {
    fn should_become_empty(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool;
    fn should_become_occupied(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool;
}

struct AdjacentPolicy {}

impl AdjacentPolicy {
    fn adjacent_occupied_seats(floor_plan: &Vec<Vec<Space>>, x: usize, y:usize) -> usize {
        let directions = vec![
            (y-1, x-1),
            (y-1, x),
            (y-1, x+1),
            (y, x-1),
            (y, x+1),
            (y+1, x-1),
            (y+1, x),
            (y+1, x+1),
        ];
        directions
            .iter()
            .map(|d| floor_plan[d.0][d.1])
            .filter(|s| *s == Space::Occupied)
            .count()
    }    
}

impl SeatingPolicy for AdjacentPolicy {
    fn should_become_empty(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool {
        AdjacentPolicy::adjacent_occupied_seats(floor_plan, x, y) >= 4
    }
    fn should_become_occupied(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool {
        AdjacentPolicy::adjacent_occupied_seats(floor_plan, x, y) < 1
    }
}

struct VisiblePolicy {}

impl VisiblePolicy {
    fn first_seat_in_direction(floor_plan: &Vec<Vec<Space>>, mut x: usize, mut y:usize, dx: i8, dy: i8) -> Space {
        while x < floor_plan[0].len()-1 && y < floor_plan.len()-1 && x > 0 && y > 0 {
            x = (x as i32 + dx as i32) as usize;
            y = (y as i32 + dy as i32) as usize;
            let space = floor_plan[y][x];
            if space != Space::Floor {
                return space;
            }
        }
        Space::Floor
    }

    fn visible_occupied_seats(floor_plan: &Vec<Vec<Space>>, x: usize, y:usize) -> usize {

        let directions = vec![
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        directions
            .iter()
            .map(|d| VisiblePolicy::first_seat_in_direction(floor_plan, x, y, d.1, d.0))
            .filter(|s| *s == Space::Occupied)
            .count()
    }    
}

impl SeatingPolicy for VisiblePolicy {

    fn should_become_empty(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool {
        VisiblePolicy::visible_occupied_seats(floor_plan, x, y) >= 5
    }

    fn should_become_occupied(&self, floor_plan: &Vec<Vec<Space>>, x: usize, y: usize) -> bool {
        VisiblePolicy::visible_occupied_seats(floor_plan, x, y) < 1
    }
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
                '#' => Space::Occupied,
                '.' => Space::Floor,
                _ => panic!("unexpected character")
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
        
        let result = count_occupied_in_steady_state(&mut floor_plan, &AdjacentPolicy{});

        assert_eq!(37, result);
    }

    #[test]
    fn test_count_occupied_in_steady_state_with_visibilty_policy() {
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
        
        let result = count_occupied_in_steady_state(&mut floor_plan, &VisiblePolicy{});

        assert_eq!(26, result);
    }
}
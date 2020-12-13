use crate::day12::part1::Pose;
use std::fs;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("input/day12.txt").unwrap();
    let instructions = load_instructions(&input);
    let final_position_1 = part1::navigate(Pose::new(), &instructions);
    let final_position_2 = part2::Ship::new(part2::Waypoint::new(10, 1)).navigate(&instructions);

    (
        final_position_1.manhattan_distance(),
        final_position_2.manhattan_distance()
    )
}

mod part1 {
    use crate::day12::Point;
    use crate::day12::Instruction;
    use crate::day12::Action;

    #[derive(Debug, Copy, Clone)]
    pub enum Direction {
        North,
        East,
        South,
        West
    }
    
    impl Direction {
        fn rotate_left(&self, n: u32) -> Direction {
            let mut direction = self;
            for _ in 0..n {
                direction = match direction {
                    Direction::North => &Direction::West,
                    Direction::East => &Direction::North,
                    Direction::South => &Direction::East,
                    Direction::West => &Direction::South
                };
            }
            *direction
        }
    
        fn rotate_right(&self, n: u32) -> Direction {
            let mut direction = self;
            for _ in 0..n {
                direction = match direction {
                    Direction::North => &Direction::East,
                    Direction::East => &Direction::South,
                    Direction::South => &Direction::West,
                    Direction::West => &Direction::North
                };
            }
            *direction
        }
    }
    
    #[derive(Debug, Copy, Clone)]
    pub struct Pose {
        pub point: Point,
        pub direction: Direction
    }
    
    impl Pose {
        pub fn new() -> Pose {
            Pose {
                point: Point::empty(),
                direction: Direction::East
            }
        }
    }
    
    pub fn navigate(start: Pose, instructions: &Vec<Instruction>) -> Point {
        instructions
            .iter()
            .fold(start, |pose,instruction| move_ship(pose, *instruction))
            .point
    }
    
    fn move_ship(pose: Pose, instruction: Instruction) -> Pose {
    
        let mut new_pose = pose;
    
        match instruction.action {
            Action::North => new_pose.point.y += instruction.magnitude as i32,
            Action::East => new_pose.point.x += instruction.magnitude as i32,
            Action::South => new_pose.point.y -= instruction.magnitude as i32,
            Action::West => new_pose.point.x -= instruction.magnitude as i32,
            Action::Left => new_pose.direction = pose.direction.rotate_left(instruction.magnitude),
            Action::Right => new_pose.direction = pose.direction.rotate_right(instruction.magnitude),
            Action::Forward => new_pose = move_ship(
                pose, 
                Instruction {
                    action: match pose.direction {
                        Direction::North => Action::North,
                        Direction::East => Action::East,
                        Direction::South => Action::South,
                        Direction::West => Action::West,
                    },
                    magnitude: instruction.magnitude
                }),    
        };
        
        new_pose
    }
}

mod part2 {
    use crate::day12::Point;
    use crate::day12::Instruction;
    use crate::day12::Action;

    #[derive(Debug, Copy, Clone)]   
    pub struct Waypoint {
        x: i32,
        y: i32    
    }

    impl Waypoint {
        pub fn new(x: i32, y: i32) -> Waypoint {
            Waypoint {
                x: x,
                y: y
            }
        }

        fn move_north(&mut self, n: i32) {
            self.y += n;
        }

        fn move_east(&mut self, n: i32) {
            self.x += n;
        }

        fn move_south(&mut self, n: i32) {
            self.y -= n;
        }

        fn move_west(&mut self, n: i32) {
            self.x -= n;
        }

        fn rotate_left(&mut self, n: i32) {
            for _ in 0..n {
                let y = self.x;
                let x = -self.y;
                self.x = x;
                self.y = y;
            }
        }

        fn rotate_right(&mut self, n: i32) {
            for _ in 0..n {
                let y = -self.x;
                let x = self.y;
                self.x = x;
                self.y = y;
            }
        }
    }

    #[derive(Debug, Copy, Clone)]   
    pub struct Ship {
        position: Point,
        waypoint: Waypoint
    }

    impl Ship {
        pub fn new(waypoint: Waypoint) -> Ship {
            Ship {
                position: Point::empty(),
                waypoint: waypoint
            }
        }

        pub fn navigate(self, instructions: &Vec<Instruction>) -> Point {
            instructions
                .iter()
                .fold(self, |ship,instruction| ship.make_move(*instruction))
                .position
        }

        fn make_move(&self, instruction: Instruction) -> Ship {

            let mut ship = *self;
            let magnitude = instruction.magnitude as i32;

            match instruction.action {
                Action::North => ship.waypoint.move_north(magnitude),
                Action::East => ship.waypoint.move_east(magnitude),
                Action::South => ship.waypoint.move_south(magnitude),
                Action::West => ship.waypoint.move_west(magnitude),
                Action::Left => ship.waypoint.rotate_left(magnitude),
                Action::Right => ship.waypoint.rotate_right(magnitude),
                Action::Forward => ship.move_to_waypoint(magnitude),    
            }

            ship
        }

        fn move_to_waypoint(&mut self, magnitude: i32) {
            self.position.x += self.waypoint.x * magnitude;
            self.position.y += self.waypoint.y * magnitude;
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point {
    x: i32,
    y: i32
}

impl Point {
    pub fn empty() -> Point {
        Point {
            x: 0,
            y: 0
        }
    }

    pub fn manhattan_distance(&self) -> usize {
        (&self.x.abs() + &self.y.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Instruction {
    action: Action,
    magnitude: u32
}

#[derive(Debug, Copy, Clone)]
pub enum Action {
    North,
    East,
    South,
    West,
    Left,
    Right,
    Forward
}

impl Instruction {
    fn parse(text: &str) -> Instruction {

        let text = text.trim();
        let letter = text.chars().next().unwrap();
        let number = text[1..].parse().unwrap();

        let action = match letter {
            'F' => Action::Forward,
            'N' => Action::North,
            'E' => Action::East,
            'S' => Action::South,
            'W' => Action::West,
            'L' => Action::Left,
            'R' => Action::Right,
            _ => panic!("Unexpected action")
        };

        let magnitude = match action {
            Action::Left | Action::Right => number / 90,
            _ => number
        };

        Instruction {
            action: action, 
            magnitude: magnitude
        }
    }
}


fn load_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(Instruction::parse)
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::day12::*;

    #[test]
    fn test_navigate_part1() {
        let input = 
            r"F10
            N3
            F7
            R90
            F11";

        let instructions = load_instructions(&input);
        let final_position = part1::navigate(Pose::new(), &instructions);
    
        assert_eq!(25, final_position.manhattan_distance());
    }

    #[test]
    fn test_navigate_part2() {
        let input = 
            r"F10
            N3
            F7
            R90
            F11";

        let instructions = load_instructions(&input);
        let final_position = part2::Ship::new(part2::Waypoint::new(10, 1)).navigate(&instructions);
    
        assert_eq!(286, final_position.manhattan_distance());
    }
}
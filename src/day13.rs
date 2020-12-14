use std::fs;

pub fn run() -> (u32, usize) {
    let input = fs::read_to_string("input/day13.txt").unwrap();
    let notes = part1::Notes::parse(&input);
    let timetable = part2::Timetable::parse(&input);
    (
        notes.calculate(),
        timetable.find_earliest_matching_timestamp()
    )
}

mod part1 {
    pub struct Notes {
        timestamp: u32,
        buses: Vec<u32>
    }

    impl Notes {
        pub fn parse(input: &str) -> Notes {
            let mut lines = input.lines();
            Notes {
                timestamp: lines
                    .next()
                    .unwrap()
                    .trim()
                    .parse()
                    .unwrap(),
                buses: lines
                    .next()
                    .unwrap()
                    .split(",")
                    .map(|s| s.parse().unwrap_or(0))
                    .filter(|n| *n != 0)
                    .collect()
            }
        }

        pub fn calculate(&self) -> u32 {
            let earliest_bus = self
                .buses
                .iter()
                .map(|b| (b, b - (self.timestamp % b)))
                .min_by(|b1, b2| b1.1.cmp(&b2.1))
                .unwrap();

            earliest_bus.0 * earliest_bus.1
        }
    }
}

mod part2 {
    #[derive(Debug)]   
    pub struct Timetable {
        pub buses: Vec<Bus>
    }

    #[derive(Debug)]   
    pub struct Bus {
        id: usize,
        offset: usize
    }

    impl Timetable {
        pub fn parse(input: &str) -> Timetable {
            let mut lines = input.lines();
            lines.next();
            Timetable {
                buses: lines
                    .next()
                    .unwrap()
                    .split(",")
                    .enumerate()
                    .map(|(i, s)| Bus {id: s.parse().unwrap_or(0), offset: i})
                    .filter(|bus| bus.id != 0)
                    .collect()
            }
        }

        // As each successive bus id is matched, multiply the timestamp step (dt)
        // by that bus id
        pub fn find_earliest_matching_timestamp(&self) -> usize {
            let mut t = self.buses[0].id;
            let mut dt = t;
            for i in 1..self.buses.len() {
                while ((t + self.buses[i].offset) % self.buses[i].id) != 0 {
                    t += dt;
                }
                dt *= self.buses[i].id;
            }
            t
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::day13::*;
    
        #[test]
        fn test_find_earliest_matching_timestamp() {
            let input = "0\n7,13,x,x,59,x,31,19";
            let timetable = part2::Timetable::parse(&input);
    
            let result = timetable.find_earliest_matching_timestamp();
            
            assert_eq!(1068781, result);
        }
    }    
}


use crate::day16::puzzle::Puzzle;
use std::fs;

pub fn run() -> (u32, u64) {
    let input = fs::read_to_string("input/day16.txt").unwrap();
    let puzzle = Puzzle::parse(&input);
    (
        puzzle.calculate_ticket_scanning_error_rate(),
        puzzle.calculate_part2()
    )
}

mod puzzle {
    use bimap::BiMap;
    use std::ops::Range;

    #[derive(Debug)]   
    pub struct Puzzle {
        rules: Vec<TicketFieldRule>,
        my_ticket: Vec<u32>,
        nearby_tickets: Vec<Vec<u32>>
    }
    
    impl Puzzle {
        pub fn parse(input: &str) -> Puzzle {
            fn parse_rules(section: &str) -> Vec<TicketFieldRule>  {
                section
                    .lines()
                    .map(TicketFieldRule::parse_rule)
                    .collect()
            }

            fn parse_my_ticket(section: &str) -> Vec<u32> {
                section
                    .lines()
                    .skip(1)
                    .map(parse_ticket)
                    .next()
                    .unwrap()
            }

            fn parse_ticket(text: &str) -> Vec<u32> {
                text
                    .split(",")
                    .map(|s| s.parse().unwrap())
                    .collect()
            }

            fn parse_nearby_tickets(section: &str) -> Vec<Vec<u32>> {
                section
                    .lines()
                    .skip(1)
                    .map(parse_ticket)
                    .collect()
            }

            let mut sections = input.split("\n\n");

            Puzzle {
                rules: parse_rules(sections.next().unwrap()),
                my_ticket: parse_my_ticket(sections.next().unwrap()),
                nearby_tickets: parse_nearby_tickets(sections.next().unwrap()),
            }
        }

        pub fn calculate_ticket_scanning_error_rate(&self) -> u32 {
            self
                .nearby_tickets
                .iter()
                .map(|ticket| self.calculate_individual_ticket_scanning_error_rate(ticket))
                .sum()
        }

        pub fn calculate_part2(&self) -> u64 {
            // Compile list of valid tickets
            let mut valid_tickets: Vec<_> = self
                .nearby_tickets
                .iter()
                .filter(|ticket| self.is_ticket_valid(ticket))
                .collect();

            valid_tickets.push(&self.my_ticket);

            // Build a list of all numbers for each position
            let mut numbers_by_position: Vec<Vec<u32>> = Vec::new();
            for p in 0..self.rules.len() {
                let numbers: Vec<_>= valid_tickets
                    .iter()
                    .map(|ticket| ticket[p])
                    .collect();                
                numbers_by_position.push(numbers);
            }

            // Ticket position <-> rule index
            let mut identified_rules: BiMap<usize, usize> = BiMap::new();

            // Try repeatedly looping through position/rule combinations successively matching
            // rule to position where only 1 rule is valid
            while identified_rules.len() < self.rules.len() {

                let mut progress = false;

                for p in 0..self.my_ticket.len() {

                    if identified_rules.contains_left(&p) {
                        // skip position if we already have found a rule for this position
                        continue;
                    }

                    // Find all rules which match this position
                    let valid_rules: Vec<_> = self
                        .rules
                        .iter()
                        .enumerate()
                        .filter(|(i, _)| !identified_rules.contains_right(i))
                        .filter(|(_, r)| numbers_by_position[p]
                            .iter()
                            .all(|num| r.is_valid(*num))
                        )
                        .map(|(i, _)| i)
                        .collect();

                    // If only one rules matches this position then we have made progress!
                    if valid_rules.len() == 1 {
                        identified_rules.insert(p, valid_rules[0]);
                        log::debug!("Rule for position {} has index {}", p, valid_rules[0]);
                        progress = true;
                    }
                }

                for r in 0..self.rules.len() {
                    if identified_rules.contains_right(&r) {
                        // skip rule if we already have found a position for this rule
                        continue;
                    }

                    let rule = &self.rules[r];

                    // Find all the positions which mactch this rule
                    let matching_positions: Vec<usize> = (0..self.my_ticket.len())
                        .filter(|p| !identified_rules.contains_left(p))
                        .filter(|p| numbers_by_position[*p as usize]
                            .iter().
                            all(|num| rule.is_valid(*num))
                        )
                        .collect();

                    // If only one position matches this rule then we have made progress!
                    if matching_positions.len() == 1 {
                        identified_rules.insert(matching_positions[0], r);
                        log::debug!("Rule for position {} has index {}", matching_positions[0], r);
                        progress = true;
                    }
                }

                if !progress {
                    panic!("No progress made - aborting!!");
                }
            }

            self
                .rules
                .iter()
                .enumerate()
                .filter(|(_, r)| r.name.starts_with("departure"))
                .map(|(i, _)| identified_rules.get_by_right(&i).unwrap())
                .map(|p| self.my_ticket[*p as usize] as u64)
                .product()
        }

        fn calculate_individual_ticket_scanning_error_rate(&self, ticket: &Vec<u32>) -> u32 {
            ticket
                .iter()
                .filter(|n| !self.is_number_valid(**n))
                .sum()
        }

        fn is_ticket_valid(&self, ticket: &Vec<u32>) -> bool {
            ticket
                .iter()
                .all(|number| self.is_number_valid(*number))
        }

        fn is_number_valid(&self, number: u32) -> bool {
            self
                .rules
                .iter()
                .any(|r| r.is_valid(number))
        }
    }

    #[derive(Debug)]   
    struct TicketFieldRule {
        name: String,
        range1: Range<u32>,
        range2: Range<u32>,
    }

    impl TicketFieldRule {

        pub fn parse_rule(text: &str) -> TicketFieldRule {
            
            fn parse_range(text: &str) -> Range<u32> {
                let mut range = text.split("-");
                let from: u32 = range.next().unwrap().trim().parse().unwrap();
                let to: u32 = range.next().unwrap().trim().parse().unwrap();
                from..to+1
            }

            let mut parts = text.split(": ");
            let name = parts.next().unwrap().to_string();
            let mut ranges = parts.next().unwrap().split(" or ");

            TicketFieldRule {
                name: name,
                range1: parse_range(ranges.next().unwrap()),
                range2: parse_range(ranges.next().unwrap()),
            }
        }

        pub fn is_valid(&self, number: u32) -> bool {
            self.range1.contains(&number) || self.range2.contains(&number)
        }
    }
}
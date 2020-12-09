use std::fs;
use std::ops::Range;

pub fn run() -> (usize, usize) {
    
    let input = fs::read_to_string("input/day2.txt").expect("cannot open input file");

    let contents: Vec<(&str, &str)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split(":");
            (
                parts.next().unwrap().trim(),
                parts.next().unwrap().trim()
            )
        })
        .collect();

    (
        count_valid_passwords(&contents, PassportPolicyType::Sled), 
        count_valid_passwords(&contents, PassportPolicyType::Toboggan), 
    )
}

fn count_valid_passwords(password_data: &Vec<(&str, &str)>, policy_type: PassportPolicyType) -> usize {
    password_data
        .iter()
        .map(|(policy, password)| (policy_type.build_policy(policy), password))
        .filter(|(policy, password)| policy.validate(password))
        .count()
}

trait PasswordPolicy {
    fn new(policy: &str) -> Self where Self: Sized;
    fn validate(&self, password: &str) -> bool;
}

enum PassportPolicyType {
    Sled,
    Toboggan
}

impl PassportPolicyType {
    fn build_policy(&self, policy: &str) -> Box<dyn PasswordPolicy> {
        match self {
            PassportPolicyType::Sled => Box::new(SledPasswordPolicy::new(policy)),
            PassportPolicyType::Toboggan => Box::new(TobogganPasswordPolicy::new(policy))
        }
    }
}

struct SledPasswordPolicy {
    range: Range<usize>,
    letter: char
}

struct TobogganPasswordPolicy {
    index1: usize,
    index2: usize,
    letter: char
}

impl PasswordPolicy for SledPasswordPolicy {

    fn new(policy_text: &str) -> SledPasswordPolicy {

        fn parse_range(val: &str) -> Range<usize> {
            let mut parts = val.split('-');
    
            let min = parts.next().unwrap().parse().unwrap();
            let max: usize = parts.next().unwrap().parse().unwrap();
    
            min..(max + 1)
        }

        let mut parts = policy_text.split_whitespace();

        SledPasswordPolicy {
            range: parse_range(parts.next().unwrap()),
            letter: parts.next().unwrap().chars().next().unwrap()
        }
    }

    fn validate(&self, password: &str) -> bool {
        let char_count = password
            .chars()
            .filter(|c| *c == self.letter)
            .count();

        self.range.contains(&char_count)
    }
}

impl PasswordPolicy for TobogganPasswordPolicy {

    fn new(policy_text: &str) -> TobogganPasswordPolicy {

        fn parse_indexes(val: &str) -> (usize, usize) {
            let mut parts = val.split('-');
            (
                parts.next().unwrap().parse::<usize>().unwrap() - 1,
                parts.next().unwrap().parse::<usize>().unwrap() - 1
            )
        }

        let mut parts = policy_text.split_whitespace();
        let (index1, index2) = parse_indexes(parts.next().unwrap());

        TobogganPasswordPolicy {
            index1: index1,
            index2: index2,
            letter: parts.next().unwrap().chars().next().unwrap()
        }
    }   

    fn validate(&self, password: &str) -> bool {
        let first = password.chars().nth((self.index1) as usize).unwrap();
        let second = password.chars().nth((self.index2) as usize).unwrap();
        (first == self.letter) ^ (second == self.letter)
    }
}

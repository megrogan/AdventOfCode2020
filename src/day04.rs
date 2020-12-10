use std::collections::HashMap;
use std::fs;
use regex::Regex;

pub fn run() -> (usize, usize) {
    
    let input = fs::read_to_string("input/day4.txt").unwrap();

    (
        input.split("\n\n").into_iter().filter(|line| validate(line)).count(), 
        input.split("\n\n").into_iter().filter(|line| validate2(line)).count()
    ) 
}

fn validate(input: &str) -> bool {

    let fields: HashMap<&str, &str> = input
        .split_whitespace()
        .map(|word| {
            let mut field = word.split(":");
            (
                field.next().unwrap().trim(), 
                field.next().unwrap().trim(),
            )
        })
        .collect();

    let required_fields = [
        "byr",
        "iyr",
        "eyr",
        "hgt",
        "hcl",
        "ecl",
        "pid",
    ];

    for name in &required_fields {
        let value = fields.get(name);
        if value.is_none() {
            return false;
        }
    }

    true
}

fn validate2(input: &str) -> bool {

    let fields: HashMap<&str, &str> = input
        .split_whitespace()
        .map(|word| {
            let mut field = word.split(":");
            (
                field.next().unwrap().trim(), 
                field.next().unwrap().trim(),
            )
        })
        .collect();

    let required_fields: [(&str, Box<dyn Fn(&str)-> bool>); 7] = [
        ("byr", Box::new(|value: &str| is_u32_in_range(value, 1920, 2002))),
        ("iyr", Box::new(|value: &str| is_u32_in_range(value, 2010, 2020))),
        ("eyr", Box::new(|value: &str| is_u32_in_range(value, 2020, 2030))),
        ("hgt", Box::new(is_height)),
        ("hcl", Box::new(is_hair_color)),
        ("ecl", Box::new(is_eye_color)),
        ("pid", Box::new(is_passport_id)),
    ];

    for (name, validator) in &required_fields {
        let value = fields.get(name);
        if value.is_none() || !validator(&value.unwrap()) {
            return false;
        }
    }

    true
}

fn is_u32_in_range(value: &str, start: u32, end: u32) -> bool {
    let result = value.parse::<u32>();

    let result = match result {
        Ok(num) => num,
        Err(_) => return false
    };

    start <= result && result <= end 
}

fn is_height(value: &str) -> bool {

    lazy_static! {
        static ref RE1: Regex = Regex::new(r"^(\d{1,3})cm$").unwrap();
        static ref RE2: Regex = Regex::new(r"^(\d{1,3})in$").unwrap();
    }

    if let Some(caps1) = RE1.captures(value) {
        let a = caps1.get(1).unwrap().as_str().parse::<u32>().unwrap();
        return 150 <= a && a <= 193;
    }

    if let Some(caps2) = RE2.captures(value) {
        let a = caps2.get(1).unwrap().as_str().parse::<u32>().unwrap();
        return 59 <= a && a <= 76;

    } 

    false
}

fn is_hair_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();

    }
    RE.is_match(value)
}

fn is_eye_color(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    }
    RE.is_match(value)
}

fn is_passport_id(value: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    }
    RE.is_match(value)
}

// pub fn run2() -> usize {
//     let input = fs::read_to_string("input/day4.txt").expect("cannot open input file");
//     let lines: Vec<String> = input.split("\n\n").map(|l| l.parse().unwrap()).collect();

//     let byr_re = regex::Regex::new(r"byr:(19[2-9][0-9]|200[0-2])\b").unwrap();
//     let iyr_re = regex::Regex::new(r"iyr:(201[0-9]|2020)\b").unwrap();
//     let eyr_re = regex::Regex::new(r"eyr:(202[0-9]|2030)\b").unwrap();
//     let hgt_re = regex::Regex::new(r"hgt:(1[5-8][0-9]cm|19[0-3]cm|59in|6[0-9]in|7[0-6]in)\b").unwrap();
//     let hcl_re = regex::Regex::new(r"hcl:#[0-9a-f]{6}\b").unwrap();
//     let ecl_re = regex::Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
//     let pid_re = regex::Regex::new(r"pid:[0-9]{9}\b").unwrap();

//     let mut num_valid: usize = 0;

//     for i in 0..lines.len() {
//         let s = (&lines[i]).to_lowercase();
//         if  hgt_re.is_match(s.as_str()) &&
//             eyr_re.is_match(s.as_str()) &&
//             ecl_re.is_match(s.as_str()) &&
//             pid_re.is_match(s.as_str()) &&
//             hcl_re.is_match(s.as_str()) &&
//             byr_re.is_match(s.as_str()) &&
//             iyr_re.is_match(s.as_str())
//             {
//                 num_valid += 1;
//             }
//     }

//     num_valid
// }
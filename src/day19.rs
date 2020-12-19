
use crate::day19::Rule::*;
use std::fs;

pub fn run() -> (usize, u64) {
    let input = fs::read_to_string("input/day19.txt").unwrap();

    let (rule_set, messages) = parse(&input);

    let part1 = count_matches(rule_set, messages);

    (
        part1,
        0
    )
}

fn parse(input: &str) -> (RuleSet, Vec<String>) {

    let mut parts = input.split("\n\n");
 
    let rules = RuleSet::parse(parts.next().unwrap());

    let messages: Vec<String> = parts
        .next()
        .unwrap()
        .lines()
        .filter_map(|line| {
            let l = line.trim();
            match l.len() {
                0 => None,
                _ => Some(l.to_string())
            }
        })
        .collect();

    (
        rules,
        messages
    )
}

fn count_matches(rule_set: RuleSet, messages: Vec<String>) -> usize {
    messages
        .iter()
        .filter(|m| rule_set.is_match(m))
        .count()
}

#[derive(Debug)]   
enum Rule {
    Letter(char),
    Seq(Vec<usize>),
    Or(Box<Rule>, Box<Rule>)
}

#[derive(Debug)]   
struct RuleSet {
    rules: Vec<Rule>
}

impl RuleSet {
    fn parse(input: &str) -> RuleSet {
        
        let mut rules: Vec<(usize, Rule)> = input
            .lines()
            .filter_map(RuleSet::parse_line)
            .collect();

        rules.sort_unstable_by_key(|p| p.0);

        RuleSet {
            rules: rules.into_iter().map(|p| p.1).collect()
        }
    }

    fn is_match(&self, message: &str) -> bool {
        let result = self.solve(&self.rules[0], message);
        result.is_some() && result.unwrap().len() == 0
    }

    fn solve<'a>(&self, rule: &Rule, message: &'a str) -> Option<&'a str> {

        let s = message.len();
    
        if s == 0 {
            return Some(message);
        }
    
        match rule {
            Letter(l) => {
                if message.chars().nth(0).unwrap() == *l {
                    Some(&message[1..s])
                } else {
                    None
                }
            },
            Seq(v) => {
                let mut m = message;
                for i in v {
                    let r = &self.rules[*i];
                    let result = self.solve(r, m);
                    if result.is_none() {
                        return None;
                    }
                    m = result.unwrap();
                }
                Some(m)
            },
            Or(r1, r2) => {
    
                let result = self.solve(r1, message);
                if result.is_some() {
                    return result;
                } 
    
                self.solve(r2, message)
            }
        }
    }    

    fn parse_line(line: &str) -> Option<(usize, Rule)> {

        let line = line.trim();
        if line.len() == 0 {
            return None
        }

        let mut parts = line.split(":");
        Some((
            parts.next().unwrap().parse().unwrap(),
            RuleSet::parse_rule(parts.next().unwrap().trim())
        ))
    }

    fn parse_rule(text: &str) -> Rule {
        if text.contains("\"") {
            let letter = text.replace("\"", "").chars().next().unwrap();
            Rule::Letter(letter)
        } else if text.contains("|") {
            let mut parts = text.split("|");

            Rule::Or(
                Box::new(Rule::Seq(RuleSet::parse_seq(parts.next().unwrap()))),
                Box::new(Rule::Seq(RuleSet::parse_seq(parts.next().unwrap())))
            )
        } else {
            Rule::Seq(RuleSet::parse_seq(text))
        }
    }

    fn parse_seq(text: &str) -> Vec<usize> {
        text
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::day19::*;

    #[test]
    fn test_rules_match() {

        let input = r#"
0: 1 2
1: "a"
2: 1 3 | 3 1
3: "b"

aba
                "#;

        let (rule_set, messages) = parse(&input);
        let result = rule_set.is_match(&messages[0]);

        assert!(result);
    }
}
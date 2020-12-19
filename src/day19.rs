
use crate::day19::Rule::*;
use std::fs;

pub fn run() -> (usize, usize) {
    let input = fs::read_to_string("input/day19.txt").unwrap();
    let (rule_set, messages) = parse(&input);
    let part1 = count_matches(&rule_set, &messages);

    let input = fs::read_to_string("input/day19_part2.txt").unwrap();
    let (rule_set, messages) = parse(&input);
    let part2 = count_matches(&rule_set, &messages);

    (
        part1,
        part2
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

fn count_matches(rule_set: &RuleSet, messages: &Vec<String>) -> usize {
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
        
        fn parse_line(line: &str) -> Option<(usize, Rule)> {

            let line = line.trim();
            if line.len() == 0 {
                return None
            }
    
            let mut parts = line.split(":");
            Some((
                parts.next().unwrap().parse().unwrap(),
                parse_rule(parts.next().unwrap().trim())
            ))
        }
    
        fn parse_rule(text: &str) -> Rule {
            if text.contains("\"") {
                let letter = text.replace("\"", "").chars().next().unwrap();
                Rule::Letter(letter)
            } else if text.contains("|") {
                let mut parts = text.split("|");
    
                Rule::Or(
                    Box::new(Seq(parse_seq(parts.next().unwrap()))),
                    Box::new(Seq(parse_seq(parts.next().unwrap())))
                )
            } else {
                Rule::Seq(parse_seq(text))
            }
        }
    
        fn parse_seq(text: &str) -> Vec<usize> {
            text
                .trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        }
    
        let mut rules: Vec<(usize, Rule)> = input
            .lines()
            .filter_map(parse_line)
            .collect();

        rules.sort_unstable_by_key(|p| p.0);

        RuleSet {
            rules: rules.into_iter().map(|p| p.1).collect()
        }
    }

    fn is_match(&self, message: &str) -> bool {
        let result = self.solve(&self.rules[0], message, 0);
        result.is_some() && result.unwrap().len() == 0
    }

    fn solve<'a>(&self, rule: &Rule, message: &'a str, depth: usize) -> Option<&'a str> {

        if depth > 50 {
            println!("Too deep");
            return None;
        }

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
                    m = self.solve(r, m, depth + 1)?;
                }
                Some(m)
            },
            Or(r1, r2) => {
                let result = self.solve(r1, message, depth + 1);
                if result.is_some() { 
                    return result; 
                } 
                self.solve(r2, message, depth + 1)
            }
        }
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

    #[test]
    fn test_rules_2() {
        let input = r#"
42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
                "#;        

        let (rule_set, messages) = parse(&input);
        let result = count_matches(&rule_set, &messages);
        assert_eq!(3, result);
    }
}
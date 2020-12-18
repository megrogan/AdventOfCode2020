use std::fs;

pub fn run() -> (u64, u64) {
    let input = fs::read_to_string("input/day18.txt").unwrap();
    (
        evaluate_expressions(&input, false),
        evaluate_expressions(&input, true),
    )
}

fn evaluate_expressions(input: &str, with_precedence: bool) -> u64 {
    input
        .lines()
        .map(|line| evaluate(line, with_precedence))
        .sum()
}

fn evaluate(expression: &str, with_precedence: bool) -> u64 {

    fn apply_operator(value_stack: &mut Vec<u64>, operator: char) {
        let v1 = value_stack.pop().unwrap();
        let v2 = value_stack.pop().unwrap();
        match operator {
            '+' => value_stack.push(v1 + v2),
            '*' => value_stack.push(v1 * v2),
            _ => panic!("Unexpected operator {}", operator)
        }
    }

    let expression = expression.replace(" ", "");

    let mut value_stack: Vec<u64> = Vec::new();
    let mut operator_stack: Vec<char> = Vec::new();

    for c in expression.chars() {
        match c {
            '0'..='9' => value_stack.push(c.to_digit(10).unwrap() as u64),
            '(' => operator_stack.push('('),
            ')' => {
                while let Some(o) = operator_stack.pop() {
                    if o != '+' && o != '*' {
                        break;
                    }
                    apply_operator(&mut value_stack, o);
                }
            },
            '+' | '*' => {
                while let Some(o) = operator_stack.last() {
                    if *o != '+' && *o != '*' {
                        break;
                    }
                    if with_precedence && c == '+' && *o == '*' {
                        break;
                    }
                    let o = operator_stack.pop().unwrap();
                    apply_operator(&mut value_stack, o);
                }
                operator_stack.push(c);
            },
            _ => ()
        }
    }

    while let Some(o) = operator_stack.pop() {
        apply_operator(&mut value_stack, o);
    };

    value_stack.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day18::*;

    #[test]
    fn test_evaluate_expressions() {
        let expressions = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 71),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 26),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 437),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 12240),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 13632),
        ];

        for e in expressions {
            let value = evaluate(e.0, false);
            assert_eq!(e.1, value);
        }
    }

    #[test]
    fn test_evaluate_expressions_with_precedence() {
        let expressions = vec![
            ("1 + 2 * 3 + 4 * 5 + 6", 231),
            ("1 + (2 * 3) + (4 * (5 + 6))", 51),
            ("2 * 3 + (4 * 5)", 46),
            ("5 + (8 * 3 + 9 + 3 * 4 * 3)", 1445),
            ("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", 669060),
            ("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", 23340),
        ];

        for e in expressions {
            let value = evaluate(e.0, true);
            assert_eq!(e.1, value);
        }
    }
}

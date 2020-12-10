use std::collections::HashSet;
use std::fs;

pub fn run() -> (i64, i64) {

    let input = fs::read_to_string("input/day9.txt").unwrap();
    
    let numbers: Vec<i64> = input
        .split_whitespace()
        .map(|word| word.trim()
        .parse()
        .unwrap())
        .collect();

    let anomaly = find_xmas_anomaly(&numbers, 25).unwrap();
    let encryption_weakness = find_xmas_encryption_weakness(&numbers, anomaly).unwrap();

    (
        anomaly, 
        encryption_weakness
    )
}

fn find_xmas_anomaly(numbers: &Vec<i64>, window_size: usize) -> Option<i64> {

    fn is_nth_valid(n: i64, prev_x: &HashSet<i64>) -> bool {
        prev_x
            .iter()
            .any(|x| prev_x.contains(&(n - *x)))
    }

    
    let mut prev_x: HashSet<i64> = numbers[0..window_size]
        .iter()
        .map(|x| *x)
        .collect();

    for nth in window_size..numbers.len() {
        
        let n = numbers[nth];

        if !is_nth_valid(n, &prev_x) {
            return Some(n);
        }

        prev_x.remove(&numbers[nth - window_size]);
        prev_x.insert(n);
    }

    None
}

fn find_xmas_encryption_weakness(numbers: &Vec<i64>, anomaly: i64) -> Option<i64> {

    for i in 0..numbers.len() {

        let mut sum = 0_i64;

        for j in i..numbers.len() {
            sum += numbers[j];

            if sum == anomaly {
                let range = &numbers[i..j];
                let min = range.iter().min().unwrap();
                let max = range.iter().max().unwrap();
                return Some(min + max);
            }
        }
    }

    None
} 

#[cfg(test)]
mod tests {
    use crate::day09::*;

    #[test]
    fn test_find_xmas_anomaly() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];

        let result = find_xmas_anomaly(&input, 5);

        assert_eq!(127, result.unwrap());
    }

    #[test]
    fn test_find_encryption_weakness() {
        let input = vec![35,20,15,25,47,40,62,55,65,95,102,117,150,182,127,219,299,277,309,576];

        let result = find_xmas_encryption_weakness(&input, 127);

        assert_eq!(62, result.unwrap());
    }    
}
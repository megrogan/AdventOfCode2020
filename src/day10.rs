use std::fs;

pub fn run() -> (usize, usize) {

    let input = fs::read_to_string("input/day10.txt").unwrap();

    let adaptors = get_sorted_adaptors(&input);

    let diffs = count_differences(&adaptors, 3);
    let combinations = count_valid_combinations(&adaptors);

    (
        diffs[1] * diffs[3], 
        combinations
    )
}

fn get_sorted_adaptors(input: &str) -> Vec<usize> {

    let mut adaptors: Vec<usize> = input
        .split_whitespace()
        .map(|word| word.trim().parse().unwrap())
        .collect();    

    // Add the "charging outlet" to the end of the list of adaptors
    // It will be sorted to the front in the following step
    adaptors.push(0);

    // Sort the adaptors from low to high
    adaptors.sort_unstable();

    // Add the "built-in adaptor" to the end of the list
    adaptors.push(*adaptors.last().unwrap() + 3_usize);

    adaptors
}

// This function allows for adaptors to work with a given max difference in joltage
// rather than being hard-coded to 3. It also counts all dfferences not assuming they 
// will always be 1s or 3s.
fn count_differences(adaptors: &Vec<usize>, max_diff: usize) -> Vec<usize> {
    let mut results = vec![0; max_diff + 1];
    for window in adaptors.windows(2) {
        let diff = window[1] - window[0];
        results[diff] += 1;
    }
    results
}

// This function calculates the number of combinations mathematically. The observation 
// is that when ordered, adjacent adaptors either differ by 1 jolt or by 3. 
// Considering the differences in joltage between adjacent adaptors they form sequences
// of 1s and 3s. Any sequence of 3s can be ignored because they do not allow for multiple
// combinations. Likewise sequences of a single 1 do not give multiple combinations.
// However longer sequences of 1s do produce multiple combinations:
// 1,1 -> 2
// 1,1,1 -> 4
// 1,1,1,1 -> 7
// 1,1,1... -> 1 + (((n - 1) * n) / 2)
// We thus multiply together the combinations due to each sequence of 1s to find the total
// number of combinations of adaptors
fn count_valid_combinations(nums: &[usize]) -> usize {

    fn combinations(n: usize) -> usize {
        1 + (((n - 1) * n) / 2)
    }

    nums
        .windows(2)
        .collect::<Vec<_>>()
        .split(|n| n[1] - n[0] == 3)
        .filter(|n| n.len() > 0)
        .map(|n| combinations(n.len()))
        .product::<usize>()    
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    #[test]
    fn test_count_differences() {
        let input = "16 10 15 5 1 11 7 19 6 12 4";
        assert_eq!(35, test_part1(input));
    }

    #[test]
    fn test_count_differences_long() {
        let input = "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";
        assert_eq!(220, test_part1(input));

    }

    fn test_part1(input: &str) -> usize {
        let adaptors = get_sorted_adaptors(input);
        let diffs = count_differences(&adaptors, 3);
        return diffs[1] * diffs[3];
    }

    #[test]
    fn test_count_valid_combinations() {
        let input = "16 10 15 5 1 11 7 19 6 12 4";
        assert_eq!(8, test_part2(input));        
    }

    #[test]
    fn test_count_valid_combinations_long() {
        let input = "28 33 18 42 31 14 46 20 48 47 24 23 49 45 19 38 39 11 1 32 25 35 8 17 7 9 4 2 34 10 3";
        assert_eq!(19208, test_part2(input));        
    }

    fn test_part2(input: &str) -> usize {
        let adaptors = get_sorted_adaptors(input);
        count_valid_combinations(&adaptors)
    }
}
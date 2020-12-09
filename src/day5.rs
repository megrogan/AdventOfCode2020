use itertools::sorted;
use std::fs;


#[cfg(test)]
mod tests {
    use crate::day5::*;

    #[test]
    fn test1() {
        let boarding_pass = "BFFFBBFRRR";
        assert_eq!(567, calculate_seat_id(&boarding_pass));
    }

    #[test]
    fn test2() {
        let boarding_pass = "FFFBBBFRRR";
        assert_eq!(119, calculate_seat_id(&boarding_pass));
    }

    #[test]
    fn test3() {
        let boarding_pass = "BBFFBBFRLL";
        assert_eq!(820, calculate_seat_id(&boarding_pass));
    }

    #[test]
    fn test_find_my_seat_1() {
        let seats = vec![10, 11, 12, 13, 15, 16, 17];

        assert_eq!(14, find_my_site_id(seats));
    }

    #[test]
    fn test_find_my_seat_2() {
        let seats = vec![10, 11, 12, 13, 16, 17, 19];

        assert_eq!(18, find_my_site_id(seats));
    }
}

pub fn run() -> (u32, u32) {
    let input = fs::read_to_string("input/day5.txt").unwrap();

    (
        calculate_max_site_id(&input),
        find_my_site_id_from_boarding_passes(&input)
    )
}

fn calculate_max_site_id(input: &str) -> u32 {
    input.lines().map(calculate_seat_id).max().unwrap()
}

fn find_my_site_id_from_boarding_passes(input: &str) -> u32 {

    let ordered_seat_ids = sorted(input.lines().map(calculate_seat_id));

    find_my_site_id(ordered_seat_ids)
}

fn find_my_site_id(seats: Vec<u32>) -> u32 {

    let mut i = *seats.first().unwrap();

    for seat in seats {

        if seat == i {
            i += 1;
        }
        else if (seat - i) == 1 {
            return i;
        }
        else {
            i = seat + 1;
        }
    }

    0  
}

fn calculate_seat_id(pass: &str) -> u32 {

    let bin_str = pass
        .replace("B", "1")
        .replace("R", "1")
        .replace("F", "0")
        .replace("L", "0");

    u32::from_str_radix(&bin_str, 2).unwrap()
}
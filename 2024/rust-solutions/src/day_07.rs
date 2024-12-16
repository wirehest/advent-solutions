use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(raw_input: &str) -> u64 {
    let equations = parse_input(raw_input);

    let total_calibration: u64 = equations
        .iter()
        .filter(|equation| {
            let target = equation.0;
            let numbers = &equation.1;
            rec_calibrate(target, numbers[0], &numbers[1..])
        })
        .map(|true_equation| true_equation.0)
        .sum();

    println!("total calibration: {total_calibration:?}");
    total_calibration
}

pub fn part_two(raw_input: &str) -> u64 {
    let equations = parse_input(raw_input);

    let total_calibration: u64 = equations
        .iter()
        .filter(|equation| {
            let target = equation.0;
            let numbers = &equation.1;
            rec_calibrate_with_concat(target, numbers[0], &numbers[1..])
        })
        .map(|true_equation| true_equation.0)
        .sum();

    println!("total calibration: {total_calibration:?}");
    total_calibration
}

pub fn rec_calibrate(target: u64, lhs: u64, rhs: &[u64]) -> bool {
    if rhs.len() == 1 {
        lhs * rhs[0] == target || lhs + rhs[0] == target
    } else {
        rec_calibrate(target, lhs * rhs[0], &rhs[1..])
            || rec_calibrate(target, lhs + rhs[0], &rhs[1..])
    }
}

pub fn rec_calibrate_with_concat(target: u64, lhs: u64, rhs: &[u64]) -> bool {
    if rhs.len() == 1 {
        lhs * rhs[0] == target || lhs + rhs[0] == target || concat(lhs, rhs[0]) == target
    } else {
        rec_calibrate_with_concat(target, lhs * rhs[0], &rhs[1..])
            || rec_calibrate_with_concat(target, lhs + rhs[0], &rhs[1..])
            || rec_calibrate_with_concat(target, concat(lhs, rhs[0]), &rhs[1..])
    }
}

pub fn concat(lhs: u64, rhs: u64) -> u64 {
    format!("{}{}", lhs, rhs).parse().expect("numeric return")
}

pub fn parse_input(raw_input: &str) -> Vec<(u64, Vec<u64>)> {
    raw_input
        .lines()
        .map(|line| {
            let equation = line.split(": ").collect::<Vec<_>>();
            let test_value: u64 = equation[0].trim().parse().unwrap();
            let numbers: Vec<u64> = equation[1].split(' ').map(|n| n.parse().unwrap()).collect();
            (test_value, numbers)
        })
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day_07.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn sample_part_one() {
        assert_eq!(3749, part_one(SAMPLE_INPUT));
    }

    #[test]
    fn sample_part_two() {
        assert_eq!(11387, part_two(SAMPLE_INPUT));
    }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        part_one(&raw_input);
    }

    #[test]
    fn actual_part_two() {
        let raw_input = get_input().unwrap();
        part_two(&raw_input);
    }
}

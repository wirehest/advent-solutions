use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(parsed_input: Vec<Vec<i32>>) {
    let deltas = parsed_input
        .iter()
        .map(|report| {
            report
                .windows(2)
                .map(|level| level[0] - level[1])
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let counts = deltas
        .iter()
        .map(|diffs| {
            let less_than_4 = diffs.iter().all(|diff| diff.abs() < 4);
            let first_sign = diffs[0].signum();
            let incr_xor_decr = diffs.iter().all(|&diff| diff.signum() == first_sign);
            less_than_4 && incr_xor_decr
        })
        .filter(|safety| *safety)
        .count();

    println!("safe reports (part one): {counts}");
}

pub fn part_two(parsed_input: Vec<Vec<i32>>) {
    let counts = parsed_input
        .iter()
        .map(|report| {
            // make vec of all report permutations (i.e., dampened)
            let length = report.len();
            let mut dampened_reports = vec![report.clone()];

            (0..length).for_each(|i| {
                let left = &report[0..i];
                let right = &report[i + 1..length];
                dampened_reports.push([left, right].concat().to_vec());
            });

            // calculate differences
            let deltas = dampened_reports
                .iter()
                .map(|report| {
                    report
                        .windows(2)
                        .map(|level| level[0] - level[1])
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();

            // apply rules to the differences
            let safe_permutations = deltas
                .iter()
                .map(|diffs| {
                    let less_than_4 = diffs.iter().all(|diff| diff.abs() < 4 && diff.abs() != 0);
                    let first_sign = diffs[0].signum();
                    let incr_xor_decr = diffs.iter().all(|&diff| diff.signum() == first_sign);
                    less_than_4 && incr_xor_decr
                })
                .collect::<Vec<_>>();
            safe_permutations.iter().any(|p| *p)
            // safe_permutations
        })
        .filter(|report| *report)
        .count();

    println!("safe reports (part two): {:?}", counts);
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<i32>> {
    raw_input
        .lines()
        .map(|substring| {
            substring
                .split_whitespace()
                .map(|char| char.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn reader() -> io::Result<String> {
    let mut file = File::open("data/day02.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str =
            "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let parsed_input = parse_input(SAMPLE_INPUT);
        part_one(parsed_input);
    }

    #[test]
    fn sample_part_two() {
        const SAMPLE_INPUT: &str =
            "7 6 4 2 1\n1 2 7 8 9\n9 7 6 2 1\n1 3 2 4 5\n8 6 4 4 1\n1 3 6 7 9";
        let parsed_input = parse_input(SAMPLE_INPUT);
        part_two(parsed_input);
    }

    #[test]
    fn actual_part_one() {
        let raw_input = reader().unwrap();
        let parsed_input = parse_input(&raw_input);
        part_one(parsed_input);
    }

    #[test]
    fn actual_part_two() {
        let raw_input = reader().unwrap();
        let parsed_input = parse_input(&raw_input);
        part_two(parsed_input);
    }
}

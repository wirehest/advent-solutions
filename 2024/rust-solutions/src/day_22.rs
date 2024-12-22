// Standard

// Local
use crate::Part;

// External

const PUZZLE_INPUT: &str = include_str!("../data/day_22.txt");

pub fn solve(raw_input: &str, part: &Part) -> u64 {
    let initial_secrets = parse(raw_input);

    match part {
        Part::One => initial_secrets
            .iter()
            .map(|secret| (0..2000).fold(*secret, |acc, _| evolve(acc)))
            .sum(),
        Part::Two => todo!(),
    }
}

pub fn parse(raw_input: &str) -> Vec<u64> {
    raw_input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn evolve(mut secret: u64) -> u64 {
    secret = mix(secret, (secret * 64));
    secret = prune(secret);
    secret = mix(secret, (secret / 32));
    secret = prune(secret);
    secret = mix(secret, (secret * 2048));
    secret = prune(secret);
    secret
}

fn mix(secret: u64, given: u64) -> u64 {
    given ^ secret
}

fn prune(secret: u64) -> u64 {
    secret % 16777216
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        const SAMPLE_INPUT: &str = "1
10
100
2024";
        let solved = solve(SAMPLE_INPUT, &Part::One);
        println!("solution: {:?}", solved);
        assert_eq!(37327623, solved);
    }

    // #[test]
    // fn part2_sample() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     let solved = solve(SAMPLE_INPUT, &Part::Two);
    //     println!("solution: {:?}", solved);
    //     assert_eq!((), solved);
    // }

    #[test]
    fn part1_actual() {
        let solved = solve(PUZZLE_INPUT, &Part::One);
        println!("solution: {:?}", solved);
        // assert_eq!((), solved);
    }

    // #[test]
    // fn part2_actual() {
    //     let solved = solve(PUZZLE_INPUT, &Part::Two);
    //     println!("solution: {:?}", solved);
    //     assert_eq!((), solved);
    // }
}

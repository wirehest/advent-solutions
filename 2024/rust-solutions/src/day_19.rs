// Standard

// Local
use crate::Part;

// External

const PUZZLE_INPUT: &str = include_str!("../data/day_19.txt");

pub fn solve(raw_input: &str, part: &Part) -> Option<usize> {
    let (patterns, towels) = parse(raw_input)?;

    match part {
        Part::One => Some(
            patterns
                .iter()
                .filter(|pattern| recursive_starts_with(pattern, &towels))
                .count(),
        ),
        Part::Two => {
            todo!();
        }
    }
}

pub fn recursive_starts_with(substring: &str, towels: &[&str]) -> bool {
    if substring.is_empty() {
        return true;
    }

    for towel in towels {
        if substring.starts_with(towel) {
            let trimmed = substring
                .get(towel.len()..)
                .expect("should receive non-empty slice");
            if recursive_starts_with(trimmed, towels) {
                return true;
            }
        }
    }

    false
}

pub fn parse(raw_input: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    let (towels_str, patterns_str) = raw_input.split_once("\n\n")?;
    let towels: Vec<_> = towels_str.split(", ").collect();
    let patterns: Vec<_> = patterns_str.lines().collect();

    Some((patterns, towels))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1_sample() {
        let solved = solve(SAMPLE_INPUT, &Part::One).expect("should receive Option<usize>");
        println!("solution: {:?}", solved);
        assert_eq!(6, solved);
    }

    #[test]
    fn part2_sample() {
        let solved = solve(SAMPLE_INPUT, &Part::Two).expect("should receive Option<usize>");
        println!("solution: {:?}", solved);
        assert_eq!(16, solved);
    }

    #[test]
    fn part1_actual() {
        let solved = solve(PUZZLE_INPUT, &Part::One).expect("should receive Option<usize>");
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

// Standard

// Local
use crate::Part;

// External
use regex::Regex;

const PUZZLE_INPUT: &str = include_str!("../data/day_04.txt");

pub fn solve(raw_input: &str, part: &Part) -> i64 {
    let parsed = parse_input(raw_input);

    match part {
        Part::One => part_one(&parsed),
        Part::Two => part_two(&parsed),
    }
}

pub fn part_one(parsed: &[Vec<char>]) -> i64 {
    let needle = "XMAS";
    let needle_rev: String = needle.chars().rev().collect();

    let rows = to_rows(parsed);
    let columns = to_columns(parsed);
    let diagonals = to_diagonals(parsed);

    let appearances = [rows, columns, diagonals]
        .iter()
        .map(|direction| {
            direction
                .iter()
                .map(|line| line.matches(needle).count() + line.matches(&needle_rev).count())
                .sum::<usize>() as i64
        })
        .sum();

    appearances
}

fn to_rows(parsed: &[Vec<char>]) -> Vec<String> {
    parsed
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn to_columns(parsed: &[Vec<char>]) -> Vec<String> {
    let height = parsed.len();
    let width = parsed[0].len();

    (0..width)
        .map(|y| (0..height).map(|x| parsed[x][y]).collect::<String>())
        .collect::<Vec<_>>()
}

fn to_diagonals(parsed: &[Vec<char>]) -> Vec<String> {
    let height = parsed.len();
    let width = parsed[0].len();
    let mut start_ne = Vec::new();
    let mut start_ns = Vec::new();
    let mut diagonals = Vec::new();

    for y in 0..height {
        start_ne.push((y, 0));
        start_ns.push((y, 0));
    }

    for x in 1..width {
        start_ne.push((height - 1, x));
        start_ns.push((0, x));
    }

    for (mut y, mut x) in start_ne.iter() {
        let mut new_diagonal = Vec::new();
        loop {
            new_diagonal.push(parsed[y][x]);

            if y.checked_sub(1).is_some() {
                y -= 1;
            } else {
                break;
            }

            if parsed[y].get(x + 1).is_some() {
                x += 1;
            } else {
                break;
            }
        }
        let diagonal_string: String = new_diagonal.iter().collect();
        if diagonal_string.len() >= 4 {
            diagonals.push(diagonal_string);
        }
    }

    for (mut y, mut x) in start_ns.iter() {
        let mut new_diagonal = Vec::new();
        loop {
            new_diagonal.push(parsed[y][x]);

            if parsed.get(y + 1).is_some() {
                y += 1;
            } else {
                break;
            }

            if parsed[y].get(x + 1).is_some() {
                x += 1;
            } else {
                break;
            }
        }
        let diagonal_string: String = new_diagonal.iter().collect();
        if diagonal_string.len() >= 4 {
            diagonals.push(diagonal_string);
        }
    }

    diagonals
}

pub fn part_two(parsed: &[Vec<char>]) -> i64 {
    let height = parsed.len();
    let width = parsed[0].len();
    assert!(height > 1 && width > 1);

    let mut count = 0;
    let re = Regex::new(r"M.M.A.S.S|S.S.A.M.M|S.M.A.S.M|M.S.A.M.S").unwrap();

    for y in 1..height - 1 {
        for x in 1..width - 1 {
            let top: String = parsed[y - 1][x - 1..=x + 1].iter().collect();
            let mid: String = parsed[y][x - 1..=x + 1].iter().collect();
            let bot: String = parsed[y + 1][x - 1..=x + 1].iter().collect();
            let window = format!("{top}{mid}{bot}");

            if re.is_match(&window) {
                count += 1;
            }
        }
    }

    count
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        const SAMPLE_INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let solved = solve(SAMPLE_INPUT, &Part::One);
        println!("solution: {:?}", solved);
        assert_eq!(18, solved);
    }

    #[test]
    fn part2_sample() {
        const SAMPLE_INPUT: &str = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let solved = solve(SAMPLE_INPUT, &Part::Two);
        println!("solution: {:?}", solved);
        assert_eq!(9, solved);
    }

    #[test]
    fn part1_actual() {
        let solved = solve(PUZZLE_INPUT, &Part::One);
        println!("solution: {:?}", solved);
        assert_eq!(2662, solved);
    }

    #[test]
    fn part2_actual() {
        let solved = solve(PUZZLE_INPUT, &Part::Two);
        println!("solution: {:?}", solved);
        assert_eq!(2034, solved);
    }
}

use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(needle: &str, raw_input: &str) -> u32 {
    let parsed = parse_input(raw_input);
    let needle_rev: String = needle.chars().rev().collect();

    let rows = to_rows(&parsed);
    let columns = to_columns(&parsed);
    let diagonals = to_diagonals(&parsed);

    let appearances = [rows, columns, diagonals]
        .iter()
        .map(|direction| {
            direction
                .iter()
                .map(|line| line.matches(needle).count() + line.matches(&needle_rev).count())
                .sum::<usize>() as u32
        })
        .sum();

    println!("appearances: {}", appearances);
    appearances
}

pub fn part_two() {
    todo!()
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

pub fn parse_input(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .trim()
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day04.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;
    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let needle = "XMAS";
        assert_eq!(18, part_one(needle, SAMPLE_INPUT));
    }

    // #[test]
    // fn sample_part_two() {
    //     let parsed_input = parse_input(SAMPLE_INPUT);
    //     part_two(parsed_input);
    // }

    #[test]
    fn actual_part_one() {
        let needle = "XMAS";
        let raw_input = get_input().unwrap();
        part_one(needle, &raw_input);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     let parsed_input = parse_input(&raw_input);
    //     part_two(parsed_input);
    // }
}

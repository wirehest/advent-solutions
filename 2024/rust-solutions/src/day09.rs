use std::fmt::Display;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(raw_input: &str) -> u64 {
    let mut parsed = parse_input(raw_input);
    let compacted = compact(&mut parsed);
    let checksum = checksum(&compacted);

    println!("p1 checksum: {checksum}");
    checksum
}

pub fn part_two(raw_input: &str) {
    todo!()
}

pub fn checksum(compacted: &[u64]) -> u64 {
    compacted
        .iter()
        .enumerate()
        .map(|(i, file_id)| *file_id * i as u64)
        .sum()
}

pub fn compact(parsed_input: &mut [Option<u64>]) -> Vec<u64> {
    let free_indices: Vec<_> = parsed_input
        .iter()
        .enumerate()
        .filter(|(i, file)| file.is_none())
        .map(|(i, _)| i)
        .collect();
    let mut file_indices: Vec<_> = parsed_input
        .iter()
        .enumerate()
        .filter(|(i, file)| file.is_some())
        .map(|(i, _)| i)
        .collect();

    for i in free_indices.iter() {
        let end_file = file_indices.pop().expect("expected index");

        if i > &end_file {
            break;
        }

        parsed_input.swap(*i, end_file);
    }

    parsed_input.iter().filter_map(|file| *file).collect()
}

pub fn parse_input(raw_input: &str) -> Vec<Option<u64>> {
    raw_input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, ch)| {
            let count = ch.to_digit(10).expect("expected numeric char") as usize;
            let id = match i % 2 {
                0 => Some((i / 2) as u64),
                _ => None,
            };
            vec![id; count]
        })
        .collect()
}

fn make_string<T: Display>(files: &[Option<T>]) -> String {
    files
        .iter()
        .map(|elem| match elem {
            Some(val) => val.to_string(),
            None => String::from("."),
        })
        .collect()
}

fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day09.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str = "2333133121414131402";
        assert_eq!(1928, part_one(SAMPLE_INPUT));
    }

    // #[test]
    // fn sample_part_two() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     part_two(SAMPLE_INPUT);
    // }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        part_one(&raw_input);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_two(raw_input);
    // }
}

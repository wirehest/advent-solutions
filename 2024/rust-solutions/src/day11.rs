use std::fs::File;
use std::io::{self, prelude::*};
use std::mem;

pub fn part_one(raw_input: &str, blink_count: u8) -> String {
    let stones: Vec<String> = parse_input(raw_input);
    let blinked: Vec<String> = blink(&stones, &blink_count);

    // println!("{:?}", stones);
    println!("stone count: {:?}", blinked.len());
    blinked.join(" ")
}

pub fn part_two(raw_input: &str) {
    todo!()
}

pub fn blink(stones: &[String], blink_count: &u8) -> Vec<String> {
    let mut stones = stones.to_vec();
    let mut transformed = Vec::new();

    (0..*blink_count).for_each(|_| {
        stones.iter().for_each(|stone| match stone.as_str() {
            "0" => transformed.push(String::from("1")),
            engraving if engraving.len() % 2 == 0 => {
                let (h1, h2) = engraving.split_at(engraving.len() / 2);
                transformed.append(&mut vec![
                    h1.parse::<u64>().unwrap().to_string(),
                    h2.parse::<u64>().unwrap().to_string(),
                ])
            }
            _ => {
                transformed.push((stone.parse::<u64>().unwrap() * 2024).to_string());
            }
        });

        stones = transformed.to_owned();
        transformed.clear();
    });

    stones
}

pub fn parse_input(raw_input: &str) -> Vec<String> {
    raw_input
        .split_whitespace()
        .map(|stone| stone.to_owned())
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day11.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample1() {
        const SAMPLE_INPUT: &str = "0 1 10 99 999";
        assert_eq!("1 2024 1 0 9 9 2021976", part_one(SAMPLE_INPUT, 1));
    }

    #[test]
    fn part1_sample2() {
        const SAMPLE_INPUT: &str = "125 17";
        assert_eq!("253000 1 7", part_one(SAMPLE_INPUT, 1));
        assert_eq!("253 0 2024 14168", part_one(SAMPLE_INPUT, 2));
        assert_eq!(
            "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2",
            part_one(SAMPLE_INPUT, 6)
        );
    }
    // #[test]
    // fn sample_part_two() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     part_two(SAMPLE_INPUT);
    // }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        part_one(&raw_input, 25);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_one(&raw_input, 75);
    // }
}

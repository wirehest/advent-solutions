use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one() {
    todo!()
}

pub fn part_one() {
    todo!()
}

pub fn parse_input(raw_input: &str) {
    todo!()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open(todo!("add path to input"))?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SAMPLE_INPUT: &str = todo!();

    #[test]
    fn sample_part_one() {
        let parsed_input = parse_input(SAMPLE_INPUT);
        part_one(parsed_input);
    }

    #[test]
    fn sample_part_two() {
        let parsed_input = parse_input(SAMPLE_INPUT);
        part_two(parsed_input);
    }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        let parsed_input = parse_input(&raw_input);
        part_one(parsed_input);
    }

    #[test]
    fn actual_part_two() {
        let raw_input = get_input().unwrap();
        let parsed_input = parse_input(&raw_input);
        part_two(parsed_input);
    }
}

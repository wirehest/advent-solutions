use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(raw_input: &str) {
    todo!()
}

pub fn part_two(raw_input: &str) {
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

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str = todo!();
        part_one(SAMPLE_INPUT);
    }

    // #[test]
    // fn sample_part_two() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     part_two(SAMPLE_INPUT);
    // }

    // #[test]
    // fn actual_part_one() {
    //     let raw_input = get_input().unwrap();
    //     part_one(&raw_input);
    // }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_two(&raw_input);
    // }
}

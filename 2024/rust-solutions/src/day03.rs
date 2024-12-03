use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(parsed_input: Vec<String>) {
    let sum: u32 = parsed_input
        .iter()
        .map(|factors| {
            factors
                .split(',')
                .fold(1, |acc, factor| acc * factor.parse::<u32>().unwrap())
        })
        .sum();

    println!("sum: {:?}", sum);
}

pub fn part_two(parsed_input: Vec<String>) {
    let mut products: Vec<String> = Vec::new();
    let mut do_bool = true;

    parsed_input
        .iter()
        .for_each(|instruction| match instruction.as_ref() {
            "don't()" => do_bool = false,
            "do()" => do_bool = true,
            product => {
                if do_bool {
                    let factors = product.replace("mul(", "").replace(")", "");
                    products.push(factors);
                }
            }
        });

    let sum: u32 = products
        .iter()
        .map(|factors| {
            factors
                .split(',')
                .fold(1, |acc, factor| acc * factor.parse::<u32>().unwrap())
        })
        .sum();

    println!("sum: {:?}", sum);
}

pub fn parser_1(raw_input: &str) -> Vec<String> {
    let re = Regex::new(r"(?:mul\()(\d+,\d+)(?:\))").unwrap();

    // returns vec of "a,b" pairs from each "mul(a,b)"
    re.captures_iter(raw_input)
        .map(|caps| caps.get(1).unwrap().as_str().to_string())
        .collect()
}

pub fn parser_2(raw_input: &str) -> Vec<String> {
    let re = Regex::new(r"mul\(\d+,\d+\)|do\(\)|don't\(\)").unwrap();

    re.find_iter(raw_input)
        .map(|m| m.as_str().to_string())
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day03.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT_1: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let parsed_input = parser_1(SAMPLE_INPUT_1);
        part_one(parsed_input);
    }

    #[test]
    #[ignore]
    fn sample_part_two() {
        const SAMPLE_INPUT_2: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let parsed_input = parser_2(SAMPLE_INPUT_2);
        part_two(parsed_input);
    }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        let parsed_input = parser_1(&raw_input);
        part_one(parsed_input);
    }

    #[test]
    #[ignore]
    fn actual_part_two() {
        let raw_input = get_input().unwrap();
        let parsed_input = parser_2(&raw_input);
        part_two(parsed_input);
    }
}

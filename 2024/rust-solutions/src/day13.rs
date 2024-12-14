use regex::Regex;
use std::fs::File;
use std::io::{self, prelude::*};

pub fn part_one(raw_input: &str) -> i64 {
    let parsed = parse_input(raw_input);

    let tokens_spent: i64 = parsed
        .iter()
        .filter_map(|machine| minimum_pushes(machine))
        .map(|(push_a, push_b)| push_a * 3 + push_b)
        .sum();

    println!("tokens spent: {:?}", tokens_spent);
    tokens_spent
}

pub fn part_two(raw_input: &str) {
    todo!()
}

pub fn minimum_pushes(machine: &[i64]) -> Option<(i64, i64)> {
    // println!("{:?}", machine);
    let [x1, y1, x2, y2, x3, y3] = machine else {
        return None;
    };

    let Some(denom) = (x1 * y2).checked_sub(y1 * x2) else {
        return None;
    };

    let Some(numer_a) = (x3 * y2).checked_sub(y3 * x2) else {
        return None;
    };

    let Some(numer_b) = (y3 * x1).checked_sub(x3 * y1) else {
        return None;
    };

    // println!("a: {:?} b: {:?} denom: {:?}", numer_a, numer_b, denom);

    if numer_a % denom == 0 || numer_b % denom == 0 {
        Some((numer_a / denom, numer_b / denom))
    } else {
        None
    }
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"\d+").unwrap();

    raw_input
        .trim()
        .split("\n\n")
        .map(|machine| {
            re.find_iter(machine)
                .map(|cfg_line| cfg_line.as_str().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day13.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn sample_part_one() {
        assert_eq!(480, part_one(SAMPLE_INPUT));
    }

    #[test]
    fn sample_part_two() {
        part_two(SAMPLE_INPUT);
    }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        part_one(&raw_input);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_two(&raw_input);
    // }
}

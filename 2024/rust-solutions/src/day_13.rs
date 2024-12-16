// Local
use crate::read_input;
use crate::Part;

// External
use regex::Regex;

const PUZZLE_INPUT: &str = "data/day_13.txt";

pub fn tokens_spent(raw_input: &str, part: Part) -> i64 {
    let parsed = parse_input(raw_input);

    parsed
        .iter()
        .filter_map(|machine| minimum_pushes(machine, &part))
        .map(|(push_a, push_b)| push_a * 3 + push_b)
        .sum()
}

pub fn minimum_pushes(machine: &[i64], part: &Part) -> Option<(i64, i64)> {
    let [x1, y1, x2, y2, mut x3, mut y3] = machine.try_into().expect("expect");

    match part {
        Part::One => (),
        Part::Two => {
            x3 += 10_000_000_000_000;
            y3 += 10_000_000_000_000;
        }
    }

    let denom = (x1 * y2).checked_sub(y1 * x2)?;
    let numer_a = (x3 * y2).checked_sub(y3 * x2)?;
    let numer_b = (y3 * x1).checked_sub(x3 * y1)?;

    if numer_a % denom == 0 || numer_b % denom == 0 {
        Some((numer_a / denom, numer_b / denom))
    } else {
        None
    }
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<i64>> {
    let re = Regex::new(r"\d+").unwrap();

    raw_input
        .split("\n\n")
        .map(|machine| {
            re.find_iter(machine)
                .map(|cfg_line| cfg_line.as_str().parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = "Button A: X+94, Y+34
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
    fn part1_sample() {
        let tokens_spent = tokens_spent(SAMPLE_INPUT, Part::One);
        println!("tokens spent (sample): {tokens_spent}");
        assert_eq!(480, tokens_spent);
    }

    #[test]
    fn part1_actual() {
        let raw_input = read_input(PUZZLE_INPUT).expect("input should not be empty");
        let tokens_spent = tokens_spent(&raw_input, Part::One);
        println!("tokens spent (part1): {tokens_spent}");
        assert_eq!(32_067, tokens_spent);
    }

    #[test]
    fn part2_actual() {
        let raw_input = read_input(PUZZLE_INPUT).expect("input should not be empty");
        let tokens_spent = tokens_spent(&raw_input, Part::Two);
        println!("tokens spent (part2): {tokens_spent}");
        assert_eq!(92_871_736_253_789, tokens_spent);
    }
}

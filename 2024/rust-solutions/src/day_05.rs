use std::clone::Clone;
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};
use std::marker::Copy;

#[derive(Debug, Clone, Copy)]
pub struct Rule {
    left: u32,
    right: u32,
}

pub fn part_one(raw_input: &str) {
    let (rules, updates) = parse_input(raw_input);
    let ordered = get_already_ordered(&rules, &updates);

    let x: u32 = ordered
        .iter()
        .map(|update| {
            let length = update.len();
            let middle_index = match length % 2 {
                0 => length / 2 - 1,
                _ => length / 2,
            };
            update.get(middle_index).unwrap().to_owned()
        })
        .sum();

    println!("{:?}", x);
}

pub fn part_two() {
    todo!()
}

pub fn get_applicable_rules(parsed_rules: &[Rule], update: &[u32]) -> Vec<Rule> {
    let mut applicable_rules: Vec<Rule> = Vec::new();
    for rule in parsed_rules {
        if update.contains(&rule.left) && update.contains(&rule.right) {
            applicable_rules.push(*rule);
        }
    }
    applicable_rules
}

pub fn find_first_and_last_pages(applicable_rules: &[Rule]) -> (u32, u32) {
    let (left, right): (Vec<_>, Vec<_>) = applicable_rules
        .iter()
        .map(|rule| (rule.left, rule.right))
        .unzip();

    let set_left: HashSet<_> = HashSet::from_iter(left);
    let set_right: HashSet<_> = HashSet::from_iter(right);

    let first = set_left
        .difference(&set_right)
        .collect::<HashSet<_>>()
        .drain()
        .next()
        .unwrap();
    let last = set_right
        .difference(&set_left)
        .collect::<HashSet<_>>()
        .drain()
        .next()
        .unwrap();

    (*first, *last)
}

pub fn get_already_ordered(parsed_rules: &[Rule], parsed_updates: &[Vec<u32>]) -> Vec<Vec<u32>> {
    let mut in_order: Vec<Vec<u32>> = Vec::new();

    'outer: for update in parsed_updates {
        let applicable_rules = get_applicable_rules(parsed_rules, update);

        // checks first and last page first
        let (first_page, last_page) = find_first_and_last_pages(&applicable_rules);
        if update.first().unwrap() != &first_page || update.last().unwrap() != &last_page {
            continue;
        }

        // checks rules
        for rule in &applicable_rules {
            let index_left = update.iter().position(|page| page == &rule.left);
            let index_right = update.iter().position(|page| page == &rule.right);

            if index_left > index_right {
                continue 'outer;
            }
        }

        in_order.push(update.to_vec());
    }

    in_order
}

pub fn parse_input(raw_input: &str) -> (Vec<Rule>, Vec<Vec<u32>>) {
    let parts: Vec<_> = raw_input.split("\n\n").collect();
    let (rules, updates) = (parts[0].trim(), parts[1].trim());

    // parse rules
    let parsed_rules = rules
        .lines()
        .map(|rule| {
            let rule_pages: Vec<_> = rule.split("|").collect();
            let (left, right) = (
                rule_pages[0].parse().unwrap(),
                rule_pages[1].parse().unwrap(),
            );
            Rule { left, right }
        })
        .collect::<Vec<_>>();

    // parse updates
    let parsed_updates: Vec<_> = updates
        .lines()
        .map(|update| -> Vec<_> {
            update
                .split(",")
                .map(|page| page.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    (parsed_rules, parsed_updates)
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day_05.txt")?;
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
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        part_one(SAMPLE_INPUT);
    }

    // #[test]
    // fn sample_part_two() {
    //     let parsed_input = parse_input(SAMPLE_INPUT);
    //     part_two(parsed_input);
    // }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        part_one(&raw_input);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     let parsed_input = parse_input(&raw_input);
    //     part_two(parsed_input);
    // }
}

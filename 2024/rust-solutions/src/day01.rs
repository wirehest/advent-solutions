use std::fs::File;
use std::io::{self, BufRead};

pub fn part_one(mut first_list: Vec<String>, mut second_list: Vec<String>) -> u32 {
    first_list.sort();
    second_list.sort();

    if first_list.len() != second_list.len() {
        panic!("mismatched lengths");
    }

    let total_distance = first_list
        .iter()
        .enumerate()
        .map(|(i, location_id)| {
            let first_id_as_int = location_id.parse::<u32>().unwrap();
            let second_id_as_int = second_list[i].parse::<u32>().unwrap();
            first_id_as_int.abs_diff(second_id_as_int)
        })
        .sum();

    println!("distance: {total_distance}");
    total_distance
}

pub fn part_two(first_list: Vec<String>, second_list: Vec<String>) -> u32 {
    let similarity_score: u32 = first_list
        .iter()
        .map(|id_in_first| {
            let count_in_second_list = second_list
                .iter()
                .filter(|id_in_second| *id_in_second == id_in_first)
                .count();
            id_in_first.parse::<u32>().unwrap() * count_in_second_list as u32
        })
        .sum();

    println!("similarity: {similarity_score}");
    similarity_score
}

pub fn reader() -> io::Result<(Vec<String>, Vec<String>)> {
    let file = File::open("data/day01.txt")?;
    let reader = io::BufReader::new(file);
    let mut first_list = Vec::new();
    let mut second_list = Vec::new();

    reader.lines().map_while(Result::ok).for_each(|line| {
        let distances: Vec<_> = line.split("  ").collect();
        first_list.push(distances[0].trim().to_string());
        second_list.push(distances[1].trim().to_string());
    });

    Ok((first_list, second_list))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_part_one() {
        let (first_list, second_list) = reader().unwrap();
        part_one(first_list, second_list);
    }

    #[test]
    fn solve_part_two() {
        let (first_list, second_list) = reader().unwrap();
        part_two(first_list, second_list);
    }
}

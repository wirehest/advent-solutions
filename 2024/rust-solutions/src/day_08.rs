// Standard
use std::collections::{HashMap, HashSet};

// Local
use crate::read_input;
use crate::Part;

// External
use itertools::Itertools;

const PUZZLE_INPUT: &str = "data/day_08.txt";

type Antennas = HashMap<Frequency, Vec<Node>>;
type Map = Vec<Vec<char>>;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Frequency(char);

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Node {
    y: i32,
    x: i32,
}

pub fn count_antinodes(raw_input: &str) -> u32 {
    let map = parse_input(raw_input);
    let height: i32 = map.len().try_into().expect("non-empty map");
    let width: i32 = map
        .first()
        .expect("non-empty map row")
        .len()
        .try_into()
        .unwrap();
    let antennas = find_antennas(&map);
    let mut antinodes: HashSet<Node> = HashSet::new();

    for frequency in antennas.values() {
        for antenna1 in frequency.iter() {
            for antenna2 in frequency.iter() {
                if antenna1 == antenna2 {
                    continue;
                }

                let (dx, dy) = (antenna2.x - antenna1.x, antenna2.y - antenna1.y);

                if antenna1.x - dx >= 0
                    && antenna1.x - dx < width
                    && antenna1.y - dy >= 0
                    && antenna1.y - dy < height
                {
                    antinodes.insert(Node {
                        x: antenna1.x - dx,
                        y: antenna1.y - dy,
                    });
                }

                if antenna2.x + dx >= 0
                    && antenna2.x + dx < width
                    && antenna2.y + dy >= 0
                    && antenna2.y + dy < height
                {
                    antinodes.insert(Node {
                        x: antenna2.x + dx,
                        y: antenna2.y + dy,
                    });
                }
            }
        }
    }

    antinodes.len() as u32
}

// pub fn part_two(raw_input: &str) {
//     todo!()
// }

pub fn find_antennas(map: &Map) -> Antennas {
    let mut antennas: Antennas = HashMap::new();

    for (y, row) in map.iter().enumerate() {
        for (x, frequency) in row.iter().enumerate() {
            if !frequency.is_alphanumeric() {
                continue;
            }

            antennas
                .entry(Frequency(*frequency))
                .and_modify(|v| {
                    v.push(Node {
                        y: y as i32,
                        x: x as i32,
                    })
                })
                .or_insert(vec![Node {
                    y: y as i32,
                    x: x as i32,
                }]);
        }
    }

    antennas
}

pub fn parse_input(raw_input: &str) -> Map {
    raw_input
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect()
}

// pub fn get_input() -> io::Result<String> {
//     let mut file = File::open("data/day08.txt")?;
//     let mut raw_input = String::new();
//     file.read_to_string(&mut raw_input)?;
//
//     Ok(raw_input)
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_sample() {
        const SAMPLE_INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let unique_antinodes = count_antinodes(SAMPLE_INPUT);
        println!("unique antinodes (sample): {unique_antinodes}");
        assert_eq!(14, unique_antinodes);
    }

    // #[test]
    // fn sample_part_two() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     part_two(SAMPLE_INPUT);
    // }

    #[test]
    fn part1_actual() {
        let raw_input = read_input(PUZZLE_INPUT).expect("input should not be empty");
        let unique_antinodes = count_antinodes(&raw_input);
        println!("unique antinodes (part1): {unique_antinodes}");
        assert_eq!(295, unique_antinodes);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_two(raw_input);
    // }
}

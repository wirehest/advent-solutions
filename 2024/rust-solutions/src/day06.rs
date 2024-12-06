use std::clone::Clone;
use std::fmt;
use std::fs::File;
use std::io::{self, prelude::*};
use std::marker::Copy;

struct Patrol {
    map: Vec<Vec<char>>,
    current: Position,
    visited: Vec<String>,
}

impl Patrol {
    fn turn(&mut self) {
        self.current.facing = match &self.current.facing {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("invalid facing"),
        }
    }

    fn advance(&mut self) {
        let width: usize = self.map[0].len();
        let height: usize = self.map.len();

        while !self.visited.contains(&self.current.to_string()) {
            let next_x: Option<usize>;
            let next_y: Option<usize>;

            match &self.current.facing {
                '^' => {
                    next_x = Some(self.current.x);
                    next_y = self.current.y.checked_sub(1);
                }
                '>' => {
                    next_x = self.current.x.checked_add(1);
                    next_y = Some(self.current.y);
                }
                'v' => {
                    next_x = Some(self.current.x);
                    next_y = self.current.y.checked_add(1);
                }
                '<' => {
                    next_x = self.current.x.checked_sub(1);
                    next_y = Some(self.current.y);
                }
                _ => panic!("invalid facing"),
            };

            if Option::is_none(&next_x)
                || Option::is_none(&next_y)
                || next_x.unwrap() >= width
                || next_y.unwrap() >= height
            {
                self.visited.push(self.current.to_string());
                break;
            }

            if self.map[next_y.unwrap()][next_x.unwrap()] == '#' {
                self.turn();
            } else {
                self.visited.push(self.current.to_string());
                self.current.x = next_x.unwrap();
                self.current.y = next_y.unwrap();
            }
        }
    }

    fn count_unique_positions(&self) -> usize {
        let mut x: Vec<_> = self
            .visited
            .iter()
            .map(|position| position.rsplit_once('-').unwrap().0)
            .collect();

        x.sort();
        x.dedup();
        x.len()
    }
}

#[derive(Clone, Copy)]
pub struct Position {
    x: usize,
    y: usize,
    facing: char,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}-{}", self.x, self.y, self.facing)
    }
}

pub fn part_one(raw_input: &str) {
    let map = parse_input(raw_input);
    let start = find_start(&map).unwrap();
    let mut patrol = Patrol {
        map,
        current: start,
        visited: Vec::new(),
    };

    patrol.advance();
    // println!("VISITED: {:?}", patrol.visited);
    println!("TOTAL: {:?}", patrol.count_unique_positions());
}

pub fn _part_two(_raw_input: &str) {
    todo!()
}

pub fn find_start(map: &[Vec<char>]) -> Option<Position> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|c| matches!(c, '^' | '>' | 'v' | '<')) {
            return Some(Position {
                x,
                y,
                facing: map[y][x],
            });
        }
    }
    None
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day06.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        part_one(SAMPLE_INPUT);
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

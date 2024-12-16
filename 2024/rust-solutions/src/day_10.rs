use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Map {
    grid: Vec<Vec<u32>>,
    height: usize,
    width: usize,
    trailheads: Vec<MapSquare>,
}

impl Map {
    fn total_score(&self, part: PuzzlePart) -> u32 {
        self.trailheads
            .iter()
            .map(|trailhead| {
                let mut complete_trails = self.hike(trailhead);

                if let PuzzlePart::One = part {
                    complete_trails.sort();
                    complete_trails.dedup();
                }

                complete_trails.len() as u32
            })
            .sum()
    }

    fn hike(&self, trailhead: &MapSquare) -> Vec<(usize, usize)> {
        let mut adjacents = vec![(trailhead.x, trailhead.y, trailhead.value)];
        let mut peaks: Vec<(usize, usize)> = Vec::new();

        while let Some((x, y, current_value)) = adjacents.pop() {
            // check up square
            if y + 1 < self.height {
                let (up_x, up_y) = (x, y + 1);
                let next_value = self.grid[up_y][up_x];
                if next_value == current_value + 1 {
                    if next_value == 9 {
                        peaks.push((up_x, up_y));
                    } else {
                        adjacents.push((up_x, up_y, current_value + 1));
                    }
                }
            };

            // check down square
            if y.checked_sub(1).is_some() {
                let (down_x, down_y) = (x, y - 1);
                let next_value = self.grid[down_y][down_x];
                if next_value == current_value + 1 {
                    if next_value == 9 {
                        peaks.push((down_x, down_y));
                    } else {
                        adjacents.push((down_x, down_y, current_value + 1))
                    }
                }
            };

            // check right square
            if x + 1 < self.width {
                let (right_x, right_y) = (x + 1, y);
                let next_value = self.grid[right_y][right_x];
                if next_value == current_value + 1 {
                    if next_value == 9 {
                        peaks.push((right_x, right_y));
                    } else {
                        adjacents.push((right_x, right_y, current_value + 1));
                    }
                }
            };

            // check left square
            if x.checked_sub(1).is_some() {
                let (left_x, left_y) = (x - 1, y);
                let next_value = self.grid[left_y][left_x];
                if next_value == current_value + 1 {
                    if next_value == 9 {
                        peaks.push((left_x, left_y));
                    } else {
                        adjacents.push((left_x, left_y, current_value + 1));
                    }
                }
            };
        }

        peaks
    }
}

#[derive(Debug)]
pub struct MapSquare {
    x: usize,
    y: usize,
    value: u32,
}

pub enum PuzzlePart {
    One,
    Two,
}

pub fn part_one(raw_input: &str) -> u32 {
    let map = parse_input(raw_input);
    let total_score = map.total_score(PuzzlePart::One);
    println!("total score: {:?}", total_score);

    total_score
}

pub fn part_two(raw_input: &str) -> u32 {
    let map = parse_input(raw_input);
    let total_score = map.total_score(PuzzlePart::Two);
    println!("total score: {:?}", total_score);

    total_score
}

pub fn parse_input(raw_input: &str) -> Map {
    let mut trailheads: Vec<MapSquare> = Vec::new();
    let grid = raw_input
        .trim()
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, char)| {
                    let height = char.to_digit(10).expect("expect parsable digit");
                    if height == 0 {
                        trailheads.push(MapSquare { x, y, value: 0 });
                    }
                    height
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid.first().expect("expect non-empty map").len();

    Map {
        grid,
        height,
        width,
        trailheads,
    }
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day_10.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn p1_short_sample() {
        const SAMPLE: &str = "
0123
1234
8765
9876";
        part_one(SAMPLE);
    }

    #[test]
    fn p1_long_sample() {
        const SAMPLE: &str = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let score = part_one(SAMPLE);
        assert_eq!(36, score, "should have score of 36");
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

    #[test]
    fn actual_part_two() {
        let raw_input = get_input().unwrap();
        part_two(&raw_input);
    }
}

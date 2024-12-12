use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Region {
    plant: char,
    plots: HashSet<Plot>,
    perimeter: u32,
}

#[derive(Default, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Plot {
    y: usize,
    x: usize,
}

impl Plot {
    fn from(y: usize, x: usize) -> Self {
        Self { y, x }
    }
}

pub fn part_one(raw_input: &str) -> u32 {
    let parsed = parse_input(raw_input);
    let mut regions = identify_regions(&parsed);
    update_perimeter(&parsed, &mut regions);

    let total_price = regions
        .iter()
        .map(|region| region.plots.len() as u32 * region.perimeter)
        .sum();

    println!("total price: {}", total_price);
    total_price
}

pub fn part_two(raw_input: &str) {
    todo!()
}

pub fn update_perimeter(parsed: &[Vec<char>], regions: &mut [Region]) {
    regions.iter_mut().for_each(|region| {
        let adjacents = region
            .plots
            .iter()
            .map(|plot| identify_adjacents(parsed, plot).len() as u32)
            .sum::<u32>();
        let area = region.plots.len() as u32;
        region.perimeter = area * 4 - adjacents;
    });
}

pub fn identify_regions(parsed: &[Vec<char>]) -> Vec<Region> {
    let mut regions: Vec<Region> = Vec::new();
    let mut checked: HashSet<Plot> = HashSet::new();

    for (y, row) in parsed.iter().enumerate() {
        for (x, plant) in row.iter().enumerate() {
            if checked.contains(&Plot::from(y, x)) {
                continue;
            }
            let mut region = Region {
                plots: HashSet::new(),
                plant: *plant,
                perimeter: 0,
            };

            region.plots.insert(Plot::from(y, x));

            let mut adjacents = identify_adjacents(parsed, &Plot::from(y, x));

            while let Some(adjacent) = adjacents.pop() {
                if checked.contains(&adjacent) {
                    continue;
                }
                region.plots.insert(adjacent);
                adjacents.append(&mut identify_adjacents(parsed, &adjacent));
                checked.insert(adjacent);
            }

            regions.push(region);
        }
    }

    regions
}

pub fn identify_adjacents(parsed: &[Vec<char>], plot: &Plot) -> Vec<Plot> {
    let max_height = parsed.len() as i32;
    let max_width = parsed
        .first()
        .map(|first_row| first_row.len() as i32)
        .expect("expect non-empty row");
    let (y, x) = (plot.y, plot.x);
    let plant = parsed[y][x];

    [(0, 1), (0, -1), (1, 0), (-1, 0)]
        .iter()
        .filter_map(|(dy, dx)| {
            let (next_y, next_x) = (y as i32 + dy, x as i32 + dx);

            if (0..max_height).contains(&next_y)
                && (0..max_width).contains(&next_x)
                && parsed[next_y as usize][next_x as usize] == plant
            {
                Some(Plot {
                    y: next_y as usize,
                    x: next_x as usize,
                })
            } else {
                None
            }
        })
        .collect()
}

pub fn parse_input(raw_input: &str) -> Vec<Vec<char>> {
    raw_input
        .trim()
        .lines()
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day12.txt")?;
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
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!(1930, part_one(SAMPLE_INPUT));
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
    //     part_two(&raw_input);
    // }
}

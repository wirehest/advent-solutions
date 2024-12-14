use regex::Regex;
use std::cmp::Ordering;
use std::fs::File;
use std::io::{self, prelude::*};

#[derive(Debug)]
pub struct Space {
    bots: Vec<Bot>,
    width: i16,
    height: i16,
}

impl Space {
    fn from(bots: Vec<Bot>, width: i16, height: i16) -> Self {
        Self {
            bots,
            width,
            height,
        }
    }

    fn move_all(&mut self, wait_time: i16) {
        let width = self.width;
        let height = self.height;

        self.bots.iter_mut().for_each(|bot| {
            (0..wait_time).for_each(|_| bot.move_bot(width, height));
        });
    }

    fn count_in_quads(&self) -> usize {
        let (mid_x, mid_y) = (self.width / 2, self.height / 2);
        let (north, south) = ((0, mid_y), (mid_y + 1, self.height));
        let (west, east) = ((0, mid_x), (mid_x + 1, self.width));

        let north_east = (north, east);
        let north_west = (north, west);
        let south_east = (south, east);
        let south_west = (south, west);

        [north_east, north_west, south_east, south_west]
            .iter()
            .map(|quad| {
                self.bots
                    .iter()
                    .filter(|bot| {
                        bot.y >= quad.0 .0
                            && bot.y < quad.0 .1
                            && bot.x >= quad.1 .0
                            && bot.x < quad.1 .1
                    })
                    .count()
            })
            .product()
    }
}

#[derive(Debug)]
pub struct Bot {
    x: i16,
    y: i16,
    dx: i16,
    dy: i16,
}

impl Bot {
    fn move_bot(&mut self, width: i16, height: i16) {
        self.x += self.dx;
        if self.x < 0 {
            self.x += width;
        } else if self.x >= width {
            self.x -= width;
        }

        self.y += self.dy;
        if self.y < 0 {
            self.y += height;
        } else if self.y >= height {
            self.y -= height;
        }
    }
}

pub fn part_one(raw_input: &str, width: i16, height: i16, wait_time: i16) -> usize {
    let parsed = parse_input(raw_input);
    let mut lobby = Space::from(parsed, width, height);

    lobby.move_all(wait_time);
    let safety_factor = lobby.count_in_quads();

    println!("safety factor: {safety_factor:?}");
    safety_factor
}

pub fn part_two(raw_input: &str) {
    todo!()
}

pub fn parse_input(raw_input: &str) -> Vec<Bot> {
    let re = Regex::new(r"-?\d+").unwrap();

    raw_input
        .lines()
        .map(|line| {
            let robot: Vec<i16> = re
                .find_iter(line)
                .map(|v| v.as_str().parse::<i16>().unwrap())
                .collect();

            Bot {
                x: robot[0],
                y: robot[1],
                dx: robot[2],
                dy: robot[3],
            }
        })
        .collect()
}

pub fn get_input() -> io::Result<String> {
    let mut file = File::open("data/day14.txt")?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;

    Ok(raw_input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_part_one() {
        const SAMPLE_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        const WIDTH: i16 = 11;
        const HEIGHT: i16 = 7;
        const WAIT_TIME: i16 = 100;
        assert_eq!(12, part_one(SAMPLE_INPUT, WIDTH, HEIGHT, WAIT_TIME));
    }

    // #[test]
    // fn sample_part_two() {
    //     const SAMPLE_INPUT: &str = todo!();
    //     part_two(SAMPLE_INPUT);
    // }

    #[test]
    fn actual_part_one() {
        let raw_input = get_input().unwrap();
        const WIDTH: i16 = 101;
        const HEIGHT: i16 = 103;
        const WAIT_TIME: i16 = 100;
        part_one(&raw_input, WIDTH, HEIGHT, WAIT_TIME);
    }

    // #[test]
    // fn actual_part_two() {
    //     let raw_input = get_input().unwrap();
    //     part_two(&raw_input);
    // }
}

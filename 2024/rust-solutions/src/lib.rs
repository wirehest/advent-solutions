use std::fs::File;
use std::io::{self, prelude::*};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

pub enum Part {
    One,
    Two,
}

pub fn read_input(input: &str) -> io::Result<String> {
    let mut file = File::open(input)?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;
    Ok(raw_input)
}

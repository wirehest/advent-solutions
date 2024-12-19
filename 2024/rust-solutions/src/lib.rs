use std::fs::File;
use std::io::{self, prelude::*};

pub mod day_01;
pub mod day_02;
pub mod day_03;
pub mod day_04;
pub mod day_05;
pub mod day_06;
pub mod day_07;
pub mod day_08;
pub mod day_09;
pub mod day_10;
pub mod day_11;
pub mod day_12;
pub mod day_13;
pub mod day_14;
pub mod day_15;
pub mod day_16;
pub mod day_19;

pub enum Part {
    One,
    Two,
}

/// Read puzzle input at runtime
pub fn read_input(input: &str) -> io::Result<String> {
    let mut file = File::open(input)?;
    let mut raw_input = String::new();
    file.read_to_string(&mut raw_input)?;
    Ok(raw_input)
}

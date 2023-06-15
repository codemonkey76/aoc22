use std::num::ParseIntError;

use crate::day01::Input;


const INPUT: &str = include_str!("../../input/01/input.txt");

pub fn read() -> Input {
    INPUT
        .trim()
        .split("\n\n")
        .map(calorie_count)
        .collect::<Result<Input,_>>()
        .expect("Failed to parse input")
}

fn calorie_count(value: &str) -> Result<u32,ParseIntError> {
    let mut total = 0;

    for line in value.lines() {
        total += line.parse::<u32>()?
    }

    Ok(total)
}

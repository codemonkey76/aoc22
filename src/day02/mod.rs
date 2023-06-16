use crate::Output;

pub mod input;
pub mod part1;
pub mod part2;

pub type Input = Vec<u32>;

pub fn run_both() -> (Output, Output) {
    let input = input::read();
    (part1::solve(&input), part2::solve(&input))
}
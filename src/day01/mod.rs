pub mod input;
pub mod part1;
pub mod part2;

use crate::{Part, Output};

pub fn run(part: Part) -> Output {
    match part {
        Part::One => part1::solve(),
        Part::Two => part2::solve(),
    }
}

pub fn run_both() -> (Output, Output) {
    (part1::solve(), part2::solve())
}

use crate::Output;
use crate::day01::Input;

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .copied()
        .max()
        .expect("Some input required")
        .into()
}


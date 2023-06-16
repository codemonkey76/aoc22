use crate::day01::Input;
use crate::Output;

pub fn solve(input: &Input) -> Output {
    input
        .iter()
        .copied()
        .max()
        .expect("Some input required")
        .into()
}

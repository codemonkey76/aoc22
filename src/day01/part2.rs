use crate::Output;
use crate::day01::Input;

pub fn solve(input: &Input) -> Output {
   largest(input, 3).iter().sum::<u32>().into()
}

pub fn largest(input: &Input, n: usize) -> Vec<u32> {
    let mut sorted = input.clone();
    sorted.sort_unstable();
    sorted.iter().rev().take(n).cloned().collect::<Vec<u32>>()
}

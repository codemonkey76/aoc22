use aoc_lib::*;
use clap::Parser;
use std::fmt::{Display, Formatter, Result as FmtResult};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Day to run
    #[arg(short, long)]
    day: Option<u8>,

    /// Run all days
    #[arg(short, long)]
    all: bool,
}

fn main() {
    let args = Args::parse();

    let day = args.day;
    let all = args.all;

    if all {
        let mut results = Vec::with_capacity(25);
        println!("Running all days");
        (1..=25).for_each(|day| results.push(run_day(day)));
        results.iter().for_each(|result| println!("{}", result));
        return;
    }

    if let Some(day) = day {
        println!("Day: {:?}", day);
        println!("{}", run_day(day));
        return;
    }

    println!("No day specified");
}

fn run_day(day: u8) -> RunResult {
    println!("Running day: {}", day);

    let run = match day {
        1 => day01::run,
        _ => panic!("Day not implemented"),
    };

    let answer_one = run(Part::One);
    let answer_two = run(Part::Two);
    RunResult {
        day,
        answer_one,
        answer_two,
    }
}

struct RunResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(
            f,
            "Day: {}\n\tPart One: {}\n\tPart Two: {}",
            self.day, self.answer_one, self.answer_two
        )
    }
}

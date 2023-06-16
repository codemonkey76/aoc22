use aoc_lib::*;
use clap::Parser;
use std::time::{Instant, Duration};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

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
    let timer = Instant::now();

    if all {
        let mut results = Vec::with_capacity(25);
        println!("Running all days");
        (1..=25).for_each(|day| results.push(run_day(day, &timer)));
        results.iter().for_each(|result| println!("{}", result));
        return;
    }

    if let Some(day) = day {
        println!("Day: {:?}", day);
        println!("{}", run_day(day, &timer));
        return;
    }

    println!("No day specified");
}

fn run_day(day: u8, timer: &Instant) -> RunResult {
    println!("Running day: {}", day);

    let run_both = match day {
        1 => day01::run_both,
        2 => day02::run_both,
        _ => panic!("Day not implemented"),
    };

    let start = timer.elapsed();
    let (answer_one, answer_two) = run_both();
    let duration = timer.elapsed() - start;

    RunResult {
        day,
        answer_one,
        answer_two,
        duration
    }
}

struct RunResult {
    day: u8,
    answer_one: Output,
    answer_two: Output,
    duration: Duration
}

impl Display for RunResult {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let microseconds = self.duration.as_secs_f32()*1_000_000.0;
        writeln!(f, "***************************************************")?;
        writeln!(f, "*    Advent of Code: 2022, Day {:0>2}                 *", self.day)?;
        writeln!(f, "*        Solution for...                          *")?;
        writeln!(f, "*            Part One: {:>12}               *", self.answer_one.to_string())?;
        writeln!(f, "*            Part Two: {:>12}               *", self.answer_two.to_string())?;
        writeln!(f, "*    Run Time: {:>10.4}µs                       *", microseconds)?;
        writeln!(f, "***************************************************")
    }
}

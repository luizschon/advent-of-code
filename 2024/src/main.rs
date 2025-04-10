use clap::Parser;
use std::fs;

use crate::days::DAY_SOLUTIONS;

pub mod days;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if let Some(solution) = DAY_SOLUTIONS.get(&day) {
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input, day);
        } else {
            eprintln!("Day {} not registered", day);
        }
    } else {
        let mut first_run = true;

        for (&day, solution) in DAY_SOLUTIONS.iter() {
            if !first_run {
                print!("\n");
            }
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input, day);
            first_run = false;
        }
    }
}

use clap::Parser;
use std::{fs, path::Path};

use crate::days::DAY_SOLUTIONS;
use colored::Colorize;

pub mod days;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u32>,
}

fn read_day_input(path: &str, day: u32) -> Option<String> {
    if let Ok(input) = fs::read_to_string(path) {
        Some(input)
    } else {
        eprintln!(
            "{}",
            format!("Missing input file for day {day}, consider adding it to the input dir.").red()
        );
        None
    }
}

fn main() {
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if let Some(solution) = DAY_SOLUTIONS.get(&day) {
            let Some(input) = read_day_input(&format!("input/day_{:02}", day), day) else {
                return;
            };
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
            let Some(input) = read_day_input(&format!("input/day_{:02}", day), day) else {
                continue;
            };
            solution.run_parts(&input, day);
            first_run = false;
        }
    }
}

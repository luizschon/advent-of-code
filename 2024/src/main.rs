use clap::Parser;
use std::fs;

pub mod days;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let registry = register_days!(day_01, day_02);
    let cli = Cli::parse();

    if let Some(day) = cli.day {
        if let Some(solution) = registry.get(day) {
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input, day);
        } else {
            eprintln!("Day {} not registered", day);
        }
    } else {
        let mut first_run = true;
        for (&day, solution) in registry.solutions.iter() {
            if !first_run {
                print!("\n");
            }
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input, day);
            first_run = false;
        }
    }
}

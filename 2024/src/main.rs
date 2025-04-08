use clap::{error::ErrorKind, CommandFactory, Parser};
use std::fs;

pub mod days;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short = 'a', long = "all")]
    run_all: bool,

    #[arg(short, long)]
    day: Option<u32>,
}

fn main() {
    let registry = register_days!(day_01, day_02);
    let cli = Cli::parse();

    if cli.run_all {
        for (day, solution) in registry.solutions.iter() {
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input);
        }
    } else {
        let Some(day) = cli.day else {
            Cli::command()
                .error(
                    ErrorKind::ArgumentConflict,
                    "Setting --day is required when not using --all",
                )
                .exit()
        };

        if let Some(solution) = registry.get(day) {
            let input = fs::read_to_string(format!("input/day_{:02}", day)).unwrap();
            solution.run_parts(&input);
        } else {
            Cli::command().error(ErrorKind::InvalidValue, "Day {} not registered");
        }
    }
}

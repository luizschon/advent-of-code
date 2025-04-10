use std::collections::BTreeMap;
use std::fmt::Display;
use std::time::Instant;

use colored::Colorize;
use lazy_static::lazy_static;
use paste::paste;

type SolutionFn = fn(&str) -> Box<dyn Display>;

#[derive(Clone, Copy)]
pub struct DaySolution {
    part_1: SolutionFn,
    part_2: SolutionFn,
}

impl DaySolution {
    pub fn new(part_1: SolutionFn, part_2: SolutionFn) -> Self {
        Self { part_1, part_2 }
    }

    pub fn run_parts(&self, input: &str, day: u32) {
        println!(
            "{} {}",
            "─┬─".cyan(),
            format!("{} {:02} {}", "Running day".blue(), day, "solution:".blue()).italic()
        );

        macro_rules! print_result {
            ($res:expr, $c:expr) => {
                let time = Instant::now();
                println!(
                    "{} {} {}. {} {:?}",
                    &format!(" {}", $c).cyan(),
                    "Answer:".green(),
                    $res,
                    "Elapsed time:".yellow(),
                    time.elapsed()
                );
            };
        }

        print_result!((self.part_1)(input), "├─");
        print_result!((self.part_2)(input), "└─");
    }
}

macro_rules! register_days {
    ($($day:literal),*) => {
        paste! {
            lazy_static! {
                pub static ref DAY_SOLUTIONS: BTreeMap<u32, DaySolution> = {
                    let mut map = BTreeMap::new();
                    $(
                        map.insert($day, DaySolution::new([<day_ $day>]::part_1, [<day_ $day>]::part_2));
                    )*
                    map
                };
            }
        }
    }
}

// The generated code exports every day_XX module and creates the DAY_SOLUTIONS
// static map.
include!("_generated_.rs");

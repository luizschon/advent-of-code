use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;
use std::time::Instant;

use colored::Colorize;

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

        let time = Instant::now();
        println!(
            "{} {} {}. {} {:?}",
            " ├─".cyan(),
            "Answer:".green(),
            (self.part_1)(input),
            "Elapsed time:".yellow(),
            time.elapsed()
        );

        let time = Instant::now();
        println!(
            "{} {} {}. {} {:?}",
            " └─".cyan(),
            "Answer:".green(),
            (self.part_2)(input),
            "Elapsed time:".yellow(),
            time.elapsed()
        );
    }
}

pub struct DaysRegistry {
    pub solutions: HashMap<u32, Box<DaySolution>>,
}

impl DaysRegistry {
    pub fn new() -> Self {
        Self {
            solutions: HashMap::new(),
        }
    }

    pub fn register_solution(&mut self, day: u32, solution: DaySolution) {
        self.solutions.insert(day, Box::new(solution));
    }

    pub fn get(&self, day: u32) -> Option<DaySolution> {
        self.solutions.get(&day).map(|d| d.deref()).copied()
    }
}

macro_rules! export_days {
    ($($day:ident),*) => {
        $(
            pub mod $day;
        )*
    };
}

export_days!(day_01, day_02);

#[macro_export]
macro_rules! register_days {
    ($($day:ident),*) => (
        {
            use crate::days::{DaySolution, DaysRegistry};
            let mut registry = DaysRegistry::new();
            $(
                use crate::days::$day;
                let day_num = stringify!($day)
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .fold(0, |acc, digit| acc * 10 + digit);
                let day_solution = DaySolution::new($day::part_1, $day::part_2);
                registry.register_solution(day_num, day_solution);
            )*
            registry
        }
    )
}

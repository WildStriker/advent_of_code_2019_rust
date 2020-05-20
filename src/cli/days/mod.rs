use clap::Clap;
use std::error;
use std::io;

mod day01;
mod shared;

/// List of Days
#[derive(Clap)]
pub enum Days {
    Day01(day01::Day01),
}

pub fn run<T>(reader: T, day: Days) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    // Run day
    match day {
        Days::Day01(d) => match d.parts {
            shared::Parts::Part01(_) => crate::days::day01::part01(reader),
            shared::Parts::Part02(_) => crate::days::day01::part02(reader),
        },
    }
}

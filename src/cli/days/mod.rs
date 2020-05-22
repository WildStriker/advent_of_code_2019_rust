use clap::Clap;
use std::error;
use std::io;

mod day01;
mod day02;
mod day03;
mod shared;

/// List of Days
#[derive(Clap)]
pub enum Days {
    Day01(day01::Day01),
    Day02(day02::Day02),
    Day03(day03::Day03),
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
        Days::Day02(d) => match d.parts {
            day02::Parts::Part01(p) => crate::days::day02::part01(reader, p.noun, p.verb),
            day02::Parts::Part02(p) => crate::days::day02::part02(reader, p.target),
        },
        Days::Day03(d) => match d.parts {
            shared::Parts::Part01(_) => crate::days::day03::part01(reader),
            shared::Parts::Part02(_) => crate::days::day03::part02(reader),
        },
    }
}
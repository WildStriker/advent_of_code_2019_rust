use clap::Clap;
use std::error;
use std::io;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod shared;

/// List of Days
#[derive(Clap)]
pub enum Days {
    Day01(day01::Day01),
    Day02(day02::Day02),
    Day03(day03::Day03),
    Day04(day04::Day04),
    Day05(day05::Day05),
    Day06(day06::Day06),
    Day07(day07::Day07),
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
        Days::Day04(d) => match d.parts {
            shared::Parts::Part01(_) => crate::days::day04::part01(reader),
            shared::Parts::Part02(_) => crate::days::day04::part02(reader),
        },
        Days::Day05(d) => match d.parts {
            day05::Parts::Part01(p) => crate::days::day05::part01(reader, p.input),
            day05::Parts::Part02(p) => crate::days::day05::part02(reader, p.input),
        },
        Days::Day06(d) => match d.parts {
            day06::Parts::Part01(_) => crate::days::day06::part01(reader),
            day06::Parts::Part02(p) => crate::days::day06::part02(reader, p.start, p.find),
        },
        Days::Day07(d) => match d.parts {
            shared::Parts::Part01(_) => crate::days::day07::part01(reader),
            shared::Parts::Part02(_) => crate::days::day07::part02(reader),
        },
    }
}

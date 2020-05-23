use std::error;
use std::io;

mod secure;
use std::convert::TryFrom;

fn get_counter<T>(mut reader: T) -> Result<secure::PatternCounter, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    let mut split = line.split("-");

    let start = match split.next() {
        Some(value) => value.parse::<u32>()?,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "No Value Found in Start Range (before hypen)",
            )))
        }
    };

    let end = match split.next() {
        Some(value) => value.parse::<u32>()?,
        _ => {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                "No Value Found in End Range (after hypen)",
            )))
        }
    };

    let counter = secure::PatternCounter::try_from((start, end))?;

    Ok(counter)
}

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let counter = get_counter(reader)?;

    let count = counter.pattern_1();

    Ok(format!("Day 04 Part 01 Answer: {}", count))
}

pub fn part02<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let counter = get_counter(reader)?;

    let count = counter.pattern_2();

    Ok(format!("Day 04 Part 02 Answer: {}", count))
}

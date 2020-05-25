use std::error;
use std::io;

mod orbit;

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let orbits = orbit::parse_orbits(reader)?;

    let count = orbits.total_orbits();

    Ok(format!("Day 06 Part 01 Answer: {}", count))
}

pub fn part02<T>(reader: T, start: String, end: String) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let orbits = orbit::parse_orbits(reader)?;
    // this count is inclusive so we want to remove 2
    let count = orbits.calculate_distance(&start, &end)?;

    Ok(format!("Day 06 Part 01 Answer: {}", count - 2))
}

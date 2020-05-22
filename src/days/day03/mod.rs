use std::error;
use std::io;

mod intersect;
use std::convert::TryFrom;

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut wires = Vec::new();

    for res in reader.lines() {
        let line = res?;

        wires.push(intersect::Wire::try_from(line)?);
    }

    match wires[0].closet_intersection(&wires[1]) {
        Some(closet) => Ok(format!("Day 03 Part 01 Answer: {}", closet)),
        None => Ok("Day 03 Part 02: No Intersections Found!".to_string()),
    }
}

pub fn part02<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut wires = Vec::new();

    for res in reader.lines() {
        let line = res?;

        wires.push(intersect::Wire::try_from(line)?);
    }

    match wires[0].fewest_steps(&wires[1]) {
        Some(fewest) => Ok(format!("Day 03 Part 02 Answer: {}", fewest)),
        None => Ok("Day 03 Part 02: No Intersections Found!".to_string()),
    }
}

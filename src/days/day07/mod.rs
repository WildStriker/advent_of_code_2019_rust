use std::error;
use std::io;

use crate::shared::intcode;
mod amplify;

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut amps = amplify::Amps::new(&mem, vec![0, 1, 2, 3, 4])?;

    let strongest = amps.run()?;

    Ok(format!("Day 07 Part 01 Answer: {}", strongest))
}

pub fn part02<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut amps = amplify::Amps::new(&mem, vec![5, 6, 7, 8, 9])?;

    let strongest = amps.run()?;

    Ok(format!("Day 07 Part 02 Answer: {}", strongest))
}

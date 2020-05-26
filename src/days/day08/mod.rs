use std::error;
use std::io;

mod imagery;

pub fn part01<T>(reader: T, wide: usize, tall: usize) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let pixels = imagery::parse(reader)?;

    let image = imagery::Image::new(&pixels, wide, tall)?;
    let check_sum = image.validate();
    Ok(format!("Day 08 Part 01 Answer: {}", check_sum))
}

pub fn part02<T>(reader: T, wide: usize, tall: usize) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let pixels = imagery::parse(reader)?;

    let image = imagery::Image::new(&pixels, wide, tall)?;
    Ok(format!("Day 08 Part 02 Answer: {}", image))
}

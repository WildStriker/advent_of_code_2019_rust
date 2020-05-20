use std::error;
use std::io;

fn calculate_fuel(mass: u32) -> u32 {
    (mass / 3).saturating_sub(2)
}

fn calculate_fuel_for_fuel(mass: u32) -> u32 {
    let mut total = 0;

    let mut fuel = calculate_fuel(mass);
    while fuel > 0 {
        total += fuel;
        fuel = calculate_fuel(fuel);
    }

    total
}

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut total = 0;

    for res in reader.lines() {
        let line = res?;

        total += calculate_fuel(line.parse::<u32>()?);
    }

    Ok(format!("Day 01 Part 01 Answer: {}", total))
}

pub fn part02<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut total = 0;

    for res in reader.lines() {
        let line = res?;
        total += calculate_fuel_for_fuel(line.parse::<u32>()?)
    }

    Ok(format!("Day 01 Part 02 Answer: {}", total))
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel() {
        assert_eq!(calculate_fuel(12), 2);
        assert_eq!(calculate_fuel(14), 2);
        assert_eq!(calculate_fuel(1969), 654);
        assert_eq!(calculate_fuel(100756), 33583);
    }

    #[test]
    fn test_calculate_fuel_for_fuel() {
        assert_eq!(calculate_fuel_for_fuel(14), 2);
        assert_eq!(calculate_fuel_for_fuel(1969), 966);
        assert_eq!(calculate_fuel_for_fuel(100756), 50346);
    }
}

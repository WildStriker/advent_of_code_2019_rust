use std::error;
use std::io;

use crate::shared::intcode;

pub fn part01<T>(reader: T, noun: isize, verb: isize) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut computer = intcode::Computer::new(&mem);
    computer.ram.insert(1, noun);
    computer.ram.insert(2, verb);

    match computer.run()? {
        intcode::HaltedState::Halt => Ok(format!("Day 02 Part 01 Answer: {}", computer.ram[&0])),
        state => Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            format!("Unexpected Halted State: {:?}", state),
        ))),
    }
}

pub fn part02<T>(reader: T, target: isize) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut comp = intcode::Computer::new(&mem);

    for noun in 1..=99 {
        for verb in 1..=99 {
            comp.ram.insert(1, noun);
            comp.ram.insert(2, verb);

            match comp.run()? {
                intcode::HaltedState::Halt => {
                    if comp.ram[&0] == target {
                        return Ok(format!("Day 02 Part 02 Answer: {}", 100 * noun + verb));
                    };
                }
                state => {
                    return Err(Box::new(io::Error::new(
                        io::ErrorKind::Other,
                        format!("Unexpected Halted State: {:?}", state),
                    )))
                }
            }

            comp.reset();
        }
    }
    Err(Box::new(io::Error::new(
        io::ErrorKind::Other,
        format!("Unable to find a noun and verb to get target of {}", target),
    )))
}

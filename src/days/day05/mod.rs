use std::error;
use std::io;

use crate::shared::intcode;

pub fn part01<T>(reader: T, input: i32) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut computer = intcode::Computer::new(&mem);

    let mut last_output = None;
    loop {
        match computer.run()? {
            intcode::HaltedState::Halt => {
                return Ok(format!(
                    "Day 05 Part 01 Answer: {}",
                    last_output
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No Output!"))?
                ))
            }
            intcode::HaltedState::Input => computer.send_input(input),
            intcode::HaltedState::Output(output) => last_output = Some(output),
            unknown => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unexpected Halted State: {:?}", unknown),
                )))
            }
        }
    }
}

pub fn part02<T>(reader: T, input: i32) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut computer = intcode::Computer::new(&mem);

    let mut last_output = None;
    loop {
        match computer.run()? {
            intcode::HaltedState::Halt => {
                return Ok(format!(
                    "Day 05 Part 01 Answer: {}",
                    last_output
                        .ok_or_else(|| io::Error::new(io::ErrorKind::Other, "No Output!"))?
                ))
            }
            intcode::HaltedState::Input => computer.send_input(input),
            intcode::HaltedState::Output(output) => last_output = Some(output),
            unknown => {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::Other,
                    format!("Unexpected Halted State: {:?}", unknown),
                )))
            }
        }
    }
}

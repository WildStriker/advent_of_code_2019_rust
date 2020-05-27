use std::error;
use std::io;

use crate::shared::intcode;

pub fn part01<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut computer = intcode::Computer::new(&mem);

    let mut last_output = 0;
    loop {
        match computer.run()? {
            intcode::HaltedState::Halt => break,
            intcode::HaltedState::Input => computer.send_input(1),
            intcode::HaltedState::Output(output) => last_output = output,
        }
    }

    Ok(format!("Day 09 Part 01 Answer: {}", last_output))
}

pub fn part02<T>(reader: T) -> Result<String, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mem = intcode::parse_mem(reader)?;
    let mut computer = intcode::Computer::new(&mem);

    let mut last_output = 0;
    loop {
        match computer.run()? {
            intcode::HaltedState::Halt => break,
            intcode::HaltedState::Input => computer.send_input(1),
            intcode::HaltedState::Output(output) => last_output = output,
        }
    }

    Ok(format!("Day 09 Part 01 Answer: {}", last_output))
}

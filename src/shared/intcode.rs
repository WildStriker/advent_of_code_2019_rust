use std::{error, io};

enum OpCode {
    Add {
        r_ptr1: usize,
        r_ptr2: usize,
        w_ptr: usize,
    },
    Mul {
        r_ptr1: usize,
        r_ptr2: usize,
        w_ptr: usize,
    },
    Halt,
}

/// Halted States for calling code
#[derive(Debug)]
pub enum HaltedState {
    Halt,
}

/// IntCode State Machine
pub struct Computer<'a> {
    rom: &'a Vec<i32>,
    pub ram: Vec<i32>,
    main_pointer: usize,
}

impl<'a> Computer<'a> {
    /// Initialize a new Computer
    pub fn new(rom: &'a Vec<i32>) -> Computer<'a> {
        Computer {
            rom,
            ram: rom.clone(),
            main_pointer: 0,
        }
    }

    /// Run Computer, returns a halted state for caller to act on.
    pub fn run(&mut self) -> Result<HaltedState, Box<dyn error::Error>> {
        loop {
            let opcode = self.read_instruction()?;
            if let Some(state) = self.execute_instruction(opcode) {
                return Ok(state);
            }
        }
    }

    /// Reset the Computer State
    pub fn reset(&mut self) {
        self.ram = self.rom.clone();
        self.main_pointer = 0;
    }
    /// Read value of current pointer and move to next
    fn advance_ptr(&mut self) -> usize {
        let val = self.ram[self.main_pointer];
        self.main_pointer += 1;
        val as usize
    }

    /// Read instructions to determine Opcode
    /// Reading instruction will advance the main pointer
    fn read_instruction(&mut self) -> Result<OpCode, Box<dyn error::Error>> {
        let code = self.advance_ptr();

        match code {
            1 => {
                let r_ptr1 = self.advance_ptr();
                let r_ptr2 = self.advance_ptr();
                let w_ptr = self.advance_ptr();

                Ok(OpCode::Add {
                    r_ptr1,
                    r_ptr2,
                    w_ptr,
                })
            }
            2 => {
                let r_ptr1 = self.advance_ptr();
                let r_ptr2 = self.advance_ptr();
                let w_ptr = self.advance_ptr();

                Ok(OpCode::Mul {
                    r_ptr1,
                    r_ptr2,
                    w_ptr,
                })
            }
            99 => Ok(OpCode::Halt),
            _ => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown Opcode Instruction: {}", code),
            ))),
        }
    }

    /// Execute a given instruction
    fn execute_instruction(&mut self, instruction: OpCode) -> Option<HaltedState> {
        match instruction {
            OpCode::Add {
                r_ptr1,
                r_ptr2,
                w_ptr,
            } => {
                self.ram[w_ptr] = self.ram[r_ptr1] + self.ram[r_ptr2];
                None
            }
            OpCode::Mul {
                r_ptr1,
                r_ptr2,
                w_ptr,
            } => {
                self.ram[w_ptr] = self.ram[r_ptr1] * self.ram[r_ptr2];
                None
            }
            OpCode::Halt => Some(HaltedState::Halt),
        }
    }
}

/// Helper function to parse memory input from file
pub fn parse_mem<T>(mut reader: T) -> Result<Vec<i32>, Box<dyn error::Error>>
where
    T: io::BufRead,
{
    let mut mem: Vec<i32> = Vec::new();

    let mut line = String::new();
    reader.read_to_string(&mut line)?;

    for code in line.split(",") {
        mem.push(code.parse()?);
    }

    Ok(mem)
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_parse() {
        let int_string = "1,2,3,4,5";
        let expected = vec![1, 2, 3, 4, 5];

        let actual = parse_mem(int_string.as_bytes()).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    pub fn test_reset() {
        let expected_ram = vec![1, 2, 3, 4];
        let mut test_computer = Computer::new(&expected_ram);
        test_computer.ram = vec![1, 1, 1, 1];
        test_computer.main_pointer = 99;

        test_computer.reset();

        assert_eq!(test_computer.ram, expected_ram);
        assert_eq!(test_computer.main_pointer, 0);
    }

    #[test]
    pub fn test_advance_ptr() {
        let test_ram = vec![1, 2, 3, 4];
        let mut test_computer = Computer::new(&test_ram);

        for index in 0..test_ram.len() {
            let val = test_computer.advance_ptr();
            assert_eq!(val, test_ram[index] as usize);
            assert_eq!(test_computer.main_pointer, index + 1);
        }
    }

    #[test]
    pub fn test_add() {
        let test_ram = vec![1, 3, 1, 0];
        let mut test_computer = Computer::new(&test_ram);
        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::Add {
                r_ptr1: 3,
                r_ptr2: 1,
                w_ptr: 0,
            }
        ));

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[0], 3);
        assert_eq!(test_computer.main_pointer, 4);
    }

    #[test]
    pub fn test_mul() {
        let test_ram = vec![2, 2, 4, 0, 8];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::Mul {
                r_ptr1: 2,
                r_ptr2: 4,
                w_ptr: 0,
            }
        ));

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[0], 32);
        assert_eq!(test_computer.main_pointer, 4);
    }

    #[test]
    pub fn test_halt() {
        let test_ram = vec![99];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();
        assert!(matches!(opcode, OpCode::Halt));

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, Some(HaltedState::Halt)));
        assert_eq!(test_computer.ram, test_ram);
        assert_eq!(test_computer.main_pointer, 1);
    }
}

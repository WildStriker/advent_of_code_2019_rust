use std::{error, io};

enum ParameterMode {
    Position,
    Immediate,
}

/// Outputs Parameter Mode
/// 0 == Position Mode
/// 1 == Immediate Mode
/// once modes has been depleted always returns an Immediate Mode
struct ParameterModeParser {
    modes: usize,
}

impl ParameterModeParser {
    fn new(modes: usize) -> Self {
        Self { modes }
    }
}

impl Iterator for ParameterModeParser {
    type Item = Result<ParameterMode, String>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.modes == 0 {
            return Some(Ok(ParameterMode::Immediate));
        }

        let mode = self.modes % 10;
        self.modes /= 10;

        match mode {
            0 => Some(Ok(ParameterMode::Immediate)),
            1 => Some(Ok(ParameterMode::Position)),
            x => Some(Err(format!("Unknown Parameter Mode {}", x))),
        }
    }
}

enum OpCode {
    Add { value_1: i32, value_2: i32 },
    Mul { value_1: i32, value_2: i32 },
    Input { input: Option<i32> },
    Output { value_1: i32 },
    JumpIfTrue { value_1: i32, value_2: i32 },
    JumpIfFalse { value_1: i32, value_2: i32 },
    LessThan { value_1: i32, value_2: i32 },
    Equals { value_1: i32, value_2: i32 },
    Halt,
}

/// Halted States for calling code
#[derive(Debug)]
pub enum HaltedState {
    Input,
    Output(i32),
    Halt,
}

/// IntCode State Machine
pub struct Computer<'a> {
    rom: &'a Vec<i32>,
    pub ram: Vec<i32>,
    main_pointer: usize,
    w_ptr: Option<usize>,
}

impl<'a> Computer<'a> {
    /// Initialize a new Computer
    pub fn new(rom: &'a Vec<i32>) -> Computer<'a> {
        Computer {
            rom,
            ram: rom.clone(),
            main_pointer: 0,
            w_ptr: None,
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
    fn advance_ptr(&mut self) -> i32 {
        let val = self.ram[self.main_pointer];
        self.main_pointer += 1;
        val
    }

    /// Advances main pointer and retrieves parameter value
    fn read_parameter(&mut self, modes: &mut ParameterModeParser) -> Result<i32, String> {
        let val = self.advance_ptr();
        match modes.next().unwrap()? {
            ParameterMode::Immediate => Ok(self.ram[val as usize]),
            ParameterMode::Position => Ok(val),
        }
    }

    /// Advances main pointer and sets the write pointer
    fn set_write_pointer(&mut self, modes: &mut ParameterModeParser) -> Result<(), String> {
        let val = self.advance_ptr() as usize;
        let w_ptr = match modes.next().unwrap()? {
            ParameterMode::Immediate => val,
            ParameterMode::Position => {
                return Err("Output pointers do not support Position Mode!".to_string())
            }
        };

        self.w_ptr = Some(w_ptr);
        Ok(())
    }

    /// Write value to the current write pointers location
    fn write(&mut self, value: i32) {
        self.ram[self.w_ptr.unwrap()] = value;
    }

    /// Read instructions to determine Opcode
    /// Reading instruction will advance the main pointer
    fn read_instruction(&mut self) -> Result<OpCode, Box<dyn error::Error>> {
        let mode_code = self.advance_ptr() as usize;
        let code = mode_code % 100;
        let mut modes = ParameterModeParser::new(mode_code / 100);

        match code {
            1 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.set_write_pointer(&mut modes)?;
                Ok(OpCode::Add { value_1, value_2 })
            }
            2 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.set_write_pointer(&mut modes)?;

                Ok(OpCode::Mul { value_1, value_2 })
            }
            3 => {
                self.set_write_pointer(&mut modes)?;

                Ok(OpCode::Input { input: None })
            }
            4 => {
                let value_1 = self.read_parameter(&mut modes)?;
                self.w_ptr = None;

                Ok(OpCode::Output { value_1 })
            }
            5 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.w_ptr = None;
                Ok(OpCode::JumpIfTrue { value_1, value_2 })
            }
            6 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.w_ptr = None;
                Ok(OpCode::JumpIfFalse { value_1, value_2 })
            }
            7 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.set_write_pointer(&mut modes)?;
                Ok(OpCode::LessThan { value_1, value_2 })
            }
            8 => {
                let value_1 = self.read_parameter(&mut modes)?;
                let value_2 = self.read_parameter(&mut modes)?;
                self.set_write_pointer(&mut modes)?;
                Ok(OpCode::Equals { value_1, value_2 })
            }
            99 => {
                self.w_ptr = None;
                Ok(OpCode::Halt)
            }
            _ => Err(Box::new(io::Error::new(
                io::ErrorKind::Other,
                format!("Unknown Opcode Instruction: {}", code),
            ))),
        }
    }

    /// Execute a given instruction
    fn execute_instruction(&mut self, instruction: OpCode) -> Option<HaltedState> {
        match instruction {
            OpCode::Add { value_1, value_2 } => {
                self.write(value_1 + value_2);
                None
            }
            OpCode::Mul { value_1, value_2 } => {
                self.write(value_1 * value_2);
                None
            }
            OpCode::Input { input: Some(input) } => {
                self.write(input);
                None
            }
            OpCode::Input { input: None } => Some(HaltedState::Input),
            OpCode::Output { value_1 } => Some(HaltedState::Output(value_1)),
            OpCode::JumpIfTrue { value_1, value_2 } => {
                if value_1 != 0 {
                    self.main_pointer = value_2 as usize;
                }
                None
            }
            OpCode::JumpIfFalse { value_1, value_2 } => {
                if value_1 == 0 {
                    self.main_pointer = value_2 as usize;
                }
                None
            }
            OpCode::LessThan { value_1, value_2 } => {
                if value_1 < value_2 {
                    self.write(1);
                } else {
                    self.write(0);
                }
                None
            }
            OpCode::Equals { value_1, value_2 } => {
                if value_1 == value_2 {
                    self.write(1);
                } else {
                    self.write(0);
                }
                None
            }
            OpCode::Halt => Some(HaltedState::Halt),
        }
    }

    /// Execute an input to the current input
    pub fn send_input(&mut self, input: i32) {
        let input = Some(input);
        self.execute_instruction(OpCode::Input { input });
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
            assert_eq!(val, test_ram[index]);
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
                value_1: 0,
                value_2: 3,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 0);

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
                value_1: 4,
                value_2: 8,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 0);

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[0], 32);
        assert_eq!(test_computer.main_pointer, 4);
    }

    #[test]
    pub fn test_input() {
        let test_ram = vec![3, 2, 0];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(opcode, OpCode::Input { input: None }));
        assert_eq!(test_computer.w_ptr.unwrap(), 2);

        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result.unwrap(), HaltedState::Input));
        test_computer.send_input(99);
        assert_eq!(test_computer.ram[2], 99);
    }

    #[test]
    pub fn test_output() {
        let test_ram = vec![4, 2, 1000];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(opcode, OpCode::Output { value_1: 1000 }));
        assert_eq!(test_computer.w_ptr, None);

        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result.unwrap(), HaltedState::Output(1000)));
    }

    #[test]
    pub fn test_jump_if_true() {
        let test_ram = vec![5, 1, 3, 0];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::JumpIfTrue {
                value_1: 1,
                value_2: 0,
            }
        ));
        assert_eq!(test_computer.main_pointer, 3);
        assert_eq!(test_computer.w_ptr, None);

        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result, None));
        assert_eq!(test_computer.main_pointer, 0);

        let test_ram = vec![5, 3, 0, 0];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::JumpIfTrue {
                value_1: 0,
                value_2: 5,
            }
        ));
        assert_eq!(test_computer.main_pointer, 3);
        assert_eq!(test_computer.w_ptr, None);
        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result, None));
        assert_eq!(test_computer.main_pointer, 3);
    }

    #[test]
    pub fn test_jump_if_false() {
        let test_ram = vec![6, 1, 3, 0];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::JumpIfFalse {
                value_1: 1,
                value_2: 0,
            }
        ));
        assert_eq!(test_computer.main_pointer, 3);
        assert_eq!(test_computer.w_ptr, None);

        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result, None));
        assert_eq!(test_computer.main_pointer, 3);

        let test_ram = vec![6, 3, 0, 0];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::JumpIfFalse {
                value_1: 0,
                value_2: 6,
            }
        ));
        assert_eq!(test_computer.main_pointer, 3);
        assert_eq!(test_computer.w_ptr, None);
        let result = test_computer.execute_instruction(opcode);
        assert!(matches!(result, None));
        assert_eq!(test_computer.main_pointer, 6);
    }

    #[test]
    pub fn test_less_than() {
        let test_ram = vec![7, 1, 2, 4, 18];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::LessThan {
                value_1: 1,
                value_2: 2,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 4);

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[4], 1);
        assert_eq!(test_computer.main_pointer, 4);
        let test_ram = vec![7, 0, 3, 4, 18];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::LessThan {
                value_1: 7,
                value_2: 4,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 4);

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[4], 0);
        assert_eq!(test_computer.main_pointer, 4);
    }

    #[test]
    pub fn test_equals() {
        let test_ram = vec![8, 1, 1, 4, 18];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::Equals {
                value_1: 1,
                value_2: 1,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 4);

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[4], 1);
        assert_eq!(test_computer.main_pointer, 4);
        let test_ram = vec![8, 0, 3, 4, 18];
        let mut test_computer = Computer::new(&test_ram);

        let opcode = test_computer.read_instruction().unwrap();

        assert!(matches!(
            opcode,
            OpCode::Equals {
                value_1: 8,
                value_2: 4,
            }
        ));
        assert_eq!(test_computer.w_ptr.unwrap(), 4);

        let result = test_computer.execute_instruction(opcode);

        assert!(matches!(result, None));
        assert_eq!(test_computer.ram[4], 0);
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

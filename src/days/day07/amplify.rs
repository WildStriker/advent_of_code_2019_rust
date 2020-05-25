use crate::shared::intcode;
use itertools::Itertools;
use std::error;
use std::io;

pub struct Amps<'a> {
    amps: Vec<intcode::Computer<'a>>,
    phases: Vec<i32>,
}

impl<'a> Amps<'a> {
    pub fn new(mem: &'a Vec<i32>, phases: Vec<i32>) -> Result<Self, Box<dyn error::Error>> {
        let mut amps = Vec::new();
        for _ in 0..phases.len() {
            amps.push(intcode::Computer::new(&mem));
        }

        Ok(Self { amps, phases })
    }

    pub fn run(&mut self) -> Result<i32, Box<dyn error::Error>> {
        let sequences = (*self.phases)
            .iter()
            .permutations(self.phases.len())
            .unique();

        let mut max_output = 0;
        for sequence in sequences {
            for (index, phase_setting) in sequence.iter().enumerate() {
                self.amps[index].reset();
                match self.amps[index].run()? {
                    intcode::HaltedState::Input => {
                        self.amps[index].send_input(**phase_setting);
                    }
                    _ => {
                        return Err(format!(
                            "Expected amp #{} accept phase setting input {}",
                            index, phase_setting
                        )
                        .into())
                    }
                }
            }
            let mut last_output = 0;
            let mut is_running = true;
            while is_running {
                for amp in &mut self.amps {
                    match amp.run()? {
                        intcode::HaltedState::Halt => {
                            is_running = false;
                            break;
                        }
                        intcode::HaltedState::Input => {
                            amp.send_input(last_output);
                            match amp.run()? {
                                intcode::HaltedState::Output(output) => {
                                    last_output = output;
                                    if last_output > max_output {
                                        max_output = last_output;
                                    }
                                }
                                unexpected => {
                                    return Err(format!(
                                        "Expected output but got State: {:?}",
                                        unexpected
                                    )
                                    .into())
                                }
                            }
                        }
                        unexpected => {
                            return Err(format!("Unexpected State: {:?}", unexpected).into())
                        }
                    }
                }
            }
        }
        Ok(max_output)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use intcode;

    #[test]
    fn test_amps() {
        // Max thruster signal 43210 (from phase setting sequence 4,3,2,1,0):
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".as_bytes();
        let expected = 43210;

        let mem = intcode::parse_mem(input).unwrap();
        let mut amps = Amps::new(&mem, vec![0, 1, 2, 3, 4]).unwrap();
        let actual = amps.run().unwrap();
        assert_eq!(expected, actual);

        // Max thruster signal 54321 (from phase setting sequence 0,1,2,3,4):
        let input =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0".as_bytes();
        let expected = 54321;

        let mem = intcode::parse_mem(input).unwrap();
        let mut amps = Amps::new(&mem, vec![0, 1, 2, 3, 4]).unwrap();
        let actual = amps.run().unwrap();
        assert_eq!(expected, actual);

        // Max thruster signal 65210 (from phase setting sequence 1,0,4,3,2):
        let input = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,\
        33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .as_bytes();
        let expected = 65210;

        let mem = intcode::parse_mem(input).unwrap();
        let mut amps = Amps::new(&mem, vec![0, 1, 2, 3, 4]).unwrap();
        let actual = amps.run().unwrap();
        assert_eq!(expected, actual);

        // Max thruster signal 139629729 (from phase setting sequence 9,8,7,6,5):
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,\
            28,-1,28,1005,28,6,99,0,0,5"
            .as_bytes();
        let expected = 139629729;

        let mem = intcode::parse_mem(input).unwrap();
        let mut amps = Amps::new(&mem, vec![5, 6, 7, 8, 9]).unwrap();
        let actual = amps.run().unwrap();
        assert_eq!(expected, actual);

        // Max thruster signal 18216 (from phase setting sequence 9,7,8,5,6):
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,\
        1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,\
        53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
            .as_bytes();
        let expected = 18216;

        let mem = intcode::parse_mem(input).unwrap();
        let mut amps = Amps::new(&mem, vec![5, 6, 7, 8, 9]).unwrap();
        let actual = amps.run().unwrap();
        assert_eq!(expected, actual);
    }
}

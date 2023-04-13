use std::fmt;

pub struct NonUsedInstructionError {
    pub pc: u16,
    pub instruction: u16,
}

impl fmt::Display for NonUsedInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Non used instruction: 0x{:03X} | 0x{:04X}",
            self.pc - 2,
            self.instruction
        )
    }
}

pub struct QuitGameError;

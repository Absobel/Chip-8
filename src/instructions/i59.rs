use super::super::custom_errors::NonUsedInstructionError;
use super::super::launch_options::*;
use super::super::memory::Memory;

pub fn r(
    instruction: u16,
    pc: &mut u16,
    opcode: u16,
    memory: &mut Memory,
) -> Result<(), NonUsedInstructionError> {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);

    if opcode == 5 || opcode == 9 {
        let condition_met = if opcode == 5 { VX == VY } else { VX != VY };

        if condition_met {
            *pc += 2;
        }

        if DEBUG {
            let skip_action = if condition_met {
                "Skipping"
            } else {
                "Not skipping"
            };
            let condition_text = if opcode == 5 { "==" } else { "!=" };
            println!(
                "0x{:03X} | 0x{:04X} | {skip_action} next instruction because V{:X} {condition_text} V{:X}",
                *pc - 2,
                instruction,
                X,
                Y
            );
        }
        return Ok(());
    }

    Err(NonUsedInstructionError {
        pc: *pc - 2,
        instruction,
    })
}

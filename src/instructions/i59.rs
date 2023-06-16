use std::sync::{Arc, Mutex};

use crate::custom_errors::NonUsedInstructionError;
use crate::launch_options::*;
use crate::memory::Memory;

pub fn r(
    instruction: u16,
    pc: &mut u16,
    opcode: u16,
    mutex_memory: &Arc<Mutex<Memory>>,
    V_adr: &[u16; 16],
) -> Result<(), NonUsedInstructionError> {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_adr[X]);
    let VY = guard.read(V_adr[Y]);
    std::mem::drop(guard);

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

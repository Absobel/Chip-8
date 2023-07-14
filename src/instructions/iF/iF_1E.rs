use std::sync::{Arc, Mutex};

use crate::constants::*;
use crate::launch_options::*;
use crate::memory::Memory;

// 0xFX1E add VX to I with carry flag if CB_BNNN = NEW
pub fn r(instruction: u16, pc: &mut u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_ADR[X]);
    let new_I = guard.read_word(I_ADR) as usize + VX as usize;
    if CB_FX1E == CB::NEW && new_I > 0xFFF {
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Adding V{:01X} to I with carry flag",
                *pc - 2,
                instruction,
                X
            );
        }
        guard.write(V_ADR[0xF], 1);
    } else if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding V{:01X} to I",
            *pc - 2,
            instruction,
            X
        );
    }
    guard.write_word(I_ADR, (new_I % 0x1000) as u16);
    std::mem::drop(guard);
}

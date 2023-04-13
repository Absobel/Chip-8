use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn r(instruction: u16, pc: &mut u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16]) {
    let NNN = instruction & 0x0FFF;
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if CB_B_NN == CB::OLD {
        // 0xBNNN jump to 0x0NNN + V0
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0",
                *pc - 2,
                instruction,
                NNN
            );
        }

        let guard = mutex_memory.lock().unwrap();
        let V0 = guard.read(V_adr[0]);
        std::mem::drop(guard);

        *pc = NNN + V0 as u16;
    } else if CB_B_NN == CB::NEW {
        // 0xBXNN jump to 0xXNN + VX
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}",
                *pc - 2,
                instruction,
                NNN,
                X
            );
        }

        let guard = mutex_memory.lock().unwrap();
        let VX = guard.read(V_adr[X]);
        std::mem::drop(guard);

        *pc = NNN + VX as u16;
    }
}

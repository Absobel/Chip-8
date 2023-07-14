use crate::constants::*;
use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let I = guard.read_word(I_ADR);
    if DEBUG {
        let (action, particle) = if instruction & 0x00FF == 0x0055 {
            ("Storing", "to")
        } else {
            ("Loading", "from")
        };
        println!("0x{:03X} | 0x{:04X} | {action} V0 through V{:01X} {particle} memory starting at address I", pc-2, instruction, X);
    }
    for (i, V_ADR_i) in V_ADR.iter().enumerate().take(X + 1) {
        let iu16 = i as u16;
        if instruction & 0x00FF == 0x0055 {
            let Vi = guard.read(*V_ADR_i);
            if DEBUG_VERBOSE {
                println!("               | Storing V{:01X} = 0x{:02X} ({Vi}) in memory at address {:03X}", i, Vi, I+i as u16);
            }
            guard.write(I + iu16, Vi);
        } else {
            /* instruction & 0x00FF == 0x0065 */
            let future_Vi = guard.read(I + iu16);
            if DEBUG_VERBOSE {
                println!("               | Storing memory at address {:03X} = 0x{:02X} ({future_Vi}) in V{:01X}", I+i as u16, future_Vi, i);
            }
            guard.write(*V_ADR_i, future_Vi);
        }
    }
    if CB_FX_5 == CB::OLD {
        guard.write_word(I_ADR, I + (X as u16) + 1);
    }
}

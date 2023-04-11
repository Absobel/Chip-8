use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn i8_6E(
    instruction: u16,
    pc: u16,
    mutex_memory: &Arc<Mutex<Memory>>,
    V_adr: &[u16; 16],
    X: usize,
    Y: usize,
) {
    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_adr[X]);
    let VY = guard.read(V_adr[Y]);
    guard.write(V_adr[0xF], VX & 0x1);

    if instruction & 0x000F == 6 {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            guard.write(V_adr[X], VY >> 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            guard.write(V_adr[X], VX >> 1);
        }
    } else if instruction & 0x000F == 0xE {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it left by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            guard.write(V_adr[X], VY << 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} left by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            guard.write(V_adr[X], VX << 1);
        }
    }
    std::mem::drop(guard);
}

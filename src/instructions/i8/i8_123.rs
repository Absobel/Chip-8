use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

// 0x8XY1 set VX to VX | VY
// 0x8XY2 set VX to VX & VY
// 0x8XY3 set VX to VX ^ VY
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16]) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_adr[X]);
    let VY = guard.read(V_adr[Y]);
    match instruction & 0x000F {
        1 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} | V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            guard.write(V_adr[X], VX | VY);
        }
        2 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} & V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            guard.write(V_adr[X], VX & VY);
        }
        3 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} ^ V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            guard.write(V_adr[X], VX ^ VY);
        }
        _ => unreachable!(),
    }
    std::mem::drop(guard);
}

use std::sync::{Arc, Mutex};

use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX15 set the delay timer to VX
// 0xFX18 set the sound timer to VX
pub fn r(instruction: u16, pc: &mut u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_ADR[X]);
    let which_timer = if instruction & 0x00FF == 0x0015 {
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Setting the delay timer to V{:01X}",
                *pc - 2,
                instruction,
                X
            );
        }
        TIMER_ADR
    } else {
        /* instruction & 0x00FF == 0x0018 */
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Setting the sound timer to V{:01X}",
                *pc - 2,
                instruction,
                X
            );
        }
        SOUND_ADR
    };
    guard.write(which_timer, VX);
    std::mem::drop(guard);
}

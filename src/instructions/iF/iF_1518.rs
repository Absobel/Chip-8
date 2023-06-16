use std::sync::{Arc, Mutex};

use crate::launch_options::*;
use crate::memory::Memory;

// 0xFX15 set the delay timer to VX
// 0xFX18 set the sound timer to VX
pub fn r(
    instruction: u16,
    pc: &mut u16,
    mutex_memory: &Arc<Mutex<Memory>>,
    V_adr: &[u16; 16],
    timer_adr: u16,
    sound_adr: u16,
) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_adr[X]);
    let which_timer = if instruction & 0x00FF == 0x0015 {
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Setting the delay timer to V{:01X}",
                *pc - 2,
                instruction,
                X
            );
        }
        timer_adr
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
        sound_adr
    };
    guard.write(which_timer, VX);
    std::mem::drop(guard);
}

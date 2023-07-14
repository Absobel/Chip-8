use std::sync::{Arc, Mutex};

use crate::constants::*;
use crate::launch_options::*;
use crate::memory::Memory;

// 0xFX29 set I to the location of the sprite for the character in VX
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting I to the location of the sprite for the character in V{:01X}", pc-2, instruction, X);
    }

    let mut guard = mutex_memory.lock().unwrap();
    let char_0x = guard.read(V_ADR[X]) & 0x0F;
    guard.write_word(I_ADR, (char_0x as u16) * 5 + 50);
    std::mem::drop(guard);
}

use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

// 0xANNN set I to 0x0NNN
pub fn r(mutex_memory: &Arc<Mutex<Memory>>, pc: u16, instruction: u16, I_adr: u16) {
    let NNN = instruction & 0x0FFF;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting I to 0x{:03X}",
            pc - 2,
            instruction,
            NNN
        );
    }

    let mut guard = mutex_memory.lock().unwrap();
    guard.write_word(I_adr, NNN);
    std::mem::drop(guard);
}
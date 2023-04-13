use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

// 0xFX07 set VX to the value of the delay timer
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16], timer_adr: u16) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to the value of the delay timer", pc-2, instruction, X);
    }

    let mut guard = mutex_memory.lock().unwrap();
    let timer_val = guard.read(timer_adr);
    guard.write(V_adr[X], timer_val);
    std::mem::drop(guard);
}
use crate::constants::*;
use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

// 0x6XNN set register VX to 0xNN
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting register V{:01X} to 0x{:02X} = {NN}",
            pc - 2,
            instruction,
            X,
            NN
        );
    }

    let mut guard = mutex_memory.lock().expect("Failed to lock memory");
    guard.write(V_ADR[X], NN as u8);
    std::mem::drop(guard);
}

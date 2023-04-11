use super::super::launch_options::*;
use super::super::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn i7(
    instruction: u16,
    pc: u16,
    mutex_memory: &Arc<Mutex<Memory>>,
    X: usize,
    NN: usize,
    V_adr: &[u16; 16],
) {
    // 0x7XNN add 0xNN to register VX (carry flag is not changed)
    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding 0x{:02X} to register V{:01X}",
            pc - 2,
            instruction,
            NN,
            X
        );
    }

    let mut guard = mutex_memory.lock().expect("Failed to lock memory");
    let VX = guard.read(V_adr[X]) as usize;
    guard.write(V_adr[X], (VX + NN) as u8);
    std::mem::drop(guard);
}

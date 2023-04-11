use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn i8_0(
    mutex_memory: &Arc<Mutex<Memory>>,
    pc: u16,
    instruction: &u16,
    X: usize,
    Y: usize,
    V_adr: &[u16; 16],
) {
    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X}",
            pc - 2,
            instruction,
            X,
            Y
        );
    }

    let mut guard = mutex_memory.lock().unwrap();
    let VY = guard.read(V_adr[Y]);
    guard.write(V_adr[X], VY);
    std::mem::drop(guard);
}

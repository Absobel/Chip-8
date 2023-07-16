use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

use std::sync::{Arc, Mutex};

// 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Storing the binary-coded decimal representation of V{:01X} at the addresses I, I+1, and I+2", pc-2, instruction, X);
    }

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_ADR[X]);
    let (digit_1, digit_2, digit_3) = (VX / 100, (VX / 10) % 10, VX % 10);
    let I = guard.read_word(I_ADR);
    guard.write(I, digit_1);
    guard.write(I + 1, digit_2);
    guard.write(I + 2, digit_3);
    std::mem::drop(guard);
}

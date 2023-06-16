use crate::launch_options::*;
use crate::memory::Memory;

use rand::Rng;
use std::sync::{Arc, Mutex};

// 0xCXNN set VX to random number and binary-AND's it with NN
pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16]) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to random number and binary-AND's it with 0x{:02X}", pc-2, instruction, X, NN);
    }

    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();

    let mut guard = mutex_memory.lock().unwrap();
    guard.write(V_adr[X], random & NN as u8);
    std::mem::drop(guard);
}
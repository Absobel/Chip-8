use crate::launch_options::*;
use crate::memory::Memory;

use std::sync::{Arc, Mutex};
use std::collections::HashMap;

// 0xFX0A wait for a key press, store the value of the key in VX
pub fn r(instruction: u16, pc: &mut u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16], dico_events: &HashMap<u8, bool>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Waiting for a key press, storing the value of the key in V{:01X}", *pc-2, instruction, X);
    }

    let mut key_pressed = 0xFF;
    for (key, state) in dico_events {
        if *state {
            key_pressed = *key;
            break;
        }
    }

    let mut guard = mutex_memory.lock().unwrap();
    if key_pressed != 0xFF {
        guard.write(V_adr[X], key_pressed);
    } else {
        *pc -= 2;
    }
    std::mem::drop(guard);
}
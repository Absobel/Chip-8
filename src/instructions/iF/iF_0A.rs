use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

use std::collections::HashMap;

// 0xFX0A wait for a key press, store the value of the key in VX
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory, dico_events: &HashMap<u8, bool>) {
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

    if key_pressed != 0xFF {
        memory.write_register(X, key_pressed);
    } else {
        *pc -= 2;
    }
}

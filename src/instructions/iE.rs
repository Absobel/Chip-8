use std::sync::{Arc, Mutex};
use std::collections::HashMap;

use crate::memory::Memory;
use crate::launch_options::*;
use crate::custom_errors::*;

// 0xEX9E skip next instruction if key with the value of VX is pressed
// 0xEXA1 skip next instruction if key with the value of VX is not pressed
pub fn r(instruction: u16, pc: &mut u16, mutex_memory: &Arc<Mutex<Memory>>, V_adr: &[u16; 16], dico_events: &HashMap<u8, bool>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    

    let guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_adr[X]);
    std::mem::drop(guard);

    let is_key_pressed_VX = *(dico_events.get(&VX).expect("VX devrait être dans le dico"));

    if instruction & 0x00FF == 0x009E {
        if is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", *pc-2, instruction, X, VX);
        } else {
            println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        }
    } else if instruction & 0x00FF == 0x00A1 {
        if !is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && !is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        } else {
            println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", *pc-2, instruction, X, VX);
        }
    } else {
        panic!(
            "{}",
            NonUsedInstructionError {
                pc: *pc - 2,
                instruction
            }
        )
    }
}
use std::collections::HashMap;

use super::super::custom_errors::*;
use super::super::launch_options::*;
use super::super::memory::Memory;

// 0xEX9E skip next instruction if key with the value of VX is pressed
// 0xEXA1 skip next instruction if key with the value of VX is not pressed
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory, dico_events: &HashMap<u8, bool>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let VX = memory.read_register(X);

    let is_key_pressed_VX = *(dico_events.get(&VX).expect("VX devrait être dans le dico"));

    if instruction & 0x00FF == 0x009E {
        if is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", *pc-2, instruction, X, VX);
        } else if DEBUG {
            println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        }
    } else if instruction & 0x00FF == 0x00A1 {
        if !is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && !is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        } else if DEBUG {
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

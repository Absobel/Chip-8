use crate::events::KeysState;
use crate::launch_options::*;
use crate::memory::Memory;

// 0xFX0A wait for a key press, store the value of the key in VX
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory, keys_state: &KeysState) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Waiting for a key press, storing the value of the key in V{:01X}", *pc-2, instruction, X);
    }

    match keys_state.is_key_pressed() {
        Some(key_pressed) => memory.write_register(X, key_pressed),
        None => *pc -= 2,
    }
}

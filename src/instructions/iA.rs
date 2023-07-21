use crate::launch_options::*;
use crate::memory::Memory;

// 0xANNN set I to 0x0NNN
pub fn r(memory: &mut Memory, pc: u16, instruction: u16) {
    let NNN = instruction & 0x0FFF;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting I to 0x{:03X}",
            pc - 2,
            instruction,
            NNN
        );
    }

    memory.write_adress(NNN);
}

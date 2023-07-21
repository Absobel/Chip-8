use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX1E add VX to I with carry flag if CB_BNNN = NEW
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let VX = memory.read_register(X);
    let new_I = memory.read_adress() as usize + VX as usize;
    if CB_FX1E == CB::NEW && new_I > 0xFFF {
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Adding V{:01X} to I with carry flag",
                *pc - 2,
                instruction,
                X
            );
        }
        memory.write_register(0xF, 1);
    } else if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding V{:01X} to I",
            *pc - 2,
            instruction,
            X
        );
    }
    memory.write_adress((new_I % 0x1000) as u16);
}

use super::super::launch_options::*;
use super::super::memory::Memory;

pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let NNN = instruction & 0x0FFF;
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if CB_B_NN == CB::OLD {
        // 0xBNNN jump to 0x0NNN + V0
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0",
                *pc - 2,
                instruction,
                NNN
            );
        }

        let V0 = memory.read_register(0);

        *pc = NNN + V0 as u16;
    } else if CB_B_NN == CB::NEW {
        // 0xBXNN jump to 0xXNN + VX
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}",
                *pc - 2,
                instruction,
                NNN,
                X
            );
        }

        let VX = memory.read_register(X);

        *pc = NNN + VX as u16;
    }
}

use crate::launch_options::*;
use crate::memory::Memory;

pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);
    memory.write_register(0xF, VX & 0x1);

    if instruction & 0x000F == 6 {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            memory.write_register(X, VY >> 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            memory.write_register(X, VX >> 1);
        }
    } else if instruction & 0x000F == 0xE {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it left by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            memory.write_register(X, VY << 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} left by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            memory.write_register(X, VX << 1);
        }
    }
}

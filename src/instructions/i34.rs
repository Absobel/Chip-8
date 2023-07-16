use super::super::constants::*;
use super::super::launch_options::*;
use super::super::memory::Memory;

// 0x3XNN skip next instruction if VX == NN
// 0x4XNN skip next instruction if VX != NN
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory, opcode: u16) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    let VX = memory.read(V_ADR[X]);

    if DEBUG {
        match (opcode, VX == NN as u8) {
            (3, true) => println!(
                "0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (3, false) => println!(
                "0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (4, true) => println!(
                "0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (4, false) => println!(
                "0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            _ => (),
        }
    }

    *pc += if (opcode == 3 && VX == NN as u8) || (opcode == 4 && VX != NN as u8) {
        2
    } else {
        0
    };
}

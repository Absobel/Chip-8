use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0x8XY1 set VX to VX | VY
// 0x8XY2 set VX to VX & VY
// 0x8XY3 set VX to VX ^ VY
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);
    match instruction & 0x000F {
        1 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} | V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX | VY);
        }
        2 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} & V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX & VY);
        }
        3 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} ^ V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX ^ VY);
        }
        _ => unreachable!(),
    }
}

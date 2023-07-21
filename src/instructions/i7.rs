use super::super::launch_options::*;
use super::super::memory::Memory;

// 0x7XNN add 0xNN to register VX (carry flag is not changed)
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding 0x{:02X} to register V{:01X}",
            pc - 2,
            instruction,
            NN,
            X
        );
    }

    let VX = memory.read_register(X) as usize;
    memory.write_register(X, (VX + NN) as u8);
}

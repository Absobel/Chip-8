use crate::launch_options::*;
use crate::memory::Memory;

// 0x6XNN set register VX to 0xNN
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting register V{:01X} to 0x{:02X} = {NN}",
            pc - 2,
            instruction,
            X,
            NN
        );
    }

    memory.write_register(X, NN as u8);
}

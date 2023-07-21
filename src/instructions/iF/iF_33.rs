use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Storing the binary-coded decimal representation of V{:01X} at the addresses I, I+1, and I+2", pc-2, instruction, X);
    }

    let VX = memory.read_register(X);
    let (digit_1, digit_2, digit_3) = (VX / 100, (VX / 10) % 10, VX % 10);
    let I = memory.read_adress();
    memory.write(I, digit_1);
    memory.write(I + 1, digit_2);
    memory.write(I + 2, digit_3);
}

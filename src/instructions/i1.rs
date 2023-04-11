use super::super::launch_options::*;

pub fn i1(instruction: u16, pc: &mut u16, NNN: u16) {
    // 0x1NNN jump to adress 0xNNN
    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Jumping to adress 0x{:03X}",
            *pc - 2,
            instruction,
            NNN
        );
    }
    *pc = NNN;
}

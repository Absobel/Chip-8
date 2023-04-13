use super::super::launch_options::*;

// 0x1NNN jump to adress 0xNNN
pub fn r(instruction: u16, pc: &mut u16) {
    let NNN = instruction & 0x0FFF;

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

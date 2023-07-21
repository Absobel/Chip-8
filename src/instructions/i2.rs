use crate::launch_options::*;

// 0x2NNN call subroutine at 0xNNN
pub fn r(instruction: u16, pc: &mut u16, stack: &mut Vec<u16>) {
    let NNN = instruction & 0x0FFF;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Calling subroutine at 0x{:03X}",
            *pc - 2,
            instruction,
            NNN
        );
    }
    stack.push(*pc);
    *pc = NNN;
}

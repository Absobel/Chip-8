use super::super::launch_options::*;

pub fn i2(instruction: u16, pc: &mut u16, stack: &mut Vec<u16>, NNN: u16) {
    // 0x2NNN call subroutine at 0xNNN
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

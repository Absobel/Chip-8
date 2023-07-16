use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX15 set the delay timer to VX
// 0xFX18 set the sound timer to VX
pub fn r(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting the sound timer to V{:01X}",
            *pc - 2,
            instruction,
            X
        );
    }

    let VX = memory.read(V_ADR[X]);

    if instruction & 0x00FF == 0x0015 {
        memory.write_delay_timer(VX);
    } else {
        memory.write_sound_timer(VX);
    }
}

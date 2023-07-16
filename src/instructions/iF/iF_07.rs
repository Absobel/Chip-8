use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX07 set VX to the value of the delay timer
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting V{:01X} to the value of the delay timer",
            pc - 2,
            instruction,
            X
        );
    }

    let timer_val = memory.read_delay_timer();
    memory.write(V_ADR[X], timer_val);
}

use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

// 0xFX29 set I to the location of the sprite for the character in VX
pub fn r(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting I to the location of the sprite for the character in V{:01X}", pc-2, instruction, X);
    }

    let char_0x = memory.read(V_ADR[X]) & 0x0F;
    memory.write_word(I_ADR, (char_0x as u16) * 5 + 50);
}

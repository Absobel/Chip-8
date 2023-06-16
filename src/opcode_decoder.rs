use crate::custom_errors::NonUsedInstructionError;
use crate::instructions::{i8::*, iF::*, *};
use crate::memory;
use crate::screen;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub fn decode(
    opcode: u16,
    instruction: u16,
    pc: &mut u16,
    stack: &mut Vec<u16>,
    screen: &mut screen::Screen,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    mutex_memory: &Arc<Mutex<memory::Memory>>,
    V_adr: &[u16; 16],
    I_adr: u16,
    timer_adr: u16,
    sound_adr: u16,
    dico_events: &HashMap<u8, bool>,
) {
    match opcode {
        // 0x00E0 : Clear screen
        // 0x00EE : Return from subroutine
        0 => {
            if let Err(e) = i0::r(instruction, pc, stack, screen, canvas) {
                panic!("{e}");
            }
        }
        // 0x1NNN jump to adress 0xNNN
        1 => i1::r(instruction, pc),
        // 0x2NNN call subroutine at 0xNNN
        2 => i2::r(instruction, pc, stack),
        // 0x3XNN skip next instruction if VX == NN
        // 0x4XNN skip next instruction if VX != NN
        3 | 4 => i34::r(instruction, pc, mutex_memory, opcode, V_adr),
        // 0x5XY0 skip next instruction if VX == VY
        // 0x9XY0 skip next instruction if VX != VY
        5 | 9 => {
            if let Err(e) = i59::r(instruction, pc, opcode, mutex_memory, V_adr) {
                panic!("{e}");
            }
        }
        // 0x6XNN set register VX to 0xNN
        6 => i6::r(instruction, *pc, mutex_memory, V_adr),
        // 0x7XNN add 0xNN to register VX (carry flag is not changed)
        7 => i7::r(instruction, *pc, mutex_memory, V_adr),
        0x8 => {
            match instruction & 0x000F {
                // 0x8XY0 set VX to VY
                0 => i8_0::r(mutex_memory, *pc, &instruction, V_adr),
                // 0x8XY1 set VX to VX | VY
                // 0x8XY2 set VX to VX & VY
                // 0x8XY3 set VX to VX ^ VY
                1 | 2 | 3 => i8_123::r(instruction, *pc, mutex_memory, V_adr),
                // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
                // 0x8XY7           VY - VX
                4 | 5 | 7 => i8_457::r(instruction, *pc, mutex_memory, V_adr),
                // 0x8XY6 OLD : VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                //        NEW : VX is shifted right by 1. VF is set to the bit shifted out
                // 0x8XYE OLD : VX is set to VY and shifted left by 1. VF is set to the bit shifted out
                //        NEW : VX is shifted left by 1. VF is set to the bit shifted out
                6 | 0xE => i8_6E::r(instruction, *pc, mutex_memory, V_adr),
                _ => panic!(
                    "{}",
                    NonUsedInstructionError {
                        pc: *pc - 2,
                        instruction
                    }
                ),
            }
        }
        // 0xANNN set I to 0x0NNN
        0xA => iA::r(mutex_memory, *pc, instruction, I_adr),
        // 0xBNNN OLD: jump to 0x0NNN + V0
        // 0xBXNN NEW: jump to 0xXNN + VX
        0xB => iB::r(instruction, pc, mutex_memory, V_adr),
        // 0xCXNN set VX to random number and binary-AND's it with NN
        0xC => iC::r(instruction, *pc, mutex_memory, V_adr),
        // 0xDXYN display sprite at (VX, VY) with width 8 and height N
        0xD => iD::r(mutex_memory, *pc, instruction, I_adr, V_adr, screen, canvas),
        // 0xEX9E skip next instruction if key with the value of VX is pressed
        // 0xEXA1 skip next instruction if key with the value of VX is not pressed
        0xE => iE::r(instruction, pc, mutex_memory, V_adr, dico_events),
        0xF => {
            match instruction & 0x00FF {
                // 0xFX07 set VX to the value of the delay timer
                0x0007 => iF_07::r(instruction, *pc, mutex_memory, V_adr, timer_adr),
                // 0xFX0A wait for a key press, store the value of the key in VX
                0x000A => iF_0A::r(instruction, pc, mutex_memory, V_adr, dico_events),
                // 0xFX15 set the delay timer to VX
                // 0xFX18 set the sound timer to VX
                0x0015 | 0x0018 => {
                    iF_1518::r(instruction, pc, mutex_memory, V_adr, timer_adr, sound_adr)
                }
                // 0xFX1E add VX to I with carry flag if CB_BNNN = NEW
                0x001E => iF_1E::r(instruction, pc, mutex_memory, V_adr, I_adr),
                // 0xFX29 set I to the location of the sprite for the character in VX
                0x0029 => iF_29::r(instruction, *pc, mutex_memory, V_adr, I_adr),
                // 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
                0x0033 => iF_33::r(instruction, *pc, mutex_memory, V_adr, I_adr),
                // 0xFX55 store V0 through VX in memory starting at address I
                // 0xFX65 store memory through V0 to VX starting at address I
                0x0055 | 0x0065 => iF_5565::r(instruction, *pc, mutex_memory, V_adr, I_adr),
                _ => {
                    panic!(
                        "{}",
                        NonUsedInstructionError {
                            pc: *pc - 2,
                            instruction,
                        },
                    );
                }
            }
        }
        _ => {
            unreachable!();
        }
    }
}

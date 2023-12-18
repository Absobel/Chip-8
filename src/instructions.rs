use rand::Rng;
use sdl2::render::{Canvas, WindowCanvas};
use sdl2::video::Window;

use crate::constants::*;
use crate::custom_errors::NonUsedInstructionError;
use crate::events::KeysState;
use crate::launch_options::*;
use crate::memory::Memory;
use crate::screen::Screen;
use crate::{display, screen};

// TODO : Move documentation to funcitons

pub fn decode(
    pc: &mut u16,
    stack: &mut Vec<u16>,
    screen: &mut screen::Screen,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    memory: &mut Memory,
    keys_state: &KeysState,
) -> Result<(), NonUsedInstructionError> {
    let instruction = memory.read_word(*pc);

    *pc += 2;
    let opcode = (instruction & 0xF000) >> 12;

    match opcode {
        // 0x00E0 : Clear screen
        // 0x00EE : Return from subroutine
        0 => i0(instruction, pc, stack, screen, canvas)?,
        // 0x1NNN jump to adress 0xNNN
        1 => i1(instruction, pc),
        // 0x2NNN call subroutine at 0xNNN
        2 => i2(instruction, pc, stack),
        // 0x3XNN skip next instruction if VX == NN
        // 0x4XNN skip next instruction if VX != NN
        3 | 4 => i34(instruction, pc, memory, opcode),
        // 0x5XY0 skip next instruction if VX == VY
        // 0x9XY0 skip next instruction if VX != VY
        5 | 9 => i59(instruction, pc, opcode, memory)?,
        // 0x6XNN set register VX to 0xNN
        6 => i6(instruction, *pc, memory),
        // 0x7XNN add 0xNN to register VX (carry flag is not changed)
        7 => i7(instruction, *pc, memory),
        0x8 => {
            match instruction & 0x000F {
                // 0x8XY0 set VX to VY
                0 => i8_0(memory, *pc, &instruction),
                // 0x8XY1 set VX to VX | VY
                // 0x8XY2 set VX to VX & VY
                // 0x8XY3 set VX to VX ^ VY
                1..=3 => i8_123(instruction, *pc, memory),
                // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
                // 0x8XY7           VY - VX
                4 | 5 | 7 => i8_457(instruction, *pc, memory),
                // 0x8XY6 OLD : VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                //        NEW : VX is shifted right by 1. VF is set to the bit shifted out
                // 0x8XYE OLD : VX is set to VY and shifted left by 1. VF is set to the bit shifted out
                //        NEW : VX is shifted left by 1. VF is set to the bit shifted out
                6 | 0xE => i8_6E(instruction, *pc, memory),
                _ => {
                    return Err(NonUsedInstructionError {
                        pc: *pc - 2,
                        instruction,
                    })
                }
            }
        }
        // 0xANNN set I to 0x0NNN
        0xA => iA(memory, *pc, instruction),
        // 0xBNNN OLD: jump to 0x0NNN + V0
        // 0xBXNN NEW: jump to 0xXNN + VX
        0xB => iB(instruction, pc, memory),
        // 0xCXNN set VX to random number and binary-AND's it with NN
        0xC => iC(instruction, *pc, memory),
        // 0xDXYN display sprite at (VX, VY) with width 8 and height N
        0xD => iD(memory, *pc, instruction, screen, canvas),
        // 0xEX9E skip next instruction if key with the value of VX is pressed
        // 0xEXA1 skip next instruction if key with the value of VX is not pressed
        0xE => iE(instruction, pc, memory, keys_state),
        0xF => {
            match instruction & 0x00FF {
                // 0xFX07 set VX to the value of the delay timer
                0x0007 => iF_07(instruction, *pc, memory),
                // 0xFX0A wait for a key press, store the value of the key in VX
                0x000A => iF_0A(instruction, pc, memory, keys_state),
                // 0xFX15 set the delay timer to VX
                // 0xFX18 set the sound timer to VX
                0x0015 | 0x0018 => iF_1518(instruction, pc, memory),
                // 0xFX1E add VX to I with carry flag if CB_BNNN = NEW
                0x001E => iF_1E(instruction, pc, memory),
                // 0xFX29 set I to the location of the sprite for the character in VX
                0x0029 => iF_29(instruction, *pc, memory),
                // 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
                0x0033 => iF_33(instruction, *pc, memory),
                // 0xFX55 store V0 through VX in memory starting at address I
                // 0xFX65 store memory through V0 to VX starting at address I
                0x0055 | 0x0065 => iF_5565(instruction, *pc, memory),
                _ => {
                    return Err(NonUsedInstructionError {
                        pc: *pc - 2,
                        instruction,
                    })
                }
            }
        }
        _ => {
            unreachable!();
        }
    }

    Ok(())
}

pub fn i0(
    instruction: u16,
    pc: &mut u16,
    stack: &mut Vec<u16>,
    screen: &mut screen::Screen,
    canvas: &mut Canvas<Window>,
) -> Result<(), NonUsedInstructionError> {
    match instruction {
        // Clear screen
        0x00E0 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Screen clearing",
                    *pc - 2,
                    instruction
                );
            }
            screen.clear();
            display::clear_screen(canvas)
        }
        // Return from subroutine
        0x00EE => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Returning from subroutine",
                    *pc - 2,
                    instruction
                );
            }
            *pc = stack.pop().unwrap();
        }
        _ => {
            return Err(NonUsedInstructionError {
                pc: *pc,
                instruction,
            });
        }
    }
    Ok(())
}

pub fn i1(instruction: u16, pc: &mut u16) {
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

pub fn i2(instruction: u16, pc: &mut u16, stack: &mut Vec<u16>) {
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

pub fn i34(instruction: u16, pc: &mut u16, memory: &mut Memory, opcode: u16) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    let VX = memory.read_register(X);

    if DEBUG {
        match (opcode, VX == NN as u8) {
            (3, true) => println!(
                "0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (3, false) => println!(
                "0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (4, true) => println!(
                "0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            (4, false) => println!(
                "0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != 0x{:02X}",
                *pc - 2,
                instruction,
                X,
                NN
            ),
            _ => (),
        }
    }

    *pc += if (opcode == 3 && VX == NN as u8) || (opcode == 4 && VX != NN as u8) {
        2
    } else {
        0
    };
}

pub fn i59(
    instruction: u16,
    pc: &mut u16,
    opcode: u16,
    memory: &mut Memory,
) -> Result<(), NonUsedInstructionError> {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);

    if opcode == 5 || opcode == 9 {
        let condition_met = if opcode == 5 { VX == VY } else { VX != VY };

        if condition_met {
            *pc += 2;
        }

        if DEBUG {
            let skip_action = if condition_met {
                "Skipping"
            } else {
                "Not skipping"
            };
            let condition_text = if opcode == 5 { "==" } else { "!=" };
            println!(
                "0x{:03X} | 0x{:04X} | {skip_action} next instruction because V{:X} {condition_text} V{:X}",
                *pc - 2,
                instruction,
                X,
                Y
            );
        }
        return Ok(());
    }

    Err(NonUsedInstructionError {
        pc: *pc - 2,
        instruction,
    })
}

pub fn i6(instruction: u16, pc: u16, memory: &mut Memory) {
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

pub fn i7(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding 0x{:02X} to register V{:01X}",
            pc - 2,
            instruction,
            NN,
            X
        );
    }

    let VX = memory.read_register(X) as usize;
    memory.write_register(X, (VX + NN) as u8);
}

pub fn i8_0(memory: &mut Memory, pc: u16, instruction: &u16) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X}",
            pc - 2,
            instruction,
            X,
            Y
        );
    }

    let VY = memory.read_register(Y);
    memory.write_register(X, VY);
}

pub fn i8_123(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);
    match instruction & 0x000F {
        1 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} | V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX | VY);
        }
        2 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} & V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX & VY);
        }
        3 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} ^ V{:01X}",
                    pc - 2,
                    instruction,
                    X,
                    X,
                    Y
                );
            }
            memory.write_register(X, VX ^ VY);
        }
        _ => unreachable!(),
    }
}

pub fn i8_457(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);
    match instruction & 0x000F {
        // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
        4 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Adding V{:01X} to V{:01X} with carry flag to VF",
                    pc - 2,
                    instruction,
                    Y,
                    X
                );
            }
            let (result, carry) = VX.overflowing_add(VY);
            memory.write_register(X, result);
            memory.write_register(0xF, carry as u8);
        }
        // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
        // 0x8XY7           VY - VX
        5 | 7 => {
            let (VX, VY) = if instruction & 0x000F == 5 {
                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, Y, X);
                }
                (VX, VY)
            } else {
                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, X, Y);
                }
                (VY, VX)
            };
            let result = VX as isize - VY as isize;
            if result < 0 {
                memory.write_register(0xF, 0);
            } else {
                memory.write_register(0xF, 1);
            }
            memory.write_register(X, (result % 255) as u8);
        }
        _ => unreachable!(),
    }
}

pub fn i8_6E(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);
    memory.write_register(0xF, VX & 0x1);

    if instruction & 0x000F == 6 {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            memory.write_register(X, VY >> 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted right by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            memory.write_register(X, VX >> 1);
        }
    } else if instruction & 0x000F == 0xE {
        if CB_8XY_ == CB::OLD {
            // VX is set to VY and shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it left by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
            }
            memory.write_register(X, VY << 1);
        } else if CB_8XY_ == CB::NEW {
            // VX is shifted left by 1. VF is set to the bit shifted out
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Shifting V{:01X} left by 1 with bit shifted out to VF",
                    pc - 2,
                    instruction,
                    X
                );
            }
            memory.write_register(X, VX << 1);
        }
    }
}

pub fn iA(memory: &mut Memory, pc: u16, instruction: u16) {
    let NNN = instruction & 0x0FFF;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting I to 0x{:03X}",
            pc - 2,
            instruction,
            NNN
        );
    }

    memory.write_adress(NNN);
}

pub fn iB(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let NNN = instruction & 0x0FFF;
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if CB_B_NN == CB::OLD {
        // 0xBNNN jump to 0x0NNN + V0
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0",
                *pc - 2,
                instruction,
                NNN
            );
        }

        let V0 = memory.read_register(0);

        *pc = NNN + V0 as u16;
    } else if CB_B_NN == CB::NEW {
        // 0xBXNN jump to 0xXNN + VX
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}",
                *pc - 2,
                instruction,
                NNN,
                X
            );
        }

        let VX = memory.read_register(X);

        *pc = NNN + VX as u16;
    }
}

pub fn iC(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let NN = (instruction & 0x00FF) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to random number and binary-AND's it with 0x{:02X}", pc-2, instruction, X, NN);
    }

    let mut rng = rand::thread_rng();
    let random: u8 = rng.gen();

    memory.write_register(X, random & NN as u8);
}

pub fn iD(
    memory: &mut Memory,
    pc: u16,
    instruction: u16,
    screen: &mut Screen,
    canvas: &mut WindowCanvas,
) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;
    let N = instruction & 0x000F;

    let VX = memory.read_register(X);
    let VY = memory.read_register(Y);

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Displaying sprite at (V{:01X}, V{:01X}) = ({VX}, {VY}) with width 8 and height {:01X}", pc-2, instruction, X, Y, N);
    }

    let mut cX = VX % 64; // coord X
    let mut cY = VY % 32; // coord Y
    let ccX = cX;
    memory.write_register(0xF, 0);

    'rows: for i in 0..N {
        let row = memory.read(memory.read_adress() + i);
        'columns: for j in 0..8 {
            let pixel = (row >> (7 - j)) & 0x1;
            if pixel == 1 {
                if screen.is_on(cX, cY) {
                    memory.write_register(0xF, 1);
                    screen.set(cX, cY, false);
                } else {
                    screen.set(cX, cY, true);
                }
            }

            let new_cX = cX as usize + 1;
            if new_cX == 64 {
                break 'columns;
            } else {
                cX = new_cX as u8;
            }
        }
        cX = ccX;
        let new_cY = cY as usize + 1;
        if new_cY == 32 {
            break 'rows;
        } else {
            cY = new_cY as u8;
        }
    }

    display::display(canvas, screen).expect("Error while displaying");
}

pub fn iE(instruction: u16, pc: &mut u16, memory: &mut Memory, keys_state: &KeysState) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let VX = memory.read_register(X);

    let is_key_pressed_VX = keys_state.read_state(VX);

    if instruction & 0x00FF == 0x009E {
        if is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", *pc-2, instruction, X, VX);
        } else if DEBUG {
            println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        }
    } else if instruction & 0x00FF == 0x00A1 {
        if !is_key_pressed_VX {
            *pc += 2;
        }
        if DEBUG && !is_key_pressed_VX {
            println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", *pc-2, instruction, X, VX);
        } else if DEBUG {
            println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", *pc-2, instruction, X, VX);
        }
    } else {
        panic!(
            "{}",
            NonUsedInstructionError {
                pc: *pc - 2,
                instruction
            }
        )
    }
}

pub fn iF_07(instruction: u16, pc: u16, memory: &mut Memory) {
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
    memory.write_register(X, timer_val);
}

pub fn iF_0A(instruction: u16, pc: &mut u16, memory: &mut Memory, keys_state: &KeysState) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Waiting for a key press, storing the value of the key in V{:01X}", *pc-2, instruction, X);
    }

    match keys_state.is_key_pressed() {
        Some(key_pressed) => memory.write_register(X, key_pressed),
        None => *pc -= 2,
    }
}

pub fn iF_1518(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Setting the sound timer to V{:01X}",
            *pc - 2,
            instruction,
            X
        );
    }

    let VX = memory.read_register(X);

    if instruction & 0x00FF == 0x0015 {
        memory.write_delay_timer(VX);
    } else {
        memory.write_sound_timer(VX);
    }
}

pub fn iF_1E(instruction: u16, pc: &mut u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let VX = memory.read_register(X);
    let new_I = memory.read_adress() as usize + VX as usize;
    if CB_FX1E == CB::NEW && new_I > 0xFFF {
        if DEBUG {
            println!(
                "0x{:03X} | 0x{:04X} | Adding V{:01X} to I with carry flag",
                *pc - 2,
                instruction,
                X
            );
        }
        memory.write_register(0xF, 1);
    } else if DEBUG {
        println!(
            "0x{:03X} | 0x{:04X} | Adding V{:01X} to I",
            *pc - 2,
            instruction,
            X
        );
    }
    memory.write_adress((new_I % 0x1000) as u16);
}

pub fn iF_29(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Setting I to the location of the sprite for the character in V{:01X}", pc-2, instruction, X);
    }

    let char_0x = memory.read_register(X) & 0x0F;
    memory.write_adress((char_0x as u16) * 5 + 50);
}

pub fn iF_33(instruction: u16, pc: u16, memory: &mut Memory) {
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

pub fn iF_5565(instruction: u16, pc: u16, memory: &mut Memory) {
    let X = ((instruction & 0x0F00) >> 8) as usize;

    let I = memory.read_adress();
    if DEBUG {
        let (action, particle) = if instruction & 0x00FF == 0x0055 {
            ("Storing", "to")
        } else {
            ("Loading", "from")
        };
        println!("0x{:03X} | 0x{:04X} | {action} V0 through V{:01X} {particle} memory starting at address I", pc-2, instruction, X);
    }
    for i in (0..NB_REGISTERS).take(X + 1) {
        let iu16 = i as u16;
        if instruction & 0x00FF == 0x0055 {
            let Vi = memory.read_register(i);
            if DEBUG_VERBOSE {
                println!("               | Storing V{:01X} = 0x{:02X} ({Vi}) in memory at address {:03X}", i, Vi, I+i as u16);
            }
            memory.write(I + iu16, Vi);
        } else {
            /* instruction & 0x00FF == 0x0065 */
            let future_Vi = memory.read(I + iu16);
            if DEBUG_VERBOSE {
                println!("               | Storing memory at address {:03X} = 0x{:02X} ({future_Vi}) in V{:01X}", I+i as u16, future_Vi, i);
            }
            memory.write_register(i, future_Vi);
        }
    }
    if CB_FX_5 == CB::OLD {
        memory.write_adress(I + (X as u16) + 1);
    }
}

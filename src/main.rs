#![allow(non_snake_case)]
mod custom_errors;
mod display;
mod events;
mod instructions;
mod launch_options;
mod memory;
mod screen;

use instructions::{i8::*, *};
use launch_options::*;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::custom_errors::NonUsedInstructionError;

fn load_font(memory: &mut memory::Memory) {
    for (i, byte) in FONT_SET.iter().enumerate() {
        memory.write(i as u16 + FONT_ADRESS, *byte);
    }
}

fn main() {
    // let args = env::args().collect::<Vec<String>>();                        // TODO
    // if !args.is_empty() {
    //     let debug_str = args[1].clone();
    //     let DEBUG = if debug_str == "true" { true } else { false };
    // }

    // INIT DISPLAY
    let mut screen = screen::Screen::new();

    let (sdl_context, mut canvas) = display::init().expect("Could not init display");

    // INIT MEMORY
    let mut memory: memory::Memory = memory::Memory::new();
    memory.load_rom(ROM_PATH).unwrap();
    load_font(&mut memory);

    let I_adr: u16 = 0xFFE; // Index register
    let mut V_adr: [u16; 16] = [0; 16]; // Registers V0 to VF
    for (i, V_adr_i) in V_adr.iter_mut().enumerate() {
        *V_adr_i = 0xFEE + i as u16;
    }

    let timer_adr = 0x0FED; // Timer register
    let sound_adr = 0x0FEC;
    memory.write(timer_adr, 0x00);
    memory.write(sound_adr, 0x00); // Sound register

    let mut stack = Vec::<u16>::new(); // Stack of adresses used to call subroutines or return from them
    let mut pc: u16 = 0x200; // program counter

    let mutex_memory = Arc::new(Mutex::new(memory));
    let mutex_memory_timer = mutex_memory.clone();
    let mutex_memory_sound = mutex_memory.clone();

    thread::spawn(move || loop {
        let mut guard = mutex_memory_timer.lock().unwrap();
        let timer = guard.read(timer_adr);
        if timer > 0 {
            guard.write(timer_adr, timer - 1);
            std::mem::drop(guard);
            thread::sleep(Duration::from_millis(16));
        } else {
            std::mem::drop(guard);
        }
    });
    thread::spawn(move || {
        loop {
            let mut guard = mutex_memory_sound.lock().unwrap();
            let timer = guard.read(sound_adr);
            if timer > 0 {
                // add beep
                guard.write(sound_adr, timer - 1);
                std::mem::drop(guard);
                thread::sleep(Duration::from_millis(16));
            } else {
                std::mem::drop(guard);
            }
        }
    });

    // GAME LOOP
    if DEBUG {
        println!();
        println!(" adr  | instr  | effect");
        println!("------+--------+--------------------------------");
    }

    'game: loop {
        let start = Instant::now();

        let guard = mutex_memory.lock().unwrap();
        let instruction = guard.read_word(pc);
        std::mem::drop(guard);

        let X = ((instruction & 0x0F00) >> 8) as usize;

        pc += 2;
        let opcode = (instruction & 0xF000) >> 12;

        match opcode {
            // 0x00E0 : Clear screen
            // 0x00EE : Return from subroutine
            0 => {
                if let Err(e) = i0::r(instruction, &mut pc, &mut stack, &mut screen, &mut canvas) {
                    panic!("{e}");
                }
            }
            // 0x1NNN jump to adress 0xNNN
            1 => i1::r(instruction, &mut pc),
            // 0x2NNN call subroutine at 0xNNN
            2 => i2::r(instruction, &mut pc, &mut stack),
            // 0x3XNN skip next instruction if VX == NN
            // 0x4XNN skip next instruction if VX != NN
            3 | 4 => i34::r(instruction, &mut pc, &mutex_memory, opcode, &V_adr),
            // 0x5XY0 skip next instruction if VX == VY
            // 0x9XY0 skip next instruction if VX != VY
            5 | 9 => {
                if let Err(e) = i59::r(instruction, &mut pc, opcode, &mutex_memory, V_adr) {
                    panic!("{e}");
                }
            }
            // 0x6XNN set register VX to 0xNN
            6 => i6::r(instruction, pc, &mutex_memory, &V_adr),
            // 0x7XNN add 0xNN to register VX (carry flag is not changed)
            7 => i7::r(instruction, pc, &mutex_memory, &V_adr),
            0x8 => {
                match instruction & 0x000F {
                    // 0x8XY0 set VX to VY
                    0 => i8_0::r(&mutex_memory, pc, &instruction, &V_adr),
                    // 0x8XY1 set VX to VX | VY
                    // 0x8XY2 set VX to VX & VY
                    // 0x8XY3 set VX to VX ^ VY
                    1 | 2 | 3 => i8_123::r(instruction, pc, &mutex_memory, &V_adr),
                    // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                    // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
                    // 0x8XY7           VY - VX
                    4 | 5 | 7 => i8_457::r(instruction, pc, &mutex_memory, &V_adr),
                    // 0x8XY6 OLD : VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                    //        NEW : VX is shifted right by 1. VF is set to the bit shifted out
                    // 0x8XYE OLD : VX is set to VY and shifted left by 1. VF is set to the bit shifted out
                    //        NEW : VX is shifted left by 1. VF is set to the bit shifted out
                    6 | 0xE => i8_6E::r(instruction, pc, &mutex_memory, &V_adr),
                    _ => panic!(
                        "{}",
                        NonUsedInstructionError {
                            pc: pc - 2,
                            instruction
                        }
                    ),
                }
            }
            // 0xANNN set I to 0x0NNN
            0xA => iA::r(&mutex_memory, pc, instruction, I_adr),
            // 0xBNNN OLD: jump to 0x0NNN + V0
            // 0xBXNN NEW: jump to 0xXNN + VX
            0xB => iB::r(instruction, &mut pc, &mutex_memory, &V_adr),
            // 0xCXNN set VX to random number and binary-AND's it with NN
            0xC => iC::r(instruction, pc, &mutex_memory, &V_adr),
            // 0xDXYN display sprite at (VX, VY) with width 8 and height N
            0xD => iD::r(
                &mutex_memory,
                pc,
                instruction,
                I_adr,
                &V_adr,
                &mut screen,
                &mut canvas,
            ),
            // 0xEX9E skip next instruction if key with the value of VX is pressed
            // 0xEXA1 skip next instruction if key with the value of VX is not pressed
            0xE => {
                // Events
                let event_key = events::events(&sdl_context);
                let key_pressed: u8 = match event_key {
                    Ok(key) => key as u8,
                    Err(_) => break 'game,
                };
                sdl_context
                    .event()
                    .expect("Error getting event")
                    .flush_event(sdl2::event::EventType::KeyDown);

                let guard = mutex_memory.lock().unwrap();
                let VX = guard.read(V_adr[X]);
                std::mem::drop(guard);

                if instruction & 0x00FF == 0x009E {
                    if key_pressed == VX {
                        pc += 2;
                    }
                    if DEBUG && key_pressed == VX {
                        println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", pc-2, instruction, X, VX);
                    } else {
                        println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", pc-2, instruction, X, VX);
                    }
                } else if instruction & 0x00FF == 0x00A1 {
                    if key_pressed != VX {
                        pc += 2;
                    }
                    if DEBUG && key_pressed != VX {
                        println!("0x{:03X} | 0x{:04X} | Skipping next instruction because the key with the value of V{:01X} ({:02X}) is not pressed", pc-2, instruction, X, VX);
                    } else {
                        println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because the key with the value of V{:01X} ({:02X}) is pressed", pc-2, instruction, X, VX);
                    }
                } else {
                    panic!(
                        "{}",
                        NonUsedInstructionError {
                            pc: pc - 2,
                            instruction
                        }
                    )
                }
            }
            0xF => {
                match instruction & 0x00FF {
                    0x0007 => {
                        // 0xFX07 set VX to the value of the delay timer
                        if DEBUG {
                            println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to the value of the delay timer", pc-2, instruction, X);
                        }

                        let mut guard = mutex_memory.lock().unwrap();
                        let timer_val = guard.read(timer_adr);
                        guard.write(V_adr[X], timer_val);
                        std::mem::drop(guard);
                    }
                    0x000A => {
                        // 0xFX0A wait for a key press, store the value of the key in VX

                        let event_key = events::events(&sdl_context);
                        let key_pressed: u8 = match event_key {
                            Ok(key) => key as u8,
                            Err(_) => break 'game,
                        };
                        sdl_context
                            .event()
                            .expect("Error getting event")
                            .flush_event(sdl2::event::EventType::KeyDown);

                        if DEBUG {
                            println!("0x{:03X} | 0x{:04X} | Waiting for a key press, storing the value of the key in V{:01X}", pc-2, instruction, X);
                        }

                        let mut guard = mutex_memory.lock().unwrap();
                        if key_pressed != 0xFF {
                            guard.write(V_adr[X], key_pressed);
                        } else {
                            pc -= 2;
                        }
                        std::mem::drop(guard);
                    }
                    0x0015 | 0x0018 => {
                        // 0xFX15 set the delay timer to VX
                        // 0xFX18 set the sound timer to VX
                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X]);
                        let which_timer = if instruction & 0x00FF == 0x0015 {
                            if DEBUG {
                                println!(
                                    "0x{:03X} | 0x{:04X} | Setting the delay timer to V{:01X}",
                                    pc - 2,
                                    instruction,
                                    X
                                );
                            }
                            timer_adr
                        } else {
                            /* instruction & 0x00FF == 0x0018 */
                            if DEBUG {
                                println!(
                                    "0x{:03X} | 0x{:04X} | Setting the sound timer to V{:01X}",
                                    pc - 2,
                                    instruction,
                                    X
                                );
                            }
                            sound_adr
                        };
                        guard.write(which_timer, VX);
                        std::mem::drop(guard);
                    }
                    0x001E => {
                        // 0xFX1E add VX to I with carry flag if CB_BNNN = NEW
                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X]);
                        let new_I = guard.read_word(I_adr) as usize + VX as usize;
                        if CB_FX1E == CB::NEW && new_I > 0xFFF {
                            if DEBUG {
                                println!(
                                    "0x{:03X} | 0x{:04X} | Adding V{:01X} to I with carry flag",
                                    pc - 2,
                                    instruction,
                                    X
                                );
                            }
                            guard.write(V_adr[0xF], 1);
                        } else if DEBUG {
                            println!(
                                "0x{:03X} | 0x{:04X} | Adding V{:01X} to I",
                                pc - 2,
                                instruction,
                                X
                            );
                        }
                        guard.write_word(I_adr, (new_I % 0x1000) as u16);
                        std::mem::drop(guard);
                    }
                    0x0029 => {
                        // 0xFX29 set I to the location of the sprite for the character in VX
                        if DEBUG {
                            println!("0x{:03X} | 0x{:04X} | Setting I to the location of the sprite for the character in V{:01X}", pc-2, instruction, X);
                        }

                        let mut guard = mutex_memory.lock().unwrap();
                        let char_0x = guard.read(V_adr[X]) & 0x0F;
                        guard.write_word(I_adr, (char_0x as u16) * 5 + 50);
                        std::mem::drop(guard);
                    }
                    0x0033 => {
                        // 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
                        if DEBUG {
                            println!("0x{:03X} | 0x{:04X} | Storing the binary-coded decimal representation of V{:01X} at the addresses I, I+1, and I+2", pc-2, instruction, X);
                        }

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X]);
                        let (digit_1, digit_2, digit_3) = (VX / 100, (VX / 10) % 10, VX % 10);
                        let I = guard.read_word(I_adr);
                        guard.write(I, digit_1);
                        guard.write(I + 1, digit_2);
                        guard.write(I + 2, digit_3);
                        std::mem::drop(guard);
                    }
                    0x0055 | 0x0065 => {
                        // 0xFX55 store V0 through VX in memory starting at address I
                        // 0xFX65 store memory through V0 to VX starting at address I
                        let mut guard = mutex_memory.lock().unwrap();
                        let I = guard.read_word(I_adr);
                        if DEBUG {
                            let (action, particle) = if instruction & 0x00FF == 0x0055 {
                                ("Storing", "to")
                            } else {
                                ("Loading", "from")
                            };
                            println!("0x{:03X} | 0x{:04X} | {action} V0 through V{:01X} {particle} memory starting at address I", pc-2, instruction, X);
                        }
                        for (i, V_adr_i) in V_adr.iter().enumerate().take(X + 1) {
                            let iu16 = i as u16;
                            if instruction & 0x00FF == 0x0055 {
                                let Vi = guard.read(*V_adr_i);
                                if DEBUG_VERBOSE {
                                    println!("               | Storing V{:01X} = 0x{:02X} ({Vi}) in memory at address {:03X}", i, Vi, I+i as u16);
                                }
                                guard.write(I + iu16, Vi);
                            } else {
                                /* instruction & 0x00FF == 0x0065 */
                                let future_Vi = guard.read(I + iu16);
                                if DEBUG_VERBOSE {
                                    println!("               | Storing memory at address {:03X} = 0x{:02X} ({future_Vi}) in V{:01X}", I+i as u16, future_Vi, i);
                                }
                                guard.write(*V_adr_i, future_Vi);
                            }
                        }
                        if CB_FX_5 == CB::OLD {
                            guard.write_word(I_adr, I + (X as u16) + 1);
                        }
                    }
                    _ => {
                        panic!(
                            "{}",
                            NonUsedInstructionError {
                                pc: pc - 2,
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

        // To have IPS instructions per second
        if let Some(time_elapsed) = Duration::from_millis(1000 / IPS).checked_sub(start.elapsed()) {
            thread::sleep(time_elapsed);
        }
    }
}

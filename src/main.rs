#![allow(non_snake_case)]
mod custom_errors;
mod display;
mod events;
mod launch_options;
mod memory;
mod screen;
mod instructions {
    pub mod i0;
    pub mod i1;
    pub mod i2;
    pub mod i34;
    pub mod i59;
    pub mod i6;
    pub mod i7;
    pub mod i8 {
        pub mod i8_0;
        pub mod i8_123;
        pub mod i8_457;
        pub mod i8_6E;
        pub mod i8_A;
    }
}

use instructions::{
    i0::*,
    i1::*,
    i2::*,
    i34::*,
    i59::*,
    i6::*,
    i7::*,
    i8::{i8_0::*, i8_123::*, i8_457::*, i8_6E::*, i8_A::*},
};
use launch_options::*;
use memory::Memory;
use screen::Screen;

use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

use crate::custom_errors::NonUsedInstructionError;

fn load_font(memory: &mut Memory) {
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
    let mut screen = Screen::new();

    let (sdl_context, mut canvas) = display::init().expect("Could not init display");

    // INIT MEMORY
    let mut memory: Memory = Memory::new();
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

        // Events
        let event_key = events::events(&sdl_context);
        let key_pressed: u8 = match event_key {
            Ok(key) => key as u8,
            Err(_) => break 'game,
        };

        let guard = mutex_memory.lock().unwrap();
        let instruction = guard.read_word(pc);
        std::mem::drop(guard);

        let X = ((instruction & 0x0F00) >> 8) as usize;
        let Y = ((instruction & 0x00F0) >> 4) as usize;
        let N = instruction & 0x000F;
        let NN = (instruction & 0x00FF) as usize;
        let NNN = instruction & 0x0FFF;

        pc += 2;
        let opcode = (instruction & 0xF000) >> 12;

        match opcode {
            // 0x00E0 : Clear screen
            // 0x00EE : Return from subroutine
            0 => {
                if let Err(e) = i0(instruction, &mut pc, &mut stack, &mut screen, &mut canvas) {
                    panic!("{e}");
                }
            }
            // 0x1NNN jump to adress 0xNNN
            1 => i1(instruction, &mut pc, NNN),
            // 0x2NNN call subroutine at 0xNNN
            2 => i2(instruction, &mut pc, &mut stack, NNN),
            // 0x3XNN skip next instruction if VX == NN
            // 0x4XNN skip next instruction if VX != NN
            3 | 4 => i34(instruction, &mut pc, &mutex_memory, X, NN, opcode, &V_adr),
            // 0x5XY0 skip next instruction if VX == VY
            // 0x9XY0 skip next instruction if VX != VY
            5 | 9 => {
                if let Err(e) = i59(instruction, &mut pc, X, Y, opcode, &mutex_memory, V_adr) {
                    panic!("{e}");
                }
            }
            // 0x6XNN set register VX to 0xNN
            6 => i6(instruction, pc, &mutex_memory, X, NN, &V_adr),
            // 0x7XNN add 0xNN to register VX (carry flag is not changed)
            7 => i7(instruction, pc, &mutex_memory, X, NN, &V_adr),
            0x8 => {
                match instruction & 0x000F {
                    // 0x8XY0 set VX to VY
                    0 => i8_0(&mutex_memory, pc, &instruction, X, Y, &V_adr),
                    // 0x8XY1 set VX to VX | VY
                    // 0x8XY2 set VX to VX & VY
                    // 0x8XY3 set VX to VX ^ VY
                    1 | 2 | 3 => i8_123(instruction, pc, &mutex_memory, &V_adr, X, Y),
                    // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                    // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
                    // 0x8XY7           VY - VX
                    4 | 5 | 7 => i8_457(instruction, pc, &mutex_memory, &V_adr, X, Y),
                    // 0x8XY6 OLD : VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                    //        NEW : VX is shifted right by 1. VF is set to the bit shifted out
                    // 0x8XYE OLD : VX is set to VY and shifted left by 1. VF is set to the bit shifted out
                    //        NEW : VX is shifted left by 1. VF is set to the bit shifted out
                    6 | 0xE => i8_6E(instruction, pc, &mutex_memory, &V_adr, X, Y),
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
            0xA => i8_A(&mutex_memory, pc, instruction, NNN, I_adr),
            0xB => {
                if CB_B_NN == CB::OLD {
                    // 0xBNNN jump to 0x0NNN + V0
                    if DEBUG {
                        println!(
                            "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0",
                            pc - 2,
                            instruction,
                            NNN
                        );
                    }

                    let guard = mutex_memory.lock().unwrap();
                    let V0 = guard.read(V_adr[0]);
                    std::mem::drop(guard);

                    pc = NNN + V0 as u16;
                } else if CB_B_NN == CB::NEW {
                    // 0xBXNN jump to 0xXNN + VX
                    if DEBUG {
                        println!(
                            "0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}",
                            pc - 2,
                            instruction,
                            NNN,
                            X
                        );
                    }

                    let guard = mutex_memory.lock().unwrap();
                    let VX = guard.read(V_adr[X]);
                    std::mem::drop(guard);

                    pc = NNN + VX as u16;
                } else {
                }
            }
            0xC => {
                // 0xCXNN set VX to random number and binary-AND's it with NN
                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to random number and binary-AND's it with 0x{:02X}", pc-2, instruction, X, NN);
                }

                let mut rng = rand::thread_rng();
                let random: u8 = rng.gen();

                let mut guard = mutex_memory.lock().unwrap();
                guard.write(V_adr[X], random & NN as u8);
                std::mem::drop(guard);
            }
            0xD => {
                // 0xDXYN display sprite at (VX, VY) with width 8 and height N
                let mut guard = mutex_memory.lock().unwrap();
                let VX = guard.read(V_adr[X]);
                let VY = guard.read(V_adr[Y]);

                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Displaying sprite at (V{:01X}, V{:01X}) = ({VX}, {VY}) with width 8 and height {:01X}", pc-2, instruction, X, Y, N);
                }

                let mut cX = VX % 64; // coord X
                let mut cY = VY % 32; // coord Y
                let ccX = cX;
                guard.write(V_adr[0xF], 0);

                'rows: for i in 0..N {
                    let row = guard.read(guard.read_word(I_adr) + i);
                    'columns: for j in 0..8 {
                        let pixel = (row >> (7 - j)) & 0x1;
                        if pixel == 1 {
                            if screen.is_on(cX, cY) {
                                guard.write(V_adr[0xF], 1);
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
                std::mem::drop(guard);

                if TERMINAL {
                    screen.debug_display();
                } else {
                    display::display(&mut canvas, &screen).expect("Error while displaying");
                }
            }
            0xE => {
                // 0xEX9E skip next instruction if key with the value of VX is pressed
                // 0xEXA1 skip next instruction if key with the value of VX is not pressed
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

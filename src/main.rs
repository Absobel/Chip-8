#![allow(non_snake_case)]
mod memory;
mod stack;
mod display;
mod screen;
mod launch_options;

use memory::Memory;
use stack::Stack;
use screen::Screen;
use launch_options::*;

use std::{
    time::{Duration, Instant},
    thread,
    sync::{Arc, Mutex},
};
use rand::Rng;

fn load_font(memory: &mut Memory) {
    for (i,byte) in FONT_SET.iter().enumerate() {
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
    let texture_creator = canvas.texture_creator();
    let (texture_off, texture_on) = display::textures_init(&texture_creator).expect("Failed to create pixel textures");


    // INIT MEMORY
    let mut memory: Memory = Memory::new();
    memory.load_rom(ROM_PATH).unwrap();
    load_font(&mut memory);

    let I_adr: u16 = 0xFFE;                           // Index register
    let mut V_adr: [u16; 16] = [0; 16];                // Registers V0 to VF 
    for i in 0..16 {
        V_adr[i] = 0xFEE + i as u16;
    }
    
    let timer_adr = 0x0FED;                       // Timer register
    let sound_adr = 0x0FEC;
    memory.write(timer_adr, 0x00);
    memory.write(sound_adr, 0x00);                       // Sound register

    let mut stack = Stack::<u16>::new();     // Stack of adresses used to call subroutines or return from them
    let mut pc: u16 = 0x200;             // program counter


    let mutex_memory = Arc::new(Mutex::new(memory));
    let mutex_memory_timer = mutex_memory.clone();
    let mutex_memory_sound = mutex_memory.clone();

    thread::spawn(move || {
        loop {
            let mut guard = mutex_memory_timer.lock().unwrap();
            let timer = guard.read(timer_adr);
            if timer > 0 {
                guard.write(timer_adr, timer-1);
                std::mem::drop(guard);
                thread::sleep(Duration::from_millis(16));
            }
        }
    });
    thread::spawn(move || {
        loop {
            let mut guard = mutex_memory_sound.lock().unwrap();
            let timer = guard.read(sound_adr);
            if timer > 0 {
                // add beep
                guard.write(sound_adr, timer-1);
                std::mem::drop(guard);
                thread::sleep(Duration::from_millis(16));
            }
        }
    });


    // GAME LOOP
    if DEBUG {
        println!("");
        println!(" adr  | instr  | effect");
        println!("------+--------+--------------------------------");
    }
    
    'game: loop { 
        let start = Instant::now();

        // Events
        let event_key = display::events(&sdl_context);
        let key_pressed: u8;
        match event_key {
            Err(e) => {
                println!("{e}");
                break 'game;
            },
            Ok(key) => {
                key_pressed = key as u8;
            }
        }

     
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
            0 => {
                match instruction {
                    // Clear screen
                    0x00E0 => {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Screen clearing", pc-2, instruction);}
                        screen.clear();
                        display::clear_screen(&mut canvas, &texture_off).expect("Failed to clear screen");
                    }
                    // Return from subroutine
                    0x00EE => {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Returning from subroutine", pc-2, instruction);}
                        pc = stack.pop().unwrap();
                    }
                    _ => {
                        println!("Non used instruction: 0x{:03X} | 0x{:04X}", pc-2, instruction);
                        println!("Either this is an error, or the program is trying to use a machine language subroutine.");
                        break;
                    }
                }
            }
            1 => {
                // 0x1NNN jump to adress 0xNNN
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Jumping to adress 0x{:03X}", pc-2, instruction, NNN);}
                pc = NNN;
            }
            2 => {
                // 0x2NNN call subroutine at 0xNNN
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Calling subroutine at 0x{:03X}", pc-2, instruction, NNN);}
                stack.push(pc);
                pc = NNN;
                
            }
            3 | 4 => {
                // 0x3XNN skip next instruction if VX == NN
                // 0x4XNN skip next instruction if VX != NN
                let guard = mutex_memory.lock().unwrap();
                let VX = guard.read(V_adr[X as usize]);
                std::mem::drop(guard);

                if opcode == 3 {
                    if VX == NN as u8 {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == 0x{:02X}", pc-2, instruction, X, NN);}
                        pc += 2;
                    } else {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != 0x{:02X}", pc-2, instruction, X, NN);}
                    }
                } else if opcode == 4 {
                    if VX != NN as u8 {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != 0x{:02X}", pc-2, instruction, X, NN);}
                        pc += 2;
                    } else {
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == 0x{:02X}", pc-2, instruction, X, NN);}
                    }
                }
            }
            5 | 9 => {
                if instruction & 0x000F == 0 {
                    // 0x5XY0 skip next instruction if VX == VY
                    let guard = mutex_memory.lock().unwrap();
                    let VX = guard.read(V_adr[X as usize]);
                    let VY = guard.read(V_adr[Y as usize]);
                    std::mem::drop(guard);

                    if opcode == 5 {
                        if VX == VY {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == V{:X}", pc-2, instruction, X, Y);}
                            pc += 2;
                        } else {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != V{:X}", pc-2, instruction, X, Y);}
                        }
                    } else if opcode == 9 {
                        if VX != VY {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != V{:X}", pc-2, instruction, X, Y);}
                            pc += 2;
                        } else {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == V{:X}", pc-2, instruction, X, Y);}
                        }
                    }
                }
                else {
                    println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                    break;
                }
            }
            6 => {
                // 0x6XNN set register VX to 0xNN
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to 0x{:02X}", pc-2, instruction, X, NN);}

                let mut guard = mutex_memory.lock().expect("Failed to lock memory");
                guard.write(V_adr[X as usize], NN as u8);
                std::mem::drop(guard);
            }
            7 => {
                // 0x7XNN add 0xNN to register VX (carry flag is not changed)
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Adding 0x{:02X} to register V{:01X}", pc-2, instruction, NN, X);}

                let mut guard = mutex_memory.lock().expect("Failed to lock memory");
                let VX = guard.read(V_adr[X]) as usize;
                guard.write(V_adr[X as usize], (VX + NN) as u8);
                std::mem::drop(guard);
            }
            0x8 => {
                match instruction & 0x000F {
                    0 => {
                        // 0x8XY0 set VX to VY
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X}", pc-2, instruction, X, Y);}

                        let mut guard = mutex_memory.lock().unwrap();
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[X as usize], VY);
                        std::mem::drop(guard);
                    }
                    1 | 2 | 3 => {
                        // 0x8XY1 set VX to VX | VY
                        // 0x8XY2 set VX to VX & VY
                        // 0x8XY3 set VX to VX ^ VY
                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        match instruction & 0x000F {
                            1 => {
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} | V{:01X}", pc-2, instruction, X, X, Y);
                                guard.write(V_adr[X as usize], VX | VY);}
                            }
                            2 => {
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} & V{:01X}", pc-2, instruction, X, X, Y);
                                guard.write(V_adr[X as usize], VX & VY);}
                            }
                            3 => {
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} ^ V{:01X}", pc-2, instruction, X, X, Y);
                                guard.write(V_adr[X as usize], VX ^ VY);}
                            }
                            _ => unreachable!(),
                        } 
                        std::mem::drop(guard);
                    }
                    4 | 5 | 7 => {
                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        match instruction & 0x000F {
                            // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                            4 => {
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Adding V{:01X} to V{:01X} with carry flag to VF", pc-2, instruction, Y, X);}
                                let (result, carry) = VX.overflowing_add(VY);
                                guard.write(V_adr[X as usize], result);
                                guard.write(V_adr[0xF], carry as u8);
                            }
                            // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
                            // 0x8XY7           VY - VX
                            5 | 7 => {
                                let (VX,VY) = if instruction & 0x000F == 5 {
                                    if DEBUG {println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, Y, X);}
                                    (VX,VY)
                                } else {
                                    if DEBUG {println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, X, Y);}
                                    (VY,VX)
                                };
                                let result = VX as isize - VY as isize;
                                if result < 0 {
                                    guard.write(V_adr[0xF], 0);
                                } else {
                                    guard.write(V_adr[0xF], 1);
                                }
                                guard.write(V_adr[X as usize], (result % 255) as u8);
                            }
                            _ => unreachable!(),
                        }
                        std::mem::drop(guard);
                    }
                    6 | 0xE => {
                        // 0x8XY6
                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[0xF], VX & 0x1);

                        if instruction & 0x000F == 6 {
                            if CB_8XY_ == CB::OLD {
                                // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);}
                                guard.write(V_adr[X as usize], VY >> 1);
                            } else if CB_8XY_ == CB::NEW {
                                // VX is shifted right by 1. VF is set to the bit shifted out
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF", pc-2, instruction, X);}
                                guard.write(V_adr[X as usize], VX >> 1);
                            }
                        } else if instruction & 0x000F == 0xE {
                            if CB_8XY_ == CB::OLD {
                                // VX is set to VY and shifted left by 1. VF is set to the bit shifted out
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it left by 1 with bit shifted out to VF", pc-2, instruction, X, Y);}
                                guard.write(V_adr[X as usize], VY << 1);
                            } else if CB_8XY_ == CB::NEW {
                                // VX is shifted left by 1. VF is set to the bit shifted out
                                if DEBUG {println!("0x{:03X} | 0x{:04X} | Shifting V{:01X} left by 1 with bit shifted out to VF", pc-2, instruction, X);}
                                guard.write(V_adr[X as usize], VX << 1);
                            }
                        }
                        std::mem::drop(guard);
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            0xA => {
                // 0xANNN set I to 0x0NNN
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting I to 0x{:03X}", pc-2, instruction, NNN);}

                let mut guard = mutex_memory.lock().unwrap();
                guard.write_word(I_adr, NNN);
                std::mem::drop(guard);
            }
            0xB => {
                if CB_B_NN == CB::OLD {
                    // 0xBNNN jump to 0x0NNN + V0
                    if DEBUG {println!("0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0", pc-2, instruction, NNN);}

                    let guard = mutex_memory.lock().unwrap();
                    let V0 = guard.read(V_adr[0]);
                    std::mem::drop(guard);

                    pc = NNN + V0 as u16;
                } else if CB_B_NN == CB::NEW {
                    // 0xBXNN jump to 0xXNN + VX
                    if DEBUG {println!("0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}", pc-2, instruction, NNN, X);}
    
                    let guard = mutex_memory.lock().unwrap();
                    let VX = guard.read(V_adr[X as usize]);
                    std::mem::drop(guard);
    
                    pc = NNN + VX as u16;
                } else {}
            }
            0xC => {
                // 0xCXNN set VX to random number and binary-AND's it with NN
                if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to random number and binary-AND's it with 0x{:02X}", pc-2, instruction, X, NN);}

                let mut rng = rand::thread_rng();
                let random : u8 = rng.gen();

                let mut guard = mutex_memory.lock().unwrap();
                guard.write(V_adr[X as usize], random & NN as u8);
                std::mem::drop(guard);
            }
            0xD => {
                    // 0xDXYN display sprite at (VX, VY) with width 8 and height N
                    if DEBUG {println!("0x{:03X} | 0x{:04X} | Displaying sprite at (V{:01X}, V{:01X}) with width 8 and height {:01X}", pc-2, instruction, X, Y, N);}
                    
                    let mut guard = mutex_memory.lock().unwrap();
                    let mut cX = guard.read(V_adr[X]) % 64;    // coord X
                    let mut cY = guard.read(V_adr[Y]) % 32;    // coord Y
                    let ccX = cX.clone();
                    guard.write(V_adr[0xF], 0);
                    let mut modified: Vec<(u8,u8)> = Vec::new();

                    'rows: for i in 0..N {
                        let row = guard.read(guard.read_word(I_adr) + i as u16);
                        'columns: for j in 0..8 {
                            let pixel = (row >> (7-j)) & 0x1;
                            if pixel == 1 {
                                if screen.is_on(cX, cY) {
                                    guard.write(V_adr[0xF], 1);
                                    screen.set(cX, cY, false);
                                } else {
                                    screen.set(cX, cY, true);
                                }
                                modified.push((cX, cY));
                            }

                            let new_cX = cX as usize + 1;
                            if new_cX == 64 {
                                break 'columns;
                            } else {
                                cX = new_cX as u8;
                            }
                        }
                        cX = ccX.clone();
                        let new_cY = cY as usize + 1;
                        if new_cY == 32 {
                            break 'rows;
                        } else {
                            cY = new_cY as u8;
                        }
                    }
                    std::mem::drop(guard);
                    
                    if TERMINAL {screen.debug_display();}
                    else {display::display(&mut canvas, (&texture_off, &texture_on), &screen, modified)
                        .expect("Error while displaying");}
            }
            0xE => {
                match instruction & 0x00FF {
                    0x009E | 0x00A1 => {
                        // 0xEX9E skip next instruction if key with the value of VX is pressed
                        // 0xEXA1 skip next instruction if key with the value of VX is not pressed
                        let guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X]);
                        std::mem::drop(guard);

                        if instruction & 0x00FF == 0x009E {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction if key with the value of V{:01X} ({:02X}) is pressed", pc-2, instruction, X, VX);}

                            if key_pressed == VX {
                                pc += 2;
                            }
                        } else if instruction & 0x00FF == 0x00A1 {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Skipping next instruction if key with the value of V{:01X} ({:02X}) is not pressed", pc-2, instruction, X, VX);}

                            if key_pressed != VX {
                                pc += 2;
                            }
                        }
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            0xF => {
                match instruction & 0x00FF {
                    0x0007 => {
                        // 0xFX07 set VX to the value of the delay timer
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to the value of the delay timer", pc-2, instruction, X);}

                        let mut guard = mutex_memory.lock().unwrap();
                        let timer_val = guard.read(timer_adr);
                        guard.write(V_adr[X], timer_val);
                        std::mem::drop(guard);
                    }
                    0x000A => {
                        // 0xFX0A wait for a key press, store the value of the key in VX
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Waiting for a key press, storing the value of the key in V{:01X}", pc-2, instruction, X);}

                        let mut guard = mutex_memory.lock().unwrap();
                        if key_pressed != 0xFF {
                            guard.write(V_adr[X], key_pressed as u8);
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
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting the delay timer to V{:01X}", pc-2, instruction, X);}
                            timer_adr
                        } else /* instruction & 0x00FF == 0x0018 */ {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting the sound timer to V{:01X}", pc-2, instruction, X);}
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
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Adding V{:01X} to I with carry flag", pc-2, instruction, X);}
                            guard.write(V_adr[0xF], 1);
                        } else {
                            if DEBUG {println!("0x{:03X} | 0x{:04X} | Adding V{:01X} to I", pc-2, instruction, X);}
                        } 
                        guard.write_word(I_adr, (new_I % 0x1000) as u16);
                        std::mem::drop(guard);
                    }
                    0x0029 => {
                        // 0xFX29 set I to the location of the sprite for the character in VX
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Setting I to the location of the sprite for the character in V{:01X}", pc-2, instruction, X);}

                        let mut guard = mutex_memory.lock().unwrap();
                        let char_0x = guard.read(V_adr[X]) & 0x0F;
                        guard.write_word(I_adr, (char_0x as u16)*5 + 50);
                        std::mem::drop(guard);
                    }
                    0x0033 => {
                        // 0xFX33 store the binary-coded decimal representation of VX at the addresses I, I+1, and I+2
                        if DEBUG {println!("0x{:03X} | 0x{:04X} | Storing the binary-coded decimal representation of V{:01X} at the addresses I, I+1, and I+2", pc-2, instruction, X);}

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X]);
                        let (digit_1,digit_2,digit_3) = (VX / 100, (VX / 10) % 10, VX % 10);   
                        let I = guard.read_word(I_adr);
                        guard.write(I, digit_1);
                        guard.write(I + 1, digit_2);
                        guard.write(I + 2, digit_3);
                        std::mem::drop(guard);
                    }
                    0x0055 | 0x0065 => {
                        // 0xFX55 store V0 to VX in memory starting at address I
                        // 0xFX65 store memory to V0 to VX starting at address I
                        let mut guard = mutex_memory.lock().unwrap();
                        let I = guard.read_word(I_adr);
                        if DEBUG {println!("0x{:03X} | 0x{:04X} |", pc-2, instruction);}
                        for i in 0..X+1 {
                            let iu16 = i as u16;
                            if instruction & 0x00FF == 0x0055 {
                                if DEBUG {println!("      | Storing V{:01X} in memory at address {:03X}+{:01X}", i, I, i);}
                                let Vi = guard.read(V_adr[i]);
                                guard.write(I+iu16, Vi);
                            } else /* instruction & 0x00FF == 0x0065 */ {
                                if DEBUG {println!("      | Storing memory at address {:03X}+{:01X} in V{:01X}", I, i, i);}
                                let future_Vi = guard.read(I+iu16);
                                guard.write(V_adr[i], future_Vi);
                            }
                        }
                        if CB_FX_5 == CB::OLD {
                            guard.write_word(I_adr, I+(X as u16)+1);
                        }
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }

        // To have IPS instructions per second
        if start.elapsed() < Duration::from_millis(1000 / IPS) {
            thread::sleep(Duration::from_millis(1000 / IPS) - start.elapsed());
        }
    }
}
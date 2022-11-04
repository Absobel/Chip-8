#![allow(non_snake_case)]
mod memory;
mod stack;
mod display;
mod screen;
use memory::Memory;
use stack::Stack;
use screen::{Screen, Pixel};

use std::{
    time::{Duration, Instant},
    thread,
    sync::{Arc, Mutex},
};

fn load_font(memory: &mut Memory) {
    let fontset: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];
    for (i,byte) in fontset.iter().enumerate() {
        memory.write(i as u16 + 0x050, *byte);
    }
}

fn main() {
    let mut screen = Screen::new();

    //display::display().unwrap();
    // Init memory
    let mut memory: Memory = Memory::new();
    memory.load_rom("roms/IBM Logo.ch8").unwrap();
    load_font(&mut memory);

    let I_adr: u16 = 0x0FFE;                           // Index register
    let mut V_adr: [u16; 16] = [0; 16];                // Registers V0 to VF 
    for i in 0..16 {
        V_adr[i] = 0x0FEE + i as u16;
    }
    
    let timer_adr = 0x0FED;                       // Timer register
    let sound_adr = 0x0FEC;
    memory.write(timer_adr, 0x00);
    memory.write(sound_adr, 0x00);                       // Sound register

    let mut stack = Stack::<u16>::new();     // Stack of adresses used to call subroutines or return from them
    let mut pc: u16 = 0x200;             // program counter

    memory.dump();

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


    // game loop
    println!(" adr  | instr  | effect");
    println!("------+--------+--------------------------------");
    loop { 
        let start = Instant::now();

        let guard = mutex_memory.lock().unwrap();
        let instruction = guard.read_word(pc);
        std::mem::drop(guard);

        pc += 2;
        let opcode = (instruction & 0xF000) >> 12; 

        match opcode {
            0 => {
                match instruction {
                    // Clear screen
                    0x00E0 => {
                        println!("0x{:03X} | 0x{:04X} | Screen clearing", pc-2, instruction);
                        screen.clear();
                    }
                    0x00EE => {
                        todo!()
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
                let NNN = instruction & 0x0FFF;
                println!("0x{:03X} | 0x{:04X} | Jumping to adress 0x{:03X}", pc-2, instruction, NNN);
                pc = NNN;
            }
            2 => {
                // 0x2NNN
                todo!()
            }
            3 => {
                // 0x3XNN
                todo!()
            }
            4 => {
                // 0x4XNN
                todo!()
            }
            5 => {
                if instruction & 0x000F == 0 {
                    // 0x5XY0
                    todo!()
                }
                else {
                    println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                    break;
                }
            }
            6 => {
                // 0x6XNN set register VX to 0xNN
                let X = (instruction & 0x0F00) >> 8;
                let NN = instruction & 0x00FF;
                println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to 0x{:02X}", pc-2, instruction, X, NN);

                let mut guard = mutex_memory.lock().unwrap();
                guard.write(V_adr[X as usize], NN as u8);
                std::mem::drop(guard);
            }
            7 => {
                // 0x7XNN add 0xNN to register VX
                let X = ((instruction & 0x0F00) >> 2) as usize;
                let NN = (instruction & 0x00FF) as usize;
                println!("0x{:03X} | 0x{:04X} | Adding 0x{:02X} to register V{:01X}", pc-2, instruction, NN, X);

                let mut guard = mutex_memory.lock().unwrap();
                let value = guard.read(V_adr[X]);
                guard.write(V_adr[X], ((value as usize + NN) % 255) as u8);
                std::mem::drop(guard);
            }
            0x8 => {
                match instruction & 0x000F {
                    1 => {
                        // 0x8XY1
                        todo!();
                    }
                    2 => {
                        // 0x8XY2
                        todo!();
                    }
                    3 => {
                        // 0x8XY3
                        todo!();
                    }
                    4 => {
                        // 0x8XY4
                        todo!();
                    }
                    5 => {
                        // 0x8XY5
                        todo!();
                    }
                    6 => {
                        // 0x8XY6
                        todo!();
                    }
                    7 => {
                        // 0x8XY7
                        todo!();
                    }
                    0xE => {
                        // 0x8XYE
                        todo!();
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            9 => {
                if instruction & 0x000F == 0 {
                    // 0x9XY0
                    todo!()
                }
                else {
                    println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                    break;
                }
            }
            0xA => {
                // 0xANNN set I to 0x0NNN
                let NNN = instruction & 0x0FFF;
                println!("0x{:03X} | 0x{:04X} | Setting I to 0x{:03X}", pc-2, instruction, NNN);

                let mut guard = mutex_memory.lock().unwrap();
                guard.write_word(I_adr, NNN);
                std::mem::drop(guard);
            }
            0xB => {
                // 0xBXNN jump to XNN + VX 
                let X = ((instruction & 0x0F00) >> 8) as u8;
                let XNN = instruction & 0x0FFF;
                println!("0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}", pc-2, instruction, XNN, X);

                let guard = mutex_memory.lock().unwrap();
                let val_VX = guard.read(V_adr[X as usize]);
                std::mem::drop(guard);

                pc = XNN + val_VX as u16;
            }
            0xC => {
                // 0xCXNN
                todo!()
            }
            0xD => {
                    // 0xDXYN display
                    println!("0x{:03X} | 0x{:04X} | Displaying", pc-2, instruction);
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let Y = ((instruction & 0x00F0) >> 4) as usize;
                    let N = (instruction & 0x000F) as u8;
                    
                    let guard = mutex_memory.lock().unwrap();
                    let coord_X = guard.read(V_adr[X]);    
                    let coord_Y = guard.read(V_adr[Y]);
                    std::mem::drop(guard);

            }
            0xE => {
                match instruction & 0x00FF {
                    0x009E => {
                        // 0xEX9E
                        todo!()
                    }
                    0x00A1 => {
                        // 0xEXA1
                        todo!()
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
                        // 0xFX07
                        todo!();
                    }
                    0x000A => {
                        // 0xFX0A
                        todo!();
                    }
                    0x0015 => {
                        // 0xFX0A
                        todo!();
                    }
                    0x0018 => {
                        // 0xFX18
                        todo!();
                    }
                    0x001E => {
                        // 0xFX1E
                        todo!();
                    }
                    0x0029 => {
                        // 0xFX29
                        todo!();
                    }
                    0x0033 => {
                        // 0xFX33
                        todo!();
                    }
                    0x0055 => {
                        // 0xFX55
                        todo!();
                    }
                    0x0065 => {
                        // 0xFX65
                        todo!();
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            _ => {
                println!("0x{:03X} | 0x{:04X} | Non used opcode", pc-2, instruction);
                break;
            }
        }

        if start.elapsed() < Duration::from_millis(2) {
            thread::sleep(Duration::from_millis(2) - start.elapsed());
        }
    }
}
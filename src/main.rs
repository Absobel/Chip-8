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
use rand::Rng;

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

#[derive(PartialEq)]
enum CommandBehavior {
    NEW,
    OLD,
}
const CB_8XY6 : CommandBehavior = CommandBehavior::NEW;
const CB_8XYE : CommandBehavior = CommandBehavior::NEW;
const CB_BNNN : CommandBehavior = CommandBehavior::NEW;
const CB_FX55 : CommandBehavior = CommandBehavior::NEW;
const CB_FX65 : CommandBehavior = CommandBehavior::NEW;

const IPS : u64 = 1; // instructions per second

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
                    // Return from subroutine
                    0x00EE => {
                        println!("0x{:03X} | 0x{:04X} | Returning from subroutine", pc-2, instruction);
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
                let NNN = instruction & 0x0FFF;
                println!("0x{:03X} | 0x{:04X} | Jumping to adress 0x{:03X}", pc-2, instruction, NNN);
                pc = NNN;
            }
            2 => {
                // 0x2NNN call subroutine at 0xNNN
                let NNN = instruction & 0x0FFF;
                println!("0x{:03X} | 0x{:04X} | Calling subroutine at 0x{:03X}", pc-2, instruction, NNN);
                stack.push(pc);
                pc = NNN;
                
            }
            3 => {
                // 0x3XNN skip next instruction if VX == NN
                let X = (instruction & 0x0F00) >> 8;
                let NN = instruction & 0x00FF;

                let guard = mutex_memory.lock().unwrap();
                let VX = guard.read(V_adr[X as usize]);
                std::mem::drop(guard);

                if VX == NN as u8 {
                    println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == 0x{:02X}", pc-2, instruction, X, NN);
                    pc += 2;
                } else {
                    println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != 0x{:02X}", pc-2, instruction, X, NN);
                }
            }
            4 => {
                // 0x4XNN skip next instruction if VX == NN
                let X = (instruction & 0x0F00) >> 8;
                let NN = (instruction & 0x00FF) as u8;

                let guard = mutex_memory.lock().unwrap();
                let VX = guard.read(V_adr[X as usize]);
                std::mem::drop(guard);

                if VX != NN {
                    println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != 0x{:02X}", pc-2, instruction, X, NN);
                    pc += 2;
                } else {
                    println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == 0x{:02X}", pc-2, instruction, X, NN);
                }
            }
            5 => {
                if instruction & 0x000F == 0 {
                    // 0x5XY0 skip next instruction if VX == VY
                    let X = (instruction & 0x0F00) >> 8;
                    let Y = (instruction & 0x00F0) >> 4;

                    let guard = mutex_memory.lock().unwrap();
                    let VX = guard.read(V_adr[X as usize]);
                    let VY = guard.read(V_adr[Y as usize]);
                    std::mem::drop(guard);

                    if VX == VY {
                        println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} == V{:X}", pc-2, instruction, X, Y);
                        pc += 2;
                    } else {
                        println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} != V{:X}", pc-2, instruction, X, Y);
                    }
                    
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
                    0 => {
                        // 0x8XY0 set VX to VY
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X}", pc-2, instruction, X, Y);

                        let mut guard = mutex_memory.lock().unwrap();
                        let value = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[X as usize], value);
                        std::mem::drop(guard);
                    }
                    1 => {
                        // 0x8XY1 set VX to VX | VY
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} | V{:01X}", pc-2, instruction, X, X, Y);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[X as usize], VX | VY);
                        std::mem::drop(guard);
                    }
                    2 => {
                        // 0x8XY1 set VX to VX & VY
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} & V{:01X}", pc-2, instruction, X, X, Y);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[X as usize], VX & VY);
                        std::mem::drop(guard);
                    }
                    3 => {
                        // 0x8XY1 set VX to VX ^ VY
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Setting register V{:01X} to V{:01X} ^ V{:01X}", pc-2, instruction, X, X, Y);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[X as usize], VX ^ VY);
                        std::mem::drop(guard);
                    }
                    4 => {
                        // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Adding V{:01X} to V{:01X} with carry flag to VF", pc-2, instruction, Y, X);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        let result = VX as usize + VY as usize;
                        if result > 255 {
                            guard.write(V_adr[0xF], 1);
                        } else {
                            guard.write(V_adr[0xF], 0);
                        }
                        guard.write(V_adr[X as usize], (result % 255) as u8);
                        std::mem::drop(guard);
                    }
                    5 => {
                        // 0x8XY4 Substract VY to VX. VF is set to 1 if VX > VY, and to 0 if not
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Substracting V{:01X} to V{:01X} with borrow flag to VF", pc-2, instruction, Y, X);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        let result = VX as isize - VY as isize;
                        if result > 0 {
                            guard.write(V_adr[0xF], 0);
                        } else {
                            guard.write(V_adr[0xF], 1);
                        }
                        guard.write(V_adr[X as usize], (result % 255) as u8);
                        std::mem::drop(guard);
                    }
                    6 => {
                        // 0x8XY6
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[0xF], VX & 0x1);

                        if CB_8XY6 == CommandBehavior::OLD {
                            // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                            println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
                            guard.write(V_adr[X as usize], VY >> 1);
                        } else if CB_8XY6 == CommandBehavior::NEW {
                            // VX is shifted right by 1. VF is set to the bit shifted out
                            println!("0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF", pc-2, instruction, X);
                            guard.write(V_adr[X as usize], VX >> 1);
                        } else {}
                        std::mem::drop(guard);
                    }
                    7 => {
                        // 0x8XY4 Substract VX to VY. VF is set to 1 if VY > VX, and to 0 if not
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;
                        println!("0x{:03X} | 0x{:04X} | Substracting V{:01X} to V{:01X} with borrow flag to VF", pc-2, instruction, X, Y);

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        let result = VY as isize - VX as isize;
                        if result > 0 {
                            guard.write(V_adr[0xF], 0);
                        } else {
                            guard.write(V_adr[0xF], 1);
                        }
                        guard.write(V_adr[X as usize], (result % 255) as u8);
                        std::mem::drop(guard);
                    }
                    0xE => {
                        // 0x8XYE
                        let X = (instruction & 0x0F00) >> 8;
                        let Y = (instruction & 0x00F0) >> 4;

                        let mut guard = mutex_memory.lock().unwrap();
                        let VX = guard.read(V_adr[X as usize]);
                        let VY = guard.read(V_adr[Y as usize]);
                        guard.write(V_adr[0xF], VX & 0x1);

                        if CB_8XYE == CommandBehavior::OLD {
                            // VX is set to VY and shifted right by 1. VF is set to the bit shifted out
                            println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to V{:01X} and shifting it right by 1 with bit shifted out to VF", pc-2, instruction, X, Y);
                            guard.write(V_adr[X as usize], VY << 1);
                        } else if CB_8XYE == CommandBehavior::NEW {
                            // VX is shifted right by 1. VF is set to the bit shifted out
                            println!("0x{:03X} | 0x{:04X} | Shifting V{:01X} right by 1 with bit shifted out to VF", pc-2, instruction, X);
                            guard.write(V_adr[X as usize], VX << 1);
                        } else {}
                        std::mem::drop(guard);
                    }
                    _ => {
                        println!("0x{:03X} | 0x{:04X} | Non used instruction", pc-2, instruction);
                        break;
                    }
                }
            }
            9 => {
                if instruction & 0x000F == 0 {
                    // 0x9XY0 skip next instruction if VX != VY
                    let X = (instruction & 0x0F00) >> 8;
                    let Y = (instruction & 0x00F0) >> 4;

                    let guard = mutex_memory.lock().unwrap();
                    let VX = guard.read(V_adr[X as usize]);
                    let VY = guard.read(V_adr[Y as usize]);
                    std::mem::drop(guard);

                    if VX != VY {
                        println!("0x{:03X} | 0x{:04X} | Skipping next instruction because V{:X} != V{:X}", pc-2, instruction, X, Y);
                        pc += 2;
                    } else {
                        println!("0x{:03X} | 0x{:04X} | Not skipping next instruction because V{:X} == V{:X}", pc-2, instruction, X, Y);
                    }
                    
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
                if CB_BNNN == CommandBehavior::OLD {
                    // 0xBNNN jump to 0x0NNN + V0
                    let NNN = instruction & 0x0FFF;
                    println!("0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V0", pc-2, instruction, NNN);

                    let guard = mutex_memory.lock().unwrap();
                    let V0 = guard.read(V_adr[0]);
                    std::mem::drop(guard);

                    pc = NNN + V0 as u16;
                } else if CB_BNNN == CommandBehavior::NEW {
                    // 0xBXNN jump to 0xXNN + VX
                    let X = ((instruction & 0x0F00) >> 8) as u8;
                    let XNN = instruction & 0x0FFF;
                    println!("0x{:03X} | 0x{:04X} | Jumping to 0x{:03X} + V{:01X}", pc-2, instruction, XNN, X);
    
                    let guard = mutex_memory.lock().unwrap();
                    let val_VX = guard.read(V_adr[X as usize]);
                    std::mem::drop(guard);
    
                    pc = XNN + val_VX as u16;
                } else {}
            }
            0xC => {
                // 0xCXNN set VX to random number and binary-AND's it with NN
                let X = (instruction & 0x0F00) >> 8;
                let NN = instruction & 0x00FF;
                println!("0x{:03X} | 0x{:04X} | Setting V{:01X} to random number and binary-AND's it with 0x{:02X}", pc-2, instruction, X, NN);

                let mut rng = rand::thread_rng();
                let random : u8 = rng.gen();

                let mut guard = mutex_memory.lock().unwrap();
                guard.write(V_adr[X as usize], random & NN as u8);
                std::mem::drop(guard);
            }
            0xD => {
                    // 0xDXYN display
                    println!("0x{:03X} | 0x{:04X} | Displaying", pc-2, instruction);
                    let X = ((instruction & 0x0F00) >> 8) as usize;
                    let Y = ((instruction & 0x00F0) >> 4) as usize;
                    let N = (instruction & 0x000F) as u8;
                    
                    let guard = mutex_memory.lock().unwrap();
                    let coord_X = guard.read(V_adr[X]) % 64;    
                    let coord_Y = guard.read(V_adr[Y]) % 32;
                    std::mem::drop(guard);

                    screen.debug_display();
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

        if start.elapsed() < Duration::from_millis(1000 / IPS) {
            thread::sleep(Duration::from_millis(1000 / IPS) - start.elapsed());
        }
    }
}
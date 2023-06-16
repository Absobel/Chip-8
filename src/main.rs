#![allow(non_snake_case)]
mod custom_errors;
mod display;
mod events;
mod instructions;
mod launch_options;
mod memory;
mod opcode_decoder;
mod screen;

use launch_options::*;

use std::{
    sync::{Arc, Mutex},
    thread,
    time::{Duration, Instant},
};

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

    // INIT EVENTS
    let mut dico_events = events::init();

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

        if events::update(&sdl_context, &mut dico_events).is_err() {
            break 'game;
        }

        let guard = mutex_memory.lock().unwrap();
        let instruction = guard.read_word(pc);
        std::mem::drop(guard);

        pc += 2;
        let opcode = (instruction & 0xF000) >> 12;

        opcode_decoder::decode(
            opcode,
            instruction,
            &mut pc,
            &mut stack,
            &mut screen,
            &mut canvas,
            &mutex_memory,
            &V_adr,
            I_adr,
            timer_adr,
            sound_adr,
            &dico_events,
        );

        // To have IPS instructions per second
        if let Some(time_elapsed) = Duration::from_millis(1000 / IPS).checked_sub(start.elapsed()) {
            thread::sleep(time_elapsed);
        }
    }
}

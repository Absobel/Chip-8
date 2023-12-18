#![allow(non_snake_case)]
mod constants;
mod custom_errors;
mod display;
mod events;
mod instruction_executioner;
mod launch_options;
mod memory;
mod screen;

use constants::*;
use launch_options::*;

use std::{
    thread,
    time::{Duration, Instant},
};

fn load_font(memory: &mut memory::Memory) {
    for (i, byte) in FONT_SET.iter().enumerate() {
        memory.write(i as u16 + FONT_ADRESS, *byte);
    }
}

fn main() {
    // INIT DISPLAY
    let mut screen = screen::Screen::new();

    let (sdl_context, mut canvas) = display::init().expect("Could not init display");

    // INIT EVENTS
    let mut keys_state = events::KeysState::new();

    // INIT MEMORY
    let mut memory: memory::Memory = memory::Memory::new();
    memory.load_rom(ROM_PATH).unwrap();
    load_font(&mut memory);

    struct PtrMem(*mut memory::Memory);
    unsafe impl Send for PtrMem {}
    let ptr_mem_delay = PtrMem(&mut memory as *mut memory::Memory);
    let ptr_mem_sound = PtrMem(&mut memory as *mut memory::Memory);

    let mut stack = Vec::<u16>::new(); // Stack of adresses used to call subroutines or return from them
    let mut pc: u16 = 0x200; // program counter

    thread::spawn(move || {
        let _ = &ptr_mem_delay;
        let memory = unsafe { &mut *ptr_mem_delay.0 };
        loop {
            let timer = memory.read_delay_timer();
            if timer > 0 {
                memory.decrement_delay_timer();

                thread::sleep(Duration::from_millis(16));
            }
        }
    });
    thread::spawn(move || {
        let _ = &ptr_mem_sound;
        let memory = unsafe { &mut *ptr_mem_sound.0 };
        loop {
            let timer = memory.read_sound_timer();
            if timer > 0 {
                // TODO: add beep
                memory.decrement_sound_timer();

                thread::sleep(Duration::from_millis(16));
            }
        }
    });

    // GAME LOOP
    if DEBUG {
        println!();
        println!(" adr  | instr  | effect");
        println!("------+--------+--------------------------------");
    }

    loop {
        let start = Instant::now();

        // Only way it could be Err is if the user wants to quit the game
        if events::update(&sdl_context, &mut keys_state).is_err() {
            break;
        }

        instruction_executioner::decode(
            &mut pc,
            &mut stack,
            &mut screen,
            &mut canvas,
            &mut memory,
            &keys_state,
        )
        .expect("Instruction not implemented");

        // To have IPS instructions per second
        let elapsed = start.elapsed();
        if let Some(time_left_frame) =
            Duration::from_secs_f64(1.0 / IPS as f64).checked_sub(elapsed)
        {
            thread::sleep(time_left_frame);
        }
        if DEBUG_PERF {
            let warning = if Duration::from_secs_f64(1.0 / IPS as f64) < elapsed {
                "/!\\/!\\/!\\  "
            } else {
                ""
            };
            println!("{warning}{:?} | {:?}", elapsed, start.elapsed());
        }
    }
}

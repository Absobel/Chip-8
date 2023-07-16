use super::custom_errors::*;

use sdl2::{event::Event, keyboard::Keycode, Sdl};
use std::collections::HashMap;

pub fn init() -> HashMap<u8, bool> {
    let mut dico_events = HashMap::new();

    for i in 0..16u8 {
        dico_events.insert(i, false);
    }

    dico_events
}

pub fn update(sdl_context: &Sdl, dico_events: &mut HashMap<u8, bool>) -> Result<(), QuitGameError> {
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    while let Some(event) = event_pump.poll_iter().next() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => return Err(QuitGameError),

            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => dico_events
                .insert(0x1, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Num1),
                ..
            } => dico_events
                .insert(0x1, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => dico_events
                .insert(0x2, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Num2),
                ..
            } => dico_events
                .insert(0x2, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => dico_events
                .insert(0x3, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Num3),
                ..
            } => dico_events
                .insert(0x3, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => dico_events
                .insert(0xC, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Num4),
                ..
            } => dico_events
                .insert(0xC, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => dico_events
                .insert(0x4, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => dico_events
                .insert(0x4, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => dico_events
                .insert(0x5, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Z),
                ..
            } => dico_events
                .insert(0x5, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => dico_events
                .insert(0x6, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::E),
                ..
            } => dico_events
                .insert(0x6, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => dico_events
                .insert(0xD, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => dico_events
                .insert(0xD, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => dico_events
                .insert(0x7, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::Q),
                ..
            } => dico_events
                .insert(0x7, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => dico_events
                .insert(0x8, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => dico_events
                .insert(0x8, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => dico_events
                .insert(0x9, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => dico_events
                .insert(0x9, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => dico_events
                .insert(0xE, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::F),
                ..
            } => dico_events
                .insert(0xE, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => dico_events
                .insert(0xA, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => dico_events
                .insert(0xA, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => dico_events
                .insert(0x0, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::X),
                ..
            } => dico_events
                .insert(0x0, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => dico_events
                .insert(0xB, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => dico_events
                .insert(0xB, false)
                .expect("Failed to insert key in dico_events"),

            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => dico_events
                .insert(0xF, true)
                .expect("Failed to insert key in dico_events"),
            Event::KeyUp {
                keycode: Some(Keycode::V),
                ..
            } => dico_events
                .insert(0xF, false)
                .expect("Failed to insert key in dico_events"),

            _ => true,
        };
    }
    Ok(())
}

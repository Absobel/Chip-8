use crate::custom_errors::*;

use sdl2::{event::Event, keyboard::Keycode, Sdl};

pub struct KeysState {
    keys: [bool; 16],
}

impl KeysState {
    pub fn new() -> Self {
        KeysState { keys: [false; 16] }
    }

    pub fn read_state(&self, key: u8) -> bool {
        self.keys[key as usize]
    }

    pub fn is_key_pressed(&self) -> Option<u8> {
        for (i, key) in self.keys.iter().enumerate() {
            if *key {
                return Some(i as u8);
            }
        }
        None
    }

    fn update_state(&mut self, key: u8, state: bool) {
        self.keys[key as usize] = state;
    }
}

pub fn update(sdl_context: &Sdl, keys_state: &mut KeysState) -> Result<(), QuitGameError> {
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
            } => keys_state.update_state(0x1, true),
            Event::KeyUp {
                keycode: Some(Keycode::Num1),
                ..
            } => keys_state.update_state(0x1, false),

            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => keys_state.update_state(0x2, true),
            Event::KeyUp {
                keycode: Some(Keycode::Num2),
                ..
            } => keys_state.update_state(0x2, false),

            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => keys_state.update_state(0x3, true),
            Event::KeyUp {
                keycode: Some(Keycode::Num3),
                ..
            } => keys_state.update_state(0x3, false),

            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => keys_state.update_state(0xC, true),
            Event::KeyUp {
                keycode: Some(Keycode::Num4),
                ..
            } => keys_state.update_state(0xC, false),

            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => keys_state.update_state(0x4, true),
            Event::KeyUp {
                keycode: Some(Keycode::A),
                ..
            } => keys_state.update_state(0x4, false),

            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => keys_state.update_state(0x5, true),
            Event::KeyUp {
                keycode: Some(Keycode::Z),
                ..
            } => keys_state.update_state(0x5, false),

            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => keys_state.update_state(0x6, true),
            Event::KeyUp {
                keycode: Some(Keycode::E),
                ..
            } => keys_state.update_state(0x6, false),

            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => keys_state.update_state(0xD, true),
            Event::KeyUp {
                keycode: Some(Keycode::R),
                ..
            } => keys_state.update_state(0xD, false),

            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => keys_state.update_state(0x7, true),
            Event::KeyUp {
                keycode: Some(Keycode::Q),
                ..
            } => keys_state.update_state(0x7, false),

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => keys_state.update_state(0x8, true),
            Event::KeyUp {
                keycode: Some(Keycode::S),
                ..
            } => keys_state.update_state(0x8, false),

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => keys_state.update_state(0x9, true),
            Event::KeyUp {
                keycode: Some(Keycode::D),
                ..
            } => keys_state.update_state(0x9, false),

            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => keys_state.update_state(0xE, true),
            Event::KeyUp {
                keycode: Some(Keycode::F),
                ..
            } => keys_state.update_state(0xE, false),

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => keys_state.update_state(0xA, true),
            Event::KeyUp {
                keycode: Some(Keycode::W),
                ..
            } => keys_state.update_state(0xA, false),

            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => keys_state.update_state(0x0, true),
            Event::KeyUp {
                keycode: Some(Keycode::X),
                ..
            } => keys_state.update_state(0x0, false),

            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => keys_state.update_state(0xB, true),
            Event::KeyUp {
                keycode: Some(Keycode::C),
                ..
            } => keys_state.update_state(0xB, false),

            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => keys_state.update_state(0xF, true),
            Event::KeyUp {
                keycode: Some(Keycode::V),
                ..
            } => keys_state.update_state(0xF, false),

            _ => {}
        };
    }
    Ok(())
}

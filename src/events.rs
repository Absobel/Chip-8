use sdl2::{event::Event, keyboard::Keycode, Sdl};

pub fn events(sdl_context: &Sdl) -> Result<usize, String> {
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    let maybe_event = event_pump.poll_iter().next();
    match maybe_event {
        Some(event) => handle_event(event),
        None => Ok(0xFF),
    }
}

pub fn handle_event(event: Event) -> Result<usize, String> {
    match event {
        Event::Quit { .. }
        | Event::KeyDown {
            keycode: Some(Keycode::Escape),
            ..
        } => Err("Quit".to_string()),
        Event::KeyDown {
            keycode: Some(Keycode::Num1),
            ..
        } => Ok(0x1),
        Event::KeyDown {
            keycode: Some(Keycode::Num2),
            ..
        } => Ok(0x2),
        Event::KeyDown {
            keycode: Some(Keycode::Num3),
            ..
        } => Ok(0x3),
        Event::KeyDown {
            keycode: Some(Keycode::Num4),
            ..
        } => Ok(0xC),
        Event::KeyDown {
            keycode: Some(Keycode::A),
            ..
        } => Ok(0x4),
        Event::KeyDown {
            keycode: Some(Keycode::Z),
            ..
        } => Ok(0x5),
        Event::KeyDown {
            keycode: Some(Keycode::E),
            ..
        } => Ok(0x6),
        Event::KeyDown {
            keycode: Some(Keycode::R),
            ..
        } => Ok(0xD),
        Event::KeyDown {
            keycode: Some(Keycode::Q),
            ..
        } => Ok(0x7),
        Event::KeyDown {
            keycode: Some(Keycode::S),
            ..
        } => Ok(0x8),
        Event::KeyDown {
            keycode: Some(Keycode::D),
            ..
        } => Ok(0x9),
        Event::KeyDown {
            keycode: Some(Keycode::F),
            ..
        } => Ok(0xE),
        Event::KeyDown {
            keycode: Some(Keycode::W),
            ..
        } => Ok(0xA),
        Event::KeyDown {
            keycode: Some(Keycode::X),
            ..
        } => Ok(0x0),
        Event::KeyDown {
            keycode: Some(Keycode::C),
            ..
        } => Ok(0xB),
        Event::KeyDown {
            keycode: Some(Keycode::V),
            ..
        } => Ok(0xF),
        _ => Ok(0xFF),
    }
}

use sdl2::{event::Event, keyboard::Keycode, Sdl};

pub fn events(sdl_context: &Sdl) -> Result<usize, String> {
    let mut event_pump = sdl_context.event_pump()?;

    if let Some(event) = event_pump.poll_iter().next() {
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                return Err("Quit".to_string());
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num1),
                ..
            } => {
                return Ok(0x1);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num2),
                ..
            } => {
                return Ok(0x2);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num3),
                ..
            } => {
                return Ok(0x3);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Num4),
                ..
            } => {
                return Ok(0xC);
            }
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => {
                return Ok(0x4);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Z),
                ..
            } => {
                return Ok(0x5);
            }
            Event::KeyDown {
                keycode: Some(Keycode::E),
                ..
            } => {
                return Ok(0x6);
            }
            Event::KeyDown {
                keycode: Some(Keycode::R),
                ..
            } => {
                return Ok(0xD);
            }
            Event::KeyDown {
                keycode: Some(Keycode::Q),
                ..
            } => {
                return Ok(0x7);
            }
            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => {
                return Ok(0x8);
            }
            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => {
                return Ok(0x9);
            }
            Event::KeyDown {
                keycode: Some(Keycode::F),
                ..
            } => {
                return Ok(0xE);
            }
            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => {
                return Ok(0xA);
            }
            Event::KeyDown {
                keycode: Some(Keycode::X),
                ..
            } => {
                return Ok(0x0);
            }
            Event::KeyDown {
                keycode: Some(Keycode::C),
                ..
            } => {
                return Ok(0xB);
            }
            Event::KeyDown {
                keycode: Some(Keycode::V),
                ..
            } => {
                return Ok(0xF);
            }
            _ => return Ok(0xFF),
        }
    }
    Ok(0xFF)
}

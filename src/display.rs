use super::launch_options::*;
use super::screen;
use super::screen::Screen;

use sdl2::{
    event::Event,
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Canvas, WindowCanvas},
    video::Window,
    Sdl,
};

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 640;
const SIZE_PIXEL: u32 = 20;

pub fn init() -> Result<(Sdl, Canvas<Window>), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem
        .window("CHIP-8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    if TERMINAL {
        window.hide();
    }
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    clear_screen(&mut canvas);
    Ok((sdl_context, canvas))
}

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

pub fn display(
    canvas: &mut WindowCanvas,
    screen: &screen::Screen,
) -> Result<(), String> {
    for (x, y) in Screen::iter_coords() {
        if screen.is_on(x, y) {
            canvas.set_draw_color(Color::RGB(PIXEL_ON.0, PIXEL_ON.1, PIXEL_ON.2));
        } else {
            canvas.set_draw_color(Color::RGB(PIXEL_OFF.0, PIXEL_OFF.1, PIXEL_OFF.2));
        }
        let pixel_emplacement = Rect::new(
            x as i32 * SIZE_PIXEL as i32,
            y as i32 * SIZE_PIXEL as i32,
            SIZE_PIXEL,
            SIZE_PIXEL,
        );
        canvas.fill_rect(pixel_emplacement)?;
    }
    canvas.present();
    Ok(())
}

pub fn clear_screen(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(PIXEL_OFF.0, PIXEL_OFF.1, PIXEL_OFF.2));
    canvas.clear();
    canvas.present();
}

use crate::launch_options::*;
use crate::screen;
use crate::screen::Screen;

use sdl2::{
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
    let window = video_subsystem
        .window("CHIP-8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window
        .into_canvas()
        .build()
        .expect("Could not make a canvas");

    clear_screen(&mut canvas);
    Ok((sdl_context, canvas))
}

pub fn display(canvas: &mut WindowCanvas, screen: &screen::Screen) -> Result<(), String> {
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

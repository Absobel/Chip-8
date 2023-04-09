use super::launch_options::*;
use super::screen;

use image::{Rgb, RgbImage};
use sdl2::{
    event::Event,
    image::LoadTexture,
    keyboard::Keycode,
    pixels::Color,
    rect::{Point, Rect},
    render::{Canvas, Texture, TextureCreator, WindowCanvas},
    video::{Window, WindowContext},
    Sdl,
};

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 640;
const SIZE_PIXEL: u32 = 20;

pub fn init() -> Result<(Sdl, Canvas<Window>), String> {
    // generate pixel textures
    let mut pixel_off = RgbImage::new(SIZE_PIXEL, SIZE_PIXEL);
    let mut pixel_on = RgbImage::new(SIZE_PIXEL, SIZE_PIXEL);
    for x in 0..SIZE_PIXEL {
        for y in 0..SIZE_PIXEL {
            pixel_off.put_pixel(x, y, Rgb(PIXEL_OFF));
            pixel_on.put_pixel(x, y, Rgb(PIXEL_ON));
        }
    }
    pixel_off.save("assets/pixel_off.png").unwrap();
    pixel_on.save("assets/pixel_on.png").unwrap();

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

    let [r, g, b] = PIXEL_OFF;
    canvas.set_draw_color(Color::RGB(r, g, b));
    canvas.clear();
    canvas.present();
    Ok((sdl_context, canvas))
}

pub fn textures_init(
    texture_creator: &TextureCreator<WindowContext>,
) -> Result<(Texture<'_>, Texture<'_>), String> {
    let texture_off = texture_creator.load_texture("assets/pixel_off.png")?;
    let texture_on = texture_creator.load_texture("assets/pixel_on.png")?;
    Ok((texture_off, texture_on))
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

pub fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    position: Point,
    sprite: Rect,
) -> Result<(), String> {
    // top left of the screen is the origin
    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    Ok(())
}

pub fn display(
    canvas: &mut WindowCanvas,
    textures: (&Texture, &Texture),
    screen: &screen::Screen,
    modified: Vec<(u8, u8)>,
) -> Result<(), String> {
    for (x, y) in modified {
        let texture = if screen.is_on(x, y) {
            textures.1
        } else {
            textures.0
        };
        let position = Point::new(x as i32 * SIZE_PIXEL as i32, y as i32 * SIZE_PIXEL as i32);
        let sprite = Rect::new(0, 0, SIZE_PIXEL, SIZE_PIXEL);
        render(canvas, texture, position, sprite)?;
    }
    canvas.present();

    Ok(())
}

pub fn clear_screen(canvas: &mut WindowCanvas, texture_off: &Texture) -> Result<(), String> {
    for x in 0..64 {
        for y in 0..32 {
            let position = Point::new(x * SIZE_PIXEL as i32, y * SIZE_PIXEL as i32);
            let sprite = Rect::new(0, 0, SIZE_PIXEL, SIZE_PIXEL);
            render(canvas, texture_off, position, sprite)?;
        }
    }
    canvas.present();

    Ok(())
}

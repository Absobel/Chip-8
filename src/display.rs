use super::screen;
use super::launch_options::*;

use sdl2::{
        pixels::Color,
        event::Event,
        keyboard::Keycode,
        render::{WindowCanvas,Texture, TextureCreator, Canvas},
        image::LoadTexture,
        rect::{Point,Rect},
        Sdl, video::{WindowContext, Window},
    };

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 640;
const SIZE_PIXEL: u32 = 20;

pub fn init() -> Result<(Sdl, Canvas<Window>), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let mut window = video_subsystem.window("CHIP-8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    if TERMINAL {window.hide();}
    let mut canvas = window.into_canvas().build().expect("Could not make a canvas");

    canvas.set_draw_color(Color::RGB(0,0,0));
    canvas.clear();
    canvas.present();
    Ok((sdl_context, canvas))
}

pub fn textures_init<'a>(texture_creator: &'a TextureCreator<WindowContext>) -> Result<(Texture<'a>,Texture<'a>),String> {
    
    let texture_off = texture_creator.load_texture("assets/pixel_off.png")?;
    let texture_on = texture_creator.load_texture("assets/pixel_on.png")?;
    Ok((texture_off,texture_on))
}

pub fn events(sdl_context: &Sdl) -> Result<usize, String> {
    let mut event_pump = sdl_context.event_pump()?;

    for event in event_pump.poll_iter() {
        match event {
            Event::Quit {..} |
            Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                return Err("Quit".to_string());
            },
            Event::KeyDown { keycode: Some(Keycode::Num1), .. } => {
                return Ok(0x1);
            },
            Event::KeyDown { keycode: Some(Keycode::Num2), .. } => {
                return Ok(0x2);
            },
            Event::KeyDown { keycode: Some(Keycode::Num3), .. } => {
                return Ok(0x3);
            },
            Event::KeyDown { keycode: Some(Keycode::Num4), .. } => {
                return Ok(0xC);
            },
            Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
                return Ok(0x4);
            },
            Event::KeyDown { keycode: Some(Keycode::W), .. } => {
                return Ok(0x5);
            },
            Event::KeyDown { keycode: Some(Keycode::E), .. } => {
                return Ok(0x6);
            },
            Event::KeyDown { keycode: Some(Keycode::R), .. } => {
                return Ok(0xD);
            },
            Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                return Ok(0x7);
            },
            Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                return Ok(0x8);
            },
            Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                return Ok(0x9);
            },
            Event::KeyDown { keycode: Some(Keycode::F), .. } => {
                return Ok(0xE);
            },
            Event::KeyDown { keycode: Some(Keycode::Z), .. } => {
                return Ok(0xA);
            },
            Event::KeyDown { keycode: Some(Keycode::X), .. } => {
                return Ok(0x0);
            },
            Event::KeyDown { keycode: Some(Keycode::C), .. } => {
                return Ok(0xB);
            },
            Event::KeyDown { keycode: Some(Keycode::V), .. } => {
                return Ok(0xF);
            },
            _ => {}
        }
    }
    Ok(0xFF)
}


pub fn render(
    canvas: &mut WindowCanvas, 
    texture : &Texture, 
    position: Point, 
    sprite: Rect
) -> Result<(), String> {

    // top left of the screen is the origin
    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;

    Ok(())
}

pub fn display(
    canvas: &mut WindowCanvas, 
    textures: (&Texture,&Texture),
    screen: &screen::Screen, 
    modified: Vec<(u8,u8)>,
) -> Result<(), String> {
    
    for (x,y) in modified {
        let (texture,position) = if screen.is_on(x,y) {
            (textures.0,Point::new(x as i32 * SIZE_PIXEL as i32, y as i32 * SIZE_PIXEL as i32))
        } else {
            (textures.1,Point::new(x as i32 * SIZE_PIXEL as i32, y as i32 * SIZE_PIXEL as i32))
        };
        let sprite = Rect::new(0,0,SIZE_PIXEL,SIZE_PIXEL);
        render(canvas, &texture, position, sprite)?;
    }
    canvas.present();

    Ok(())
}


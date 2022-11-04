use sdl2::{
        pixels::Color,
        event::Event,
        keyboard::Keycode,
        render::{WindowCanvas,Texture},
        image::{self, LoadTexture, InitFlag},
        rect::{Point,Rect},
    };
use std::time::Duration;

const SCREEN_WIDTH: u32 = 1280;
const SCREEN_HEIGHT: u32 = 640;
const SIZE_PIXEL: u32 = 20;

fn render(
    canvas: &mut WindowCanvas, 
    color: Color, 
    texture : &Texture, 
    position: Point, 
    sprite: Rect
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    // top left of the screen is the origin
    let screen_rect = Rect::from_center(position, sprite.width(), sprite.height());

    canvas.copy(texture, sprite, screen_rect)?;
    canvas.present();

    Ok(())
}

pub fn display() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;

    let window = video_subsystem.window("Chip-8", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .expect("could not initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture_off = texture_creator.load_texture("assets/pixel_off.png")?;
    let texture_on = texture_creator.load_texture("assets/pixel_on.png")?;

    let position = Point::new(0, 0);
    // src position in the spritesheet
    let sprite = Rect::new(0, 0, SIZE_PIXEL, SIZE_PIXEL);
    let position2 = Point::new(SIZE_PIXEL as i32, 0);

    let mut event_pump = sdl_context.event_pump()?;
    let mut i = 0;

    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }
        
        // Update
        i = (i + 1) % 255;

        // Render the canvas
        render(&mut canvas, Color::RGB(0xFF,0xFF,0xFF), &texture_off, position, sprite)?;
        render(&mut canvas, Color::RGB(0xFF,0xFF,0xFF), &texture_on, position2, sprite)?;


        // Time management
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}


use crate::launch_options::*;

#[derive(Copy, Clone)]
pub struct Pixel {
    state: bool,
}

pub struct Screen {
    pub pixels: [[Pixel; 64]; 32],
}

impl Screen {
    // Static methods
    pub fn new() -> Screen {
        Screen {
            pixels: [[Pixel { state: false }; 64]; 32],
        }
    }

    pub fn iter_coords() -> impl Iterator<Item = (u8, u8)> {
        (0..32).flat_map(|x| (0..64).map(move |y| (y, x)))
    }

    // Methods
    pub fn clear(&mut self) {
        for x in 0..32 {
            for y in 0..64 {
                self.pixels[x][y].state = false;
            }
        }
    }

    pub fn is_on(&self, x: u8, y: u8) -> bool {
        self.pixels[y as usize][x as usize].state
    }

    pub fn set(&mut self, x: u8, y: u8, state: bool) {
        let x = x as usize;
        let y = y as usize;
        self.pixels[y][x].state = state;
    }

    // DEBUG

    #[allow(dead_code)]
    pub fn debug_display(&self) {
        if !DEBUG {
            print!("\x1B[2J\x1B[1;1H");
        }
        for x in 0..32 {
            for y in 0..64 {
                if self.pixels[x][y].state {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

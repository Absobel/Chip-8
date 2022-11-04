#[derive(Copy, Clone)]
pub struct Pixel {
    x: u32,
    y: u32,
    state: bool,
}

pub struct Screen {
    pixels: [[Pixel; 64]; 32]
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            pixels: [[Pixel {x: 0, y: 0, state: false}; 64]; 32]
        }
    }

    pub fn clear(&mut self) {
        for x in 0..32 {
            for y in 0..64 {
                self.pixels[x][y].state = false;
            }
        }
    }

    pub fn is_on(&self, x: u32, y: u32) -> bool {
        self.pixels[y as usize][x as usize].state
    }

    pub fn set(&mut self, x: u8, y: u8, state: bool) {
        let x = x as usize;
        let y = y as usize;
        let old_state = self.pixels[y][x].state;
        self.pixels[y][x].state = state;
    }

    pub fn get(&self, x: u8, y: u8) -> bool {
        self.pixels[y as usize][x as usize].state
    }

    #[allow(dead_code)]
    pub fn debug_display(&self) {
        for x in 0..32 {
            for y in 0..64 {
                if self.pixels[x][y].state {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!("");
        }
    }
}
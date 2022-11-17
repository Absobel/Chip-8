#![allow(dead_code)]

//pub const ROM_PATH: &str = "roms/programs/Clock Program [Bill Fisher, 1981].ch8";
pub const ROM_PATH: &str = "roms/programs/IBM Logo.ch8";
pub const IPS : u64 = 400; // instructions per second

pub const DEBUG: bool = false;
pub const TERMINAL: bool = false;

pub const CB_8XY_ : CB = CB::NEW;   // NEW : does not                           |||| OLD : + Set VX to the value of VY
pub const CB_B_NN : CB = CB::NEW;   // NEW : Jump to adress NNN + VX            |||| OLD : Jump to the address NNN plus V0.
pub const CB_FX1E : CB = CB::NEW;   // NEW : + If I overlfow the memory, VF = 1 |||| OLD : does not
pub const CB_FX_5 : CB = CB::NEW;   // NEW : is not                             |||| OLD : I is incremented 

pub const FONT_ADRESS: u16 = 0x50;
pub const FONT_SET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];


#[derive(PartialEq)]
pub enum CB {        // Command behavior
    NEW,
    OLD,
}





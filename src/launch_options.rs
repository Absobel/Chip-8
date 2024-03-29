#![allow(clippy::upper_case_acronyms)]

//pub const ROM_PATH: &str = "roms/test_opcode.ch8";
//pub const ROM_PATH: &str = "roms/autre/PONG2";          // Problème d'affichage des points
pub const ROM_PATH: &str = "roms/autre/INVADERS";
//pub const ROM_PATH: &str = "roms/autre/TETRIS";
//pub const ROM_PATH: &str = "roms/programs/Keypad Test [Hap, 2006].ch8";  // Affiche mal somehow ??

pub const IPS: u64 = 700; // instructions per second

pub const DEBUG: bool = false;
pub const DEBUG_VERBOSE: bool = false;
pub const DEBUG_PERF: bool = false;

pub const PIXEL_ON: (u8, u8, u8) = (0x21, 0x31, 0x34);
pub const PIXEL_OFF: (u8, u8, u8) = (0xFF, 0xFF, 0xFF);

pub const CB_8XY_: CB = CB::NEW; // NEW : does not                           |||| OLD : + Set VX to the value of VY
pub const CB_B_NN: CB = CB::NEW; // NEW : Jump to adress NNN + VX            |||| OLD : Jump to the address NNN plus V0.
pub const CB_FX1E: CB = CB::NEW; // NEW : + If I overlfow the memory, VF = 1 |||| OLD : does not
pub const CB_FX_5: CB = CB::NEW; // NEW : is not                             |||| OLD : I is incremented

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
pub enum CB {
    // Command behavior
    NEW,
    OLD,
}

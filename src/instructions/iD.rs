use super::super::constants::*;
use super::super::display;
use super::super::launch_options::*;
use super::super::memory::Memory;
use super::super::screen::Screen;

use sdl2::render::WindowCanvas;

// 0xDXYN display sprite at (VX, VY) with width 8 and height N
pub fn r(
    memory: &mut Memory,
    pc: u16,
    instruction: u16,
    screen: &mut Screen,
    canvas: &mut WindowCanvas,
) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;
    let N = instruction & 0x000F;

    let VX = memory.read(V_ADR[X]);
    let VY = memory.read(V_ADR[Y]);

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Displaying sprite at (V{:01X}, V{:01X}) = ({VX}, {VY}) with width 8 and height {:01X}", pc-2, instruction, X, Y, N);
    }

    let mut cX = VX % 64; // coord X
    let mut cY = VY % 32; // coord Y
    let ccX = cX;
    memory.write(V_ADR[0xF], 0);

    'rows: for i in 0..N {
        let row = memory.read(memory.read_word(I_ADR) + i);
        'columns: for j in 0..8 {
            let pixel = (row >> (7 - j)) & 0x1;
            if pixel == 1 {
                if screen.is_on(cX, cY) {
                    memory.write(V_ADR[0xF], 1);
                    screen.set(cX, cY, false);
                } else {
                    screen.set(cX, cY, true);
                }
            }

            let new_cX = cX as usize + 1;
            if new_cX == 64 {
                break 'columns;
            } else {
                cX = new_cX as u8;
            }
        }
        cX = ccX;
        let new_cY = cY as usize + 1;
        if new_cY == 32 {
            break 'rows;
        } else {
            cY = new_cY as u8;
        }
    }

    display::display(canvas, screen).expect("Error while displaying");
}

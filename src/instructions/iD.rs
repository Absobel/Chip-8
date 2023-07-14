use crate::constants::*;
use crate::display;
use crate::launch_options::*;
use crate::memory::Memory;
use crate::screen::Screen;

use sdl2::render::WindowCanvas;
use std::sync::{Arc, Mutex};

// 0xDXYN display sprite at (VX, VY) with width 8 and height N
pub fn r(
    mutex_memory: &Arc<Mutex<Memory>>,
    pc: u16,
    instruction: u16,
    screen: &mut Screen,
    canvas: &mut WindowCanvas,
) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;
    let N = instruction & 0x000F;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_ADR[X]);
    let VY = guard.read(V_ADR[Y]);

    if DEBUG {
        println!("0x{:03X} | 0x{:04X} | Displaying sprite at (V{:01X}, V{:01X}) = ({VX}, {VY}) with width 8 and height {:01X}", pc-2, instruction, X, Y, N);
    }

    let mut cX = VX % 64; // coord X
    let mut cY = VY % 32; // coord Y
    let ccX = cX;
    guard.write(V_ADR[0xF], 0);

    'rows: for i in 0..N {
        let row = guard.read(guard.read_word(I_ADR) + i);
        'columns: for j in 0..8 {
            let pixel = (row >> (7 - j)) & 0x1;
            if pixel == 1 {
                if screen.is_on(cX, cY) {
                    guard.write(V_ADR[0xF], 1);
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
    std::mem::drop(guard);

    if TERMINAL {
        screen.debug_display();
    } else {
        display::display(canvas, screen).expect("Error while displaying");
    }
}

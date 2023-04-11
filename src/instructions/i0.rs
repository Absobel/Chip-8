use super::super::custom_errors::*;
use super::super::display;
use super::super::launch_options::*;
use super::super::screen;

use sdl2::render::Canvas;
use sdl2::video::Window;

pub fn i0(
    instruction: u16,
    pc: &mut u16,
    stack: &mut Vec<u16>,
    screen: &mut screen::Screen,
    canvas: &mut Canvas<Window>,
) -> Result<(), NonUsedInstructionError> {
    match instruction {
        // Clear screen
        0x00E0 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Screen clearing",
                    *pc - 2,
                    instruction
                );
            }
            screen.clear();
            display::clear_screen(canvas)
        }
        // Return from subroutine
        0x00EE => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Returning from subroutine",
                    *pc - 2,
                    instruction
                );
            }
            *pc = stack.pop().unwrap();
        }
        _ => {
            return Err(NonUsedInstructionError {
                pc: *pc,
                instruction,
            });
        }
    }
    Ok(())
}

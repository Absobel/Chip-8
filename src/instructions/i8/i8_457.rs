use super::super::super::constants::*;
use super::super::super::launch_options::*;
use super::super::super::memory::Memory;

use std::sync::{Arc, Mutex};

pub fn r(instruction: u16, pc: u16, mutex_memory: &Arc<Mutex<Memory>>) {
    let X = ((instruction & 0x0F00) >> 8) as usize;
    let Y = ((instruction & 0x00F0) >> 4) as usize;

    let mut guard = mutex_memory.lock().unwrap();
    let VX = guard.read(V_ADR[X]);
    let VY = guard.read(V_ADR[Y]);
    match instruction & 0x000F {
        // 0x8XY4 Add VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
        4 => {
            if DEBUG {
                println!(
                    "0x{:03X} | 0x{:04X} | Adding V{:01X} to V{:01X} with carry flag to VF",
                    pc - 2,
                    instruction,
                    Y,
                    X
                );
            }
            let (result, carry) = VX.overflowing_add(VY);
            guard.write(V_ADR[X], result);
            guard.write(V_ADR[0xF], carry as u8);
        }
        // 0x8XY5 Set VX to VX - VY, set VF to 0 when there's a borrow, and 1 when there isn't
        // 0x8XY7           VY - VX
        5 | 7 => {
            let (VX, VY) = if instruction & 0x000F == 5 {
                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, Y, X);
                }
                (VX, VY)
            } else {
                if DEBUG {
                    println!("0x{:03X} | 0x{:04X} | Subtracting V{:01X} from V{:01X} with borrow flag to VF", pc-2, instruction, X, Y);
                }
                (VY, VX)
            };
            let result = VX as isize - VY as isize;
            if result < 0 {
                guard.write(V_ADR[0xF], 0);
            } else {
                guard.write(V_ADR[0xF], 1);
            }
            guard.write(V_ADR[X], (result % 255) as u8);
        }
        _ => unreachable!(),
    }
    std::mem::drop(guard);
}

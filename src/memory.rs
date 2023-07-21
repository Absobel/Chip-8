use std::fs::File;
use std::io::prelude::*;
use std::sync::atomic::{AtomicU8, Ordering};

pub struct Memory {
    data: [u8; 4096],
    registers: [u8; 16],
    adress_register: u16,
    delay_timer: AtomicU8,
    sound_timer: AtomicU8,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            data: [0; 4096],
            registers: [0; 16],
            adress_register: 0,
            delay_timer: AtomicU8::new(0),
            sound_timer: AtomicU8::new(0),
        }
    }

    // SETUP

    pub fn load_rom(&mut self, rom: &str) -> Result<(), String> {
        let mut file = File::open(rom).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        // write the memory from 0x200 (because historically the first 512 bytes were reserved for the interpreter)
        for (i, byte) in buffer.iter().enumerate() {
            self.data[0x200 + i] = *byte;
        }
        Ok(())
    }

    // DATA

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high = self.data[address as usize] as u16;
        let low = self.data[address as usize + 1] as u16;
        (high << 8) | low
    }

    // TIMERS

    pub fn read_delay_timer(&self) -> u8 {
        self.delay_timer.load(Ordering::Relaxed)
    }

    pub fn write_delay_timer(&mut self, value: u8) {
        self.delay_timer.store(value, Ordering::Relaxed);
    }

    pub fn decrement_delay_timer(&mut self) {
        self.delay_timer.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn read_sound_timer(&self) -> u8 {
        self.sound_timer.load(Ordering::Relaxed)
    }

    pub fn write_sound_timer(&mut self, value: u8) {
        self.sound_timer.store(value, Ordering::Relaxed);
    }

    pub fn decrement_sound_timer(&mut self) {
        self.sound_timer.fetch_sub(1, Ordering::Relaxed);
    }

    // REGISTERS

    pub fn write_adress(&mut self, value: u16) {
        self.adress_register = value;
    }

    pub fn read_adress(&self) -> u16 {
        self.adress_register
    }

    pub fn read_register(&self, index: usize) -> u8 {
        self.registers[index]
    }

    pub fn write_register(&mut self, index: usize, value: u8) {
        self.registers[index] = value;
    }

    // DEBUG

    #[allow(dead_code)]
    pub fn dump(&self) {
        for (i, byte) in self.data.iter().enumerate() {
            // print 16 bytes per line
            if i % 16 == 0 {
                println!();
                print!("{:03x} : ", i);
            }
            print!("{:02X} ", byte);
        }
        println!();
    }
}

use std::fs::File;
use std::io::prelude::*;

pub struct Memory {
    data: [u8; 4096],
}

impl Memory {
    pub fn new() -> Memory {
        Memory { data: [0; 4096] }
    }

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

    pub fn read(&self, address: u16) -> u8 {
        self.data[address as usize]
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let high = self.data[address as usize] as u16;
        let low = self.data[address as usize + 1] as u16;
        (high << 8) | low
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let high = ((value & 0xFF00) >> 8) as u8;
        let low = (value & 0x00FF) as u8;
        self.data[address as usize] = high;
        self.data[address as usize + 1] = low;
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.data[address as usize] = value;
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
